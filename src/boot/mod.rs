#[macro_use]
pub mod vga_buffer;
mod multiboot2;
use memory;

use self::vga_buffer::Color;

pub fn setup(){
	vga_buffer::set_color(Color::White, Color::Black);
	vga_buffer::clear_screen();

	early_kprintln!("setting up page allocation module");
	memory::paging::ALLOCATOR.lock().init(multiboot2::get_boot_information());
}
