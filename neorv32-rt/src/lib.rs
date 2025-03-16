#![no_std]

#[cfg(neorv32)]
use core::arch::global_asm;

#[cfg(neorv32)]
global_asm!(include_str!(concat!(env!("OUT_DIR"), "/crt0.S")));

// TODO: Add a compiler fence after crt0 and before calling main.
