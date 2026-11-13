//! GPU tests for `optix-device`: OptiX programs written in Rust (compiled by cuda-oxide into this
//! binary's embedded PTX) driven by the host `optix` crate. Run with `cargo oxide run --arch
//! <sm>` (optionally followed by case names to run only those); every case prints PASS or FAIL
//! with its first mismatches, and the process exits non-zero when any case fails. The cases
//! live in [`cases`], one module each.
#![expect(non_snake_case, reason = "OptiX entry points are named by their semantic prefix (`__raygen__*`, `__closesthit__*`, ...)")]

mod cases;

use cuda_core::embedded::{ArtifactPayloadKind, embedded_modules_from_current_exe};

fn main() {
    let ptx = match embedded_modules_from_current_exe().map(|m| m.iter().find_map(|m| m.payload(ArtifactPayloadKind::Ptx)).map(<[u8]>::to_vec)) {
        Ok(Some(p)) => p,
        Ok(None) => fail("no PTX payload embedded in this binary"),
        Err(e) => fail(&format!("embedded modules: {e}")),
    };
    // Kept for inspection when OptiX rejects the module.
    if let Err(e) = std::fs::create_dir_all("target").and_then(|()| std::fs::write("target/optix_device_tests.ptx", &ptx)) {
        fail(&format!("write target/optix_device_tests.ptx: {e}"));
    }
    let gpu = match cases::common::Gpu::new(ptx) {
        Ok(g) => g,
        Err(e) => fail(&e),
    };
    if let Ok(path) = std::env::var("ODT_PTX_PROBE") {
        println!("probe {path}: {:?}", probe(&gpu, &path));
        return;
    }
    let only: Vec<String> = std::env::args().skip(1).collect();
    let mut failed = 0;
    for (name, case) in cases::ALL.iter().filter(|(n, _)| only.is_empty() || only.iter().any(|o| o == n)) {
        match case(&gpu) {
            Ok(()) => println!("{name} PASS"),
            Err(msg) => {
                eprintln!("{name} FAIL: {msg}");
                failed += 1;
            }
        }
    }
    if failed > 0 {
        eprintln!("{failed} case(s) failed");
        std::process::exit(1);
    }
}

/// Diagnostic: runs raygen `__raygen__lp` of a hand-written PTX (32 threads, `params_lp.out`)
/// and returns the words it wrote.
fn probe(g: &cases::common::Gpu, path: &str) -> Result<Vec<u32>, String> {
    use cases::common::Ctx;
    let ptx = std::fs::read(path).ctx("read")?;
    let o = cases::common::opts("params_lp", 16)?;
    let (m, _) = optix::Module::new(&g.ctx, &Default::default(), &o, &ptx).ctx("module")?;
    let rg = optix::ProgramGroup::raygen(&m, "__raygen__lp").ctx("rg")?;
    let ms = optix::ProgramGroup::miss(&m, "__miss__lp").ctx("ms")?;
    let hg = optix::ProgramGroup::hit_group(&g.ctx, optix::HitGroup::default()).ctx("hg")?;
    let p = g.pipeline(&o, optix::LinkOptions { max_trace_depth: 0, ..Default::default() }, &[rg.clone(), ms.clone(), hg.clone()])?;
    let sbt = optix::ShaderBindingTable::new(&g.stream, &optix::SbtRecord::new(&rg, 0u32).ctx("r")?, &[optix::SbtRecord::new(&ms, 0u32).ctx("m")?], &[optix::SbtRecord::new(&hg, 0u32).ctx("h")?]).ctx("sbt")?;
    let out = g.zeros::<u32>(32)?;
    let params = [out.cu_deviceptr(), 0u64];
    // SAFETY: diagnostic; the PTX writes 32 words to `out`.
    unsafe { g.launch(&p, &sbt, &params, [32, 1, 1]) }?;
    g.down(&out)
}

fn fail(msg: &str) -> ! {
    eprintln!("setup FAIL: {msg}");
    std::process::exit(2);
}
