#![cfg_attr(feature="serde_derive", feature(proc_macro))]

#[cfg(feature = "serde_derive")]
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate libc;
extern crate seahash;


pub mod json_types;
