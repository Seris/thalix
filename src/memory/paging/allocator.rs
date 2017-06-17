use multiboot2;
use spin;

pub const PAGE_SIZE: usize = 4096;

pub trait FrameAllocator {
	fn alloc_frame(&mut self, size: usize) -> Frame;
	fn free_frame(&mut self, frame: Frame);
}

#[derive(Debug)]
pub struct BasicFrameAllocator {
	current_page: usize,
	kernel_start: u64,
	kernel_end: u64,
	multiboot_start: u64,
	multiboot_end: u64,
	memory_end: u64,
	initialized: bool
}

impl BasicFrameAllocator {
	pub fn init(&mut self, info: &multiboot2::BootInformation) {
		let elf = info.elf_sections_tag()
			.expect("elf section required to detect kernel address in memory");


		let first_section = elf.sections().next().unwrap();
		let last_section = elf.sections().last().unwrap();

		self.kernel_start = first_section.addr;
		self.kernel_end = last_section.addr + last_section.size;

		self.multiboot_start = info.start_address() as u64;
		self.multiboot_end = info.end_address() as u64;

		self.initialized = true;
	}

	fn frame_available(&self, page_start: usize, frame_size: usize) -> bool {
		let page_end = page_start + frame_size;

		for page in page_start .. page_end {
			let address: u64 = (page * PAGE_SIZE) as u64;
			if address >= self.kernel_start && address <= self.kernel_end {
				return false;
			} else if address >= self.multiboot_start && address <= self.multiboot_end {
				return false;
			}
		}

		true
	}
}

impl FrameAllocator for BasicFrameAllocator {
	fn alloc_frame(&mut self, frame_size: usize) -> Frame {
		let mut frame_found: bool = false;
		let mut frame = Frame { page_start: 0, page_end: 0 };

		while !frame_found {
			if self.frame_available(self.current_page, frame_size) {

			}
		}

		frame
	}

	fn free_frame(&mut self, frame: Frame){

	}
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Frame {
	page_start: usize,
	page_end: usize
}

pub static ALLOCATOR: spin::Mutex<BasicFrameAllocator> = spin::Mutex::new(BasicFrameAllocator {
	memory_end: 0,
	current_page: 0,
	kernel_start: 0,
	kernel_end: 0,
	multiboot_start: 0,
	multiboot_end: 0,
	initialized: false
});
