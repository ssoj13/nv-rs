//! Raw Linux NCCL host C API, generated from the installed SDK by `nv-bindgen`.
//!
//! Includes the nccl/pnccl host calls and the host-side device-communicator requirements,
//! layouts and queries. Functions without a documented failure sentinel retain their C name
//! as an unsafe Rust function returning `Result<_, nv_loader::CallError>`.
//! The shared loader keeps NCCL alive; CUDA stream and memory-location types are reused.
//! Used by nv-rs collective communication consumers; device code is a separate NCCL API.
#![cfg(target_os = "linux")]

use nv_loader::{DllSpec, Group};

nv_loader::exports_sys! {
    lib: "nccl",
    api: NcclApi,
    loader: nccl_api,
    dll: DllSpec {
        lib: "nccl",
        group: Group::Sdk {
            root_var: "NCCL_ROOT",
            hint: nv_loader::hint(env!("NV_RS_SDK_BIN")),
        },
        windows: &[],
        linux: &["libnccl.so.2", "libnccl.so"],
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use cuda_core::{Context, DeviceBuffer};
    use serde_json::Value;

    #[test]
    fn inventory_covers_core_api() {
        let inventory: Value = serde_json::from_str(INVENTORY).expect("valid generated inventory");
        let symbols = inventory
            .get("symbols")
            .and_then(Value::as_array)
            .expect("symbols array");
        for name in [
            "ncclGetVersion",
            "ncclCommInitAll",
            "ncclAllReduce",
            "ncclGroupStart",
        ] {
            assert!(
                symbols.iter().any(|symbol| symbol.as_str() == Some(name)),
                "{name} missing"
            );
        }
    }

    #[cfg(nccl_has_device)]
    #[test]
    fn inventory_covers_public_host_api() {
        let inventory: Value = serde_json::from_str(INVENTORY).expect("valid generated inventory");
        let symbols = inventory
            .get("symbols")
            .and_then(Value::as_array)
            .expect("symbols array");
        for name in [
            "ncclGetVersion",
            "pncclGetVersion",
            "ncclGetUniqueId",
            "pncclGetUniqueId",
            "ncclGetErrorString",
            "pncclGetErrorString",
            "ncclGetLastError",
            "pncclGetLastError",
            "ncclCommInitAll",
            "pncclCommInitAll",
            "ncclCommInitRank",
            "pncclCommInitRank",
            "ncclCommDestroy",
            "pncclCommDestroy",
            "ncclAllReduce",
            "pncclAllReduce",
            "ncclAllReduceConfig",
            "pncclAllReduceConfig",
            "ncclGroupStart",
            "pncclGroupStart",
            "ncclGroupEnd",
            "pncclGroupEnd",
            "ncclCommWindowRegister",
            "pncclCommWindowRegister",
            "ncclPutSignal",
            "pncclPutSignal",
            "ncclWaitSignal",
            "pncclWaitSignal",
            "ncclParamBind",
            "pncclParamBind",
            "ncclParamDumpAll",
            "pncclParamDumpAll",
            "ncclResetDebugInit",
            "pncclResetDebugInit",
            "ncclCommQueryProperties",
            "ncclDevCommCreate",
            "ncclDevCommDestroy",
            "ncclGetLsaMultimemDevicePointer",
            "ncclGetMultimemDevicePointer",
            "ncclGetLsaDevicePointer",
            "ncclGetPeerDevicePointer",
            "ncclGetMultimemDeviceLeInfo",
            "ncclGetCftDeviceLeInfo",
            "ncclGetPeerDeviceLeInfo",
            "ncclLsaBarrierCreateRequirement",
            "ncclGinBarrierCreateRequirement",
            "ncclCftBarrierCreateRequirement",
            "ncclLLA2ACreateRequirement",
            "ncclLLA2ACalcSlots",
            "ncclTeamCft",
            "ncclTeamCftMultimem",
            "ncclTeamLsa",
            "ncclTeamRail",
            "ncclTeamRankToLsa",
            "ncclTeamRankToWorld",
            "ncclTeamWorld",
        ] {
            assert!(
                symbols.iter().any(|symbol| symbol.as_str() == Some(name)),
                "{name} missing"
            );
        }
        let unshimmed = inventory
            .get("unshimmed")
            .and_then(Value::as_array)
            .expect("unshimmed array");
        let names: Vec<&str> = unshimmed
            .iter()
            .map(|value| value.as_str().expect("entry point name"))
            .collect();
        assert_eq!(
            names,
            [
                "ncclLLA2ACalcSlots",
                "ncclParamDumpAll",
                "ncclResetDebugInit",
                "ncclTeamCft",
                "ncclTeamCftMultimem",
                "ncclTeamLsa",
                "ncclTeamRail",
                "ncclTeamRankToLsa",
                "ncclTeamRankToWorld",
                "ncclTeamWorld",
                "pncclParamDumpAll",
                "pncclResetDebugInit",
            ]
        );
    }

    #[cfg(nccl_has_device)]
    #[test]
    fn cuda_types_are_shared() {
        let _: cudaStream_t = std::ptr::null_mut::<cuda_bindings::CUstream_st>();
        let location: cuda_bindings::CUmemLocationType =
            cuda_bindings::CUmemLocationType_enum_CU_MEM_LOCATION_TYPE_DEVICE;
        let segment = ncclSegmentWindow {
            memType: location,
            ..Default::default()
        };
        assert_eq!(
            segment.memType,
            cuda_bindings::CUmemLocationType_enum_CU_MEM_LOCATION_TYPE_DEVICE
        );
    }

    #[cfg(nccl_has_device)]
    #[test]
    fn host_queries_are_checked() {
        let _: unsafe fn(i32, i32) -> Result<i32, nv_loader::CallError> = ncclLLA2ACalcSlots;
        let _: unsafe fn(
            ncclComm_t,
            ncclCftTeamMode_t,
        ) -> Result<ncclTeam_t, nv_loader::CallError> = ncclTeamCft;
        let _: unsafe fn(ncclComm_t) -> Result<ncclTeam_t, nv_loader::CallError> =
            ncclTeamCftMultimem;
        let _: unsafe fn(ncclComm_t) -> Result<ncclTeam_t, nv_loader::CallError> = ncclTeamLsa;
        let _: unsafe fn(ncclComm_t) -> Result<ncclTeam_t, nv_loader::CallError> = ncclTeamRail;
        let _: unsafe fn(ncclComm_t, ncclTeam_t, i32) -> Result<i32, nv_loader::CallError> =
            ncclTeamRankToLsa;
        let _: unsafe fn(ncclComm_t, ncclTeam_t, i32) -> Result<i32, nv_loader::CallError> =
            ncclTeamRankToWorld;
        let _: unsafe fn(ncclComm_t) -> Result<ncclTeam_t, nv_loader::CallError> = ncclTeamWorld;
        let _: unsafe fn() -> Result<(), nv_loader::CallError> = ncclParamDumpAll;
        let _: unsafe fn() -> Result<(), nv_loader::CallError> = pncclParamDumpAll;
        let _: unsafe fn() -> Result<(), nv_loader::CallError> = ncclResetDebugInit;
        let _: unsafe fn() -> Result<(), nv_loader::CallError> = pncclResetDebugInit;
    }

    #[test]
    #[ignore = "runtime: needs the Linux NCCL library; run with --ignored"]
    fn version_is_reported() {
        load().expect("nccl loads");
        let mut version = 0;
        let mut profiling_version = 0;
        // SAFETY: both version pointers refer to writable, aligned integers for each call.
        let version_status = unsafe { ncclGetVersion(&mut version) };
        assert_eq!(version_status, ncclResult_t_ncclSuccess);
        // SAFETY: profiling_version is writable and valid for the call.
        let profiling_status = unsafe { pncclGetVersion(&mut profiling_version) };
        assert_eq!(profiling_status, ncclResult_t_ncclSuccess);
        assert_eq!(
            version,
            i32::try_from(NCCL_VERSION_CODE).expect("header version fits C int")
        );
        assert_eq!(profiling_version, version);
        // SAFETY: a valid status is passed; NCCL returns a library-owned NUL-terminated string.
        let text = unsafe { ncclGetErrorString(ncclResult_t_ncclSuccess) };
        assert!(!text.is_null(), "success status string was not reported");
        // SAFETY: the documented string is NUL-terminated and the loader retains its library.
        let text_cstr = unsafe { std::ffi::CStr::from_ptr(text) };
        assert!(!text_cstr.to_bytes().is_empty());
    }

    #[derive(Debug)]
    struct Cleanup<E> {
        completion: Option<Result<(), E>>,
        destroy: Option<ncclResult_t>,
        abort: Option<ncclResult_t>,
    }

    impl<E> Cleanup<E> {
        fn succeeded(&self) -> bool {
            self.completion.as_ref().is_some_and(Result::is_ok)
                && self.destroy == Some(ncclResult_t_ncclSuccess)
                && self.abort.is_none()
        }
    }

    fn abort_partial_init(
        initialized: ncclResult_t,
        has_comm: bool,
        abort: impl FnOnce() -> ncclResult_t,
    ) -> Option<ncclResult_t> {
        (initialized != ncclResult_t_ncclSuccess && has_comm).then(abort)
    }

    fn finish_comm<E>(
        work_ok: bool,
        synchronize: impl FnOnce() -> Result<(), E>,
        destroy: impl FnOnce() -> ncclResult_t,
        abort: impl FnOnce() -> ncclResult_t,
    ) -> Cleanup<E> {
        if !work_ok {
            let status = abort();
            let completion = if status == ncclResult_t_ncclSuccess {
                Some(synchronize())
            } else {
                None
            };
            return Cleanup {
                completion,
                destroy: None,
                abort: Some(status),
            };
        }
        let completion = synchronize();
        let (destroy, abort) = if completion.is_ok() {
            let status = destroy();
            let aborted = (status != ncclResult_t_ncclSuccess).then(abort);
            (Some(status), aborted)
        } else {
            (None, Some(abort()))
        };
        Cleanup {
            completion: Some(completion),
            destroy,
            abort,
        }
    }

    #[test]
    fn cleanup_orders_abort_wait_and_destroy_without_hiding_failures() {
        let success = ncclResult_t_ncclSuccess;
        let failure = ncclResult_t_ncclSystemError;
        for (work_ok, sync_ok, destroy_status, abort_status, order) in [
            (false, true, success, success, &["abort", "sync"][..]),
            (false, true, success, failure, &["abort"][..]),
            (true, true, success, success, &["sync", "destroy"][..]),
            (true, false, success, success, &["sync", "abort"][..]),
            (
                true,
                true,
                failure,
                success,
                &["sync", "destroy", "abort"][..],
            ),
            (true, false, success, failure, &["sync", "abort"][..]),
        ] {
            let calls = std::cell::RefCell::new(Vec::new());
            let cleanup = finish_comm(
                work_ok,
                || {
                    calls.borrow_mut().push("sync");
                    if sync_ok { Ok(()) } else { Err("sync failed") }
                },
                || {
                    calls.borrow_mut().push("destroy");
                    destroy_status
                },
                || {
                    calls.borrow_mut().push("abort");
                    abort_status
                },
            );
            assert_eq!(calls.borrow().as_slice(), order);
            assert_eq!(
                cleanup.completion,
                order.contains(&"sync").then_some(if sync_ok {
                    Ok(())
                } else {
                    Err("sync failed")
                })
            );
            assert_eq!(
                cleanup.destroy,
                order.contains(&"destroy").then_some(destroy_status)
            );
            assert_eq!(
                cleanup.abort,
                order.contains(&"abort").then_some(abort_status)
            );
            assert_eq!(
                cleanup.succeeded(),
                work_ok && sync_ok && destroy_status == success
            );
        }
    }

    #[test]
    fn failed_initialization_aborts_only_a_non_null_partial_communicator() {
        for (initialized, has_comm) in [
            (ncclResult_t_ncclSystemError, true),
            (ncclResult_t_ncclSystemError, false),
            (ncclResult_t_ncclSuccess, true),
            (ncclResult_t_ncclSuccess, false),
        ] {
            let called = std::cell::Cell::new(false);
            let status = abort_partial_init(initialized, has_comm, || {
                called.set(true);
                ncclResult_t_ncclSystemError
            });
            let required = initialized != ncclResult_t_ncclSuccess && has_comm;
            assert_eq!(called.get(), required);
            assert_eq!(status, required.then_some(ncclResult_t_ncclSystemError));
        }
    }

    #[test]
    #[ignore = "GPU: needs Linux NCCL and one CUDA device; run with --ignored --test-threads=1"]
    fn single_gpu_all_reduce_round_trips() {
        load().expect("nccl loads");
        let context = Context::new(0).expect("CUDA device 0 primary context");
        let stream = context.new_stream().expect("CUDA stream");
        let values = [2_i32, -5, 7, 11];
        let input = DeviceBuffer::from_host(&stream, &values).expect("upload input");
        let output = DeviceBuffer::from_host(&stream, &[0_i32; 4]).expect("allocate output");
        context.bind_to_thread().expect("bind NCCL device context");
        let mut comm: ncclComm_t = std::ptr::null_mut();
        let device = 0_i32;
        // SAFETY: comm stores one communicator; device names the retained current CUDA device.
        let initialized = unsafe { ncclCommInitAll(&mut comm, 1, &device) };
        let partial_abort = abort_partial_init(initialized, !comm.is_null(), || {
            // SAFETY: failed initialization produced a non-null partial communicator owned here.
            unsafe { ncclCommAbort(comm) }
        });
        assert_eq!(
            initialized, ncclResult_t_ncclSuccess,
            "initialize one rank; partial abort: {partial_abort:?}"
        );
        assert!(
            !comm.is_null(),
            "successful initialization returned no communicator"
        );
        let work = (|| -> Result<(), String> {
            for count in [1_usize, values.len()] {
                // SAFETY: both allocations contain count i32 values, belong to the current
                // primary context, and remain alive through synchronization and communicator
                // cleanup. The real stream and one-rank communicator match that context.
                let status = unsafe {
                    ncclAllReduce(
                        input.cu_deviceptr() as *const std::ffi::c_void,
                        output.cu_deviceptr() as *mut std::ffi::c_void,
                        count,
                        ncclDataType_t_ncclInt32,
                        ncclRedOp_t_ncclSum,
                        comm,
                        stream.cu_stream(),
                    )
                };
                if status != ncclResult_t_ncclSuccess {
                    return Err(format!("all-reduce of {count} values failed with {status}"));
                }
                stream.synchronize().map_err(|error| error.to_string())?;
                let actual = output
                    .to_host_vec(&stream)
                    .map_err(|error| error.to_string())?;
                if actual.get(..count) != values.get(..count) {
                    return Err(format!(
                        "one-rank result differs for count {count}: {actual:?}"
                    ));
                }
            }
            Ok(())
        })();
        let cleanup = finish_comm(
            work.is_ok(),
            || stream.synchronize(),
            // SAFETY: the unique communicator is destroyed only after successful synchronization.
            || unsafe { ncclCommDestroy(comm) },
            // SAFETY: abort releases this unique communicator before failed work can block a wait.
            || unsafe { ncclCommAbort(comm) },
        );
        assert!(
            work.is_ok() && cleanup.succeeded(),
            "work: {work:?}; cleanup: {cleanup:?}"
        );
    }
}
