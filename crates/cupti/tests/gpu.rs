//! GPU tests (T2/T3) for `cupti`: activity record capture and callback subscriber.

use cuda_core::{Context, DeviceBuffer};
use cupti::{ActivityKind, ActivityProfiler, ActivityRecord, CallbackDomain, Subscriber};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[test]
#[ignore = "GPU"]
fn activity_captures_memcpy_records() {
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
