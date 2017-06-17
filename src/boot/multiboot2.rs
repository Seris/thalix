use multiboot2;

extern "C" {
	static multiboot_address: u32;
}

pub fn get_boot_information<'a>() -> &'a multiboot2::BootInformation {
	unsafe { multiboot2::load(multiboot_address as usize) }
}
