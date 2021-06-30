# `wasm-ld` does not de-duplicate WASI imports between different languages

This test case links C into a Rust project. I've also reproduced this by linking
Rust into a C project. I assume other languages linked together with `wasm-ld`
would have the same issue?

I can't find any `wasm-ld` CLI flags that seem to control
this. Enabling/disabling LTO and GC sections, for example, doesn't make a
difference.

## Steps to Reproduce

* Ensure you have Rust toolchain that can target `wasm32-wasi`:

  ```
  $ rustup target add wasm32-wasi
  ```

* Ensure you have a C toolchain that can target `wasm32-unknown-wasi`. Multiple
  ways to do this, easiest is via
  [wasi-sdk](https://github.com/WebAssembly/wasi-sdk).

* Clone this repo and `cd` into it.

* Build:

  ```
  $ CC=path/to/wasi-sdk/bin/clang \
    SYSROOT=path/to/wasi-sdk/share/wasi-sysroot \
    RANLIB=path/to/wasi-sdk/bin/ranlib \
    cargo build --target wasm32-wasi
  ```

* Inspect the built `.wasm` file's imports:

  ```
  $ wasm-objdump -x -j Import target/wasm32-wasi/debug/lld_dupe_import.wasm
  ```

## Expected Results

`"wasi_snapshot_preview1" "fd_write"` is imported once.

## Actual Results

`"wasi_snapshot_preview1" "fd_write"` is imported twice:

```
lld_dupe_import.wasm:	file format wasm 0x1

Section Details:

Import[8]:
 - func[0] sig=7 <_ZN4wasi13lib_generated22wasi_snapshot_preview18fd_write17h133cd9fedd86597bE> <- wasi_snapshot_preview1.fd_write
 - func[1] sig=0 <__wasi_proc_exit> <- wasi_snapshot_preview1.proc_exit
 - func[2] sig=3 <__wasi_environ_sizes_get> <- wasi_snapshot_preview1.environ_sizes_get
 - func[3] sig=3 <__wasi_environ_get> <- wasi_snapshot_preview1.environ_get
 - func[4] sig=8 <__wasi_fd_seek> <- wasi_snapshot_preview1.fd_seek
 - func[5] sig=7 <__wasi_fd_write> <- wasi_snapshot_preview1.fd_write
 - func[6] sig=9 <__wasi_fd_close> <- wasi_snapshot_preview1.fd_close
 - func[7] sig=3 <__wasi_fd_fdstat_get> <- wasi_snapshot_preview1.fd_fdstat_get
```
