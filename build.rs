use std::env;

fn main() {
    let base_path = env!("CARGO_MANIFEST_DIR");

    println!("cargo:rustc-link-search={base_path}/nuttx-export/libs");
    println!("cargo:rustc-link-lib=apps");
    println!("cargo:rustc-link-lib=arch");
    println!("cargo:rustc-link-lib=binfmt");
    println!("cargo:rustc-link-lib=board");
    println!("cargo:rustc-link-lib=boards");
    println!("cargo:rustc-link-lib=c");
    println!("cargo:rustc-link-lib=drivers");
    println!("cargo:rustc-link-lib=fs");
    println!("cargo:rustc-link-lib=gcc");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=mm");
    println!("cargo:rustc-link-lib=sched");
    println!("cargo:rustc-link-lib=wireless");

    let bindings = bindgen::builder()
        .header("wrapper.h")
        .clang_arg(format!("-I{base_path}/nuttx-export/include"))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .use_core()
        .generate()
        .unwrap();
    bindings
        .write_to_file(format!("{}/nuttx.rs", env::var("OUT_DIR").unwrap()))
        .unwrap();

    imxrt_rt::RuntimeBuilder::from_flexspi(imxrt_rt::Family::Imxrt1060, 256 * 1024 * 1024)
        .build()
        .unwrap();
}
