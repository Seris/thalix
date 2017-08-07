use core;
use super::boot::vga_buffer::Color;
use super::boot::vga_buffer;

#[no_mangle]
#[lang = "panic_fmt"]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
	early_kprintln!("");
	vga_buffer::set_color(Color::White, Color::Red);
	early_kprintln!("KERNEL PANIC: {}:{}", file, line);
	early_kprint!("\t{}", fmt);
	loop {}
}

#[lang = "eh_personality"]
extern fn eh_personality(){}
