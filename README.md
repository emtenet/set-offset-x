# Overview

This repository is to support issue # 1017 of https://github.com/microsoft/windows-rs

At the time of this writting it applies to version 0.18.0 of the [windows](https://crates.io/crates/windows) crate

## Original issue

My original issue was that calls to `IDCompositionVisual2::SetOffsetX(&self, offsetx: f32)`
would either fail with `E_INVALIDARG` or crash.

A simple project exhibiting this behaviour is in the `issue` directory.

Compiled with:

- Microsoft Windows 10 Pro
- 10.0.19043 N/A Build 19043
- rustc 1.52.1 (9bc8c42bb 2021-05-09)
- Toolchain: stable-i686-pc-windows-msvc

When running this project you would expect it to print **SUCCESS**.

Instead it either panics:

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: 
  Error { code: 0x80070057, message: "The parameter is incorrect.", win32_code: 87 }
', src\main.rs:49:32
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

or, it crashes if something other than `0.0` is passed to `SetOffsetX`.

## Working projects

For comparison, two working projects are included in the 
`ok_rust` and `ok_cpp` directories.

These two projects implement the same logic but with different crates or language.

The two comparison projects successfully print **SUCCESS**.

- The `ok_rust` project uses the `winapi` and `wio` crates rather than the `windows` crate.
- The `ok_cpp` project is written in `C++` using Visual Studio 2019.

