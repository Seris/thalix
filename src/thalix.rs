#![feature(lang_items)]
#![no_std]

extern crate rlibc;

#[no_mangle]
pub extern fn kernel_main(){
	let hello = b"Un homme rentre dans un bar et dit \"c'est moi\", mais en fait c'etait pas lui !";

	let vga_buffer;
	unsafe {
		vga_buffer = &mut *(0xB8000 as *mut [u8; 160*25]);
	}

	for i in 0..4000 {
		vga_buffer[i] = 0x4f;
	}

	loop {}
}

#[no_mangle]
#[lang = "panic_fmt"]
pub extern fn panic_fmt(){ loop {} }

#[lang = "eh_personality"]
extern fn eh_personality(){}
