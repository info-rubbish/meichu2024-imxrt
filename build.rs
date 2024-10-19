use std::{env, fs};

fn main() {
    println!("cargo:rustc-link-arg-bins=--entry=__start");

    let base_path = env!("CARGO_MANIFEST_DIR");

    println!("cargo:rustc-link-arg-bins=-T{base_path}/nuttx-export/scripts/flash.ld");
    println!("cargo:rustc-link-search={base_path}/nuttx-export/libs");

    println!("cargo:rustc-link-arg-bins=--start-group");
    let libs = fs::read_dir(format!("{base_path}/nuttx-export/libs")).unwrap();
    for lib in libs {
        let lib_name = lib.unwrap().file_name().into_string().unwrap();
        let lib_name = lib_name
            .strip_prefix("lib")
            .unwrap()
            .strip_suffix(".a")
            .unwrap();
        println!("cargo:rustc-link-arg-bins=-l{lib_name}");
    }
    println!("cargo:rustc-link-arg-bins=--end-group");

    let bindings = bindgen::builder()
        .header("wrapper.h")
        .clang_args([
            format!("-I{base_path}/nuttx-export/include"),
            format!("-I{base_path}/nuttx-export/arch/chip"),
            format!("-I{base_path}/nuttxspace/nuttx/arch/arm/src/imxrt"),
        ])
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .use_core()
        .generate_cstr(true)
        .generate()
        .unwrap();
    bindings
        .write_to_file(format!("{}/nuttx.rs", env::var("OUT_DIR").unwrap()))
        .unwrap();

    slint_build::compile_with_config(
        "ui/main.slint",
        slint_build::CompilerConfiguration::new()
            .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer),
    )
    .unwrap();
}
