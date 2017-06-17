#![feature(asm, lang_items, const_fn, unique)]
#![no_std]

#[macro_use]
pub mod boot;
pub mod panic;
pub mod memory;

extern crate rlibc;
extern crate spin;
extern crate multiboot2;

#[no_mangle]
pub extern "C" fn kernel_main(){
    boot::setup();
	panic!("reached end of kernel_main");
}
