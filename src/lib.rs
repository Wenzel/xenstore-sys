#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(not(feature = "manual"))]
include!(concat!(env!("OUT_DIR"), "/auto_bindings.rs"));

#[cfg(feature = "manual")]
include!("manual_bindings.rs");
