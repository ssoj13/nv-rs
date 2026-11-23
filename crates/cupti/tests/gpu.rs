//! GPU tests (T2/T3) for `cupti`: activity record capture and callback subscriber.

use cuda_core::{Context, DeviceBuffer};
use cupti::{ActivityKind, ActivityProfiler, ActivityRecord, CallbackDomain, Subscriber};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

/// CUPTI activity state and the single allowed subscriber are process-global, while the test
/// harness runs tests on parallel threads; every test holds this lock.
static SERIAL: Mutex<()> = Mutex::new(());

fn serial() -> MutexGuard<'static, ()> {
    SERIAL.lock().unwrap_or_else(|e| e.into_inner())
}

#[test]
#[ignore = "GPU"]
fn activity_captures_memcpy_records() {
    let _serial = serial();
    let ver = cupti::version().expect("cuptiGetVersion");
    assert!(ver >= 12);

    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let mut profiler = ActivityProfiler::new(&cuda).expect("ActivityProfiler");
    profiler
        .enable(ActivityKind::Memcpy)
        .expect("enable memcpy");

    let host_data = vec![42u8; 4096];
    let dev = DeviceBuffer::from_host(&stream, &host_data).expect("HtoD");
    let back = dev.to_host_vec(&stream).expect("DtoH");
    assert_eq!(back, host_data);
    stream.synchronize().expect("sync");

    profiler.flush(true).expect("flush");
    let (records, _dropped) = profiler.drain_records().expect("drain");
    assert!(
        records
            .iter()
            .any(|r| matches!(r, ActivityRecord::Memcpy { bytes, .. } if *bytes >= 4096)),
        "expected at least one Memcpy activity record >= 4096 bytes, got {records:?}"
    );
}

#[test]
#[ignore = "GPU"]
fn subscriber_captures_driver_api_callbacks() {
    let _serial = serial();
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let hits = Arc::new(AtomicUsize::new(0));
    let hits_cb = hits.clone();
    let sub = Subscriber::new(
        &cuda,
        Box::new(move |dom, _cbid| {
            if dom == CallbackDomain::DriverApi {
                hits_cb.fetch_add(1, Ordering::Relaxed);
            }
        }),
    )
    .expect("Subscriber");
    sub.enable_domain(CallbackDomain::DriverApi, true)
        .expect("enable DriverApi");

    let _dev = DeviceBuffer::from_host(&stream, &[1u32, 2, 3, 4]).expect("alloc+upload");
    stream.synchronize().expect("sync");
    sub.check_panic().expect("no panic");
    assert!(
        hits.load(Ordering::Relaxed) > 0,
        "expected DriverApi callbacks"
    );
}

#[test]
#[ignore = "GPU"]
fn dropping_one_profiler_keeps_shared_kind_enabled() {
    // CUPTI-1: `cuptiActivityEnable` is process-global; dropping profiler A must not disable a
    // kind that profiler B still collects.
    let _serial = serial();
    const BYTES: usize = 12_345;
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let mut a = ActivityProfiler::new(&cuda).expect("profiler A");
    let mut b = ActivityProfiler::new(&cuda).expect("profiler B");
    a.enable(ActivityKind::Memcpy).expect("A enables memcpy");
    b.enable(ActivityKind::Memcpy).expect("B enables memcpy");
    drop(a);
    b.drain_records().expect("discard records flushed by A's drop");

    let dev = DeviceBuffer::from_host(&stream, &vec![7u8; BYTES]).expect("HtoD");
    stream.synchronize().expect("sync");
    drop(dev);
    b.flush(true).expect("flush");
    let (records, _dropped) = b.drain_records().expect("drain");
    assert!(
        records
            .iter()
            .any(|r| matches!(r, ActivityRecord::Memcpy { bytes, .. } if *bytes == BYTES as u64)),
        "profiler B lost Memcpy collection when profiler A dropped, got {records:?}"
    );
}

#[test]
#[ignore = "GPU"]
fn new_profiler_starts_without_records_of_a_dropped_one() {
    // CUPTI-2: the forced flush in `Drop` delivers final records into the shared queue; when no
    // profiler is left they must not leak into the next session.
    let _serial = serial();
    const BYTES: usize = 23_456;
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    {
        let mut a = ActivityProfiler::new(&cuda).expect("profiler A");
        a.enable(ActivityKind::Memcpy).expect("A enables memcpy");
        let dev = DeviceBuffer::from_host(&stream, &vec![9u8; BYTES]).expect("HtoD");
        stream.synchronize().expect("sync");
        drop(dev);
        // `a` drops here without draining.
    }
    let b = ActivityProfiler::new(&cuda).expect("profiler B");
    let (records, dropped) = b.drain_records().expect("drain");
    assert!(
        !records
            .iter()
            .any(|r| matches!(r, ActivityRecord::Memcpy { bytes, .. } if *bytes == BYTES as u64)),
        "profiler B received a record of the dropped profiler A: {records:?}"
    );
    assert_eq!(dropped, 0, "stale dropped-record count from profiler A");
}

static CB_ENTERED: AtomicBool = AtomicBool::new(false);
static CB_EXITED: AtomicBool = AtomicBool::new(false);

#[test]
#[ignore = "GPU"]
fn subscriber_drop_waits_for_in_flight_callback() {
    // CUPTI-4: CUPTI does not document that `cuptiUnsubscribe` waits for a callback already
    // running on another thread. `Subscriber::drop` must not return (and free the closure) while
    // one is still executing. The closure touches only statics, so a failure here is reported
    // without reading freed memory.
    let _serial = serial();
    CB_ENTERED.store(false, Ordering::SeqCst);
    CB_EXITED.store(false, Ordering::SeqCst);
    let cuda = Context::new(0).expect("CUDA device 0");
    let sub = Subscriber::new(
        &cuda,
        Box::new(|dom, _cbid| {
            if dom == CallbackDomain::DriverApi && !CB_ENTERED.swap(true, Ordering::SeqCst) {
                std::thread::sleep(Duration::from_millis(300));
                CB_EXITED.store(true, Ordering::SeqCst);
            }
        }),
    )
    .expect("Subscriber");
    sub.enable_domain(CallbackDomain::DriverApi, true)
        .expect("enable DriverApi");

    let worker_cuda = cuda.clone();
    let worker = std::thread::spawn(move || {
        // Driver API calls on this thread (`cuCtxGetCurrent`, `cuStreamSynchronize`) enter the
        // callback; the first one sleeps inside it.
        worker_cuda.bind_to_thread().expect("bind on worker");
        worker_cuda
            .default_stream()
            .synchronize()
            .expect("sync on worker");
    });
    while !CB_ENTERED.load(Ordering::SeqCst) {
        std::thread::yield_now();
    }
    drop(sub);
    let exited_before_drop_returned = CB_EXITED.load(Ordering::SeqCst);
    worker.join().expect("worker thread");
    assert!(
        exited_before_drop_returned,
        "Subscriber::drop returned while a callback was still running on another thread"
    );
}
