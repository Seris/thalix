#![feature(asm, lang_items, const_fn, unique)]
#![no_std]

extern crate spin;

#[macro_use]
pub mod arch;
pub mod memory;

#[no_mangle]
pub extern "C" fn kernel_main(){
    arch::setup();
	panic!("reached end of kernel_main");
}
