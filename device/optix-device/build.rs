//! Generates the raw OptiX device layer: one `ptx_asm!` wrapper per `_optix_*` intrinsic of the
//! installed OptiX SDK's device headers (located by vcv-rs), plus an inventory with the header
//! hashes. Fails when any intrinsic in the headers has no wrapper. Headers are never committed.

use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-env-changed=OPTIX_ROOT");
    println!("cargo:rerun-if-env-changed=OptiX_INSTALL_DIR");
    let Some(optix) = vcv_rs::optix::detect_optix() else {
        println!("cargo::error=OptiX SDK not found (set OPTIX_ROOT or install the OptiX SDK)");
        std::process::exit(1);
    };
    let include = optix.include_dir();
    for h in nv_bindgen::device_asm::OPTIX_DEVICE_HEADERS {
        println!("cargo:rerun-if-changed={}", include.join(h).display());
    }
    let (scan, hashes) = match nv_bindgen::device_asm::scan_optix(&include) {
        Ok(v) => v,
        Err(e) => {
            println!("cargo::error={e}");
            std::process::exit(1);
        }
    };
    let out = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR"));
    let src = nv_bindgen::device_asm::emit(&scan, "optix_device_impl*.h", "::cuda_device::ptx_asm");
    std::fs::write(out.join("optix_raw.rs"), src).expect("write optix_raw.rs");
    let names: Vec<&String> = scan.intrinsics.keys().collect();
    let inventory = format!(
        "{{\"intrinsics\":{},\"names\":[{}],\"headers\":[{}]}}",
        names.len(),
        names.iter().map(|n| format!("\"{n}\"")).collect::<Vec<_>>().join(","),
        hashes.iter().map(|(h, s)| format!("{{\"file\":\"{h}\",\"sha256\":\"{s}\"}}")).collect::<Vec<_>>().join(",")
    );
    std::fs::write(out.join("optix_device_inventory.json"), inventory).expect("write inventory");
}
