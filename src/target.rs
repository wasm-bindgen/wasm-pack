//! Information about the target wasm-pack is currently being compiled for.
//!
//! That is, whether we are building wasm-pack for windows vs linux, and x86 vs
//! x86-64, etc.

#![allow(missing_docs)]

pub const LINUX: bool = cfg!(target_os = "linux");
pub const MACOS: bool = cfg!(target_os = "macos");
pub const WINDOWS: bool = cfg!(target_os = "windows");

#[allow(non_upper_case_globals)]
pub const x86_64: bool = cfg!(target_arch = "x86_64");
#[allow(non_upper_case_globals)]
pub const x86: bool = cfg!(target_arch = "x86");
#[allow(non_upper_case_globals)]
pub const aarch64: bool = cfg!(target_arch = "aarch64");
#[allow(non_upper_case_globals)]
pub const riscv64: bool = cfg!(target_arch = "riscv64");
