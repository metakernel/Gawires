/// gawires_core is the shared crate used by all other gawires crates. This where all the kernel is implemented

pub extern crate gawires_core;
pub use gawires_core::*;
pub mod extensiontypes;

#[no_mangle]
pub extern "C" fn test(){
    println!("Testing extension lib");
}