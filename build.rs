use std::env;
use std::path::PathBuf;

fn main() {
    let cc = env::var("CC").expect("need to set `CC=path/to/clang` that supports `wasm32-wasi`");
    let sysroot = env::var("SYSROOT").expect("need to set `SYSROOT=path/to/wasi-libc`");
    let ranlib = env::var("RANLIB")
        .expect("need to set `RANLIB=path/to/ranlib` that supports `wasm32-wasi`");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_obj = out_dir.join("dupeimport.o");
    let out_lib = out_dir.join("libdupeimport.a");

    let status = std::process::Command::new(&cc)
        .arg("--target=wasm32-unknown-wasi")
        .arg("--sysroot")
        .arg(&sysroot)
        .arg("-o")
        .arg(&out_obj)
        .arg("-c")
        .arg("dupeimport.c")
        .status()
        .unwrap();
    assert!(status.success());

    let status = std::process::Command::new("ar")
        .arg("rcs")
        .arg(&out_lib)
        .arg(&out_obj)
        .status()
        .unwrap();
    assert!(status.success());

    let status = std::process::Command::new(ranlib)
        .arg(&out_lib)
        .status()
        .unwrap();
    assert!(status.success());

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-lib=dupeimport");
}
