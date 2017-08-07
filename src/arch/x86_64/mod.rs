extern crate rlibc;
extern crate multiboot2;

#[macro_use]
pub mod boot;
#[macro_use]
pub mod panic;
pub mod memory;

pub use self::boot::vga_buffer;

pub fn setup(){
	vga_buffer::set_color(vga_buffer::Color::White, vga_buffer::Color::Black);
	vga_buffer::clear_screen();
}
