//! Propagates `has_ngx_sdk` from `ngx-sys` (`links = "ngx"` metadata) to `ngx`.

fn main() {
    println!("cargo::rustc-check-cfg=cfg(has_ngx_sdk)");
    println!("cargo::rerun-if-env-changed=DEP_NGX_HAS_NGX_SDK");
    if std::env::var("DEP_NGX_HAS_NGX_SDK").as_deref() == Ok("1") {
        println!("cargo::rustc-cfg=has_ngx_sdk");
    }
}
