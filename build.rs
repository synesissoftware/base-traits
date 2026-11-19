// build.rs : base-traits

use bt_rs::rustc;

fn main() {
    println!("cargo:rustc-check-cfg=cfg(sis_rustc_1_76_or_newer)");
    println!("cargo:rustc-check-cfg=cfg(sis_rustc_1_79_or_newer)");
    println!("cargo:rustc-check-cfg=cfg(sis_rustc_nightly)");

    let version = rustc::compiler_version().expect("failed to execute `rustc --version`");

    if version >= (1, 76) {
        println!("cargo:rustc-cfg=sis_rustc_1_76_or_newer");
    }

    if version >= (1, 79) {
        println!("cargo:rustc-cfg=sis_rustc_1_79_or_newer");
    }

    if version.is_nightly() {
        println!("cargo:rustc-cfg=sis_rustc_nightly");
    }
}

// ///////////////////////////// end of file //////////////////////////// //
