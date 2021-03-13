#![feature(stdsimd)]
mod multiply;
pub use core_arch::arch::x86_64 as x86_64;
pub use multiply::*;