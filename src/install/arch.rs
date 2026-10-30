use anyhow::{bail, Result};
use std::fmt;

use crate::target;

/// An enum representing supported architectures
#[derive(Clone, PartialEq, Eq)]
pub enum Arch {
    /// x86 64-bit
    X86_64,
    /// x86 32-bit
    X86,
    /// ARM 64-bit
    AArch64,
    /// RISC-V 64-bit
    Riscv64,
}

impl Arch {
    /// Gets the current architecture
    pub fn get() -> Result<Self> {
        if target::x86_64 {
            Ok(Arch::X86_64)
        } else if target::x86 {
            Ok(Arch::X86)
        } else if target::aarch64 {
            Ok(Arch::AArch64)
        } else if target::riscv64 {
            Ok(Arch::Riscv64)
        } else {
            bail!("Unrecognized target!")
        }
    }
}

impl fmt::Display for Arch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Arch::X86_64 => "x86-64",
            Arch::X86 => "x86",
            Arch::AArch64 => "aarch64",
            Arch::Riscv64 => "riscv64",
        };
        write!(f, "{}", s)
    }
}
