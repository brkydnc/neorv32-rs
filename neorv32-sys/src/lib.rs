#![no_std]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(neorv32)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
