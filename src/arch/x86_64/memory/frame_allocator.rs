use super::multiboot2::{ElfSection, MemoryAreaIter, MemoryArea};
use arch::boot::get_multiboot::get_boot_information;
use super::frame::Frame;
use memory::PhysicalAddress;
use memory::traits::FrameAllocator;

#[derive(Debug)]
pub struct AreaFrameAllocator {
	kernel_start: PhysicalAddress,
	kernel_end: PhysicalAddress,
	multiboot_start: PhysicalAddress,
	multiboot_end: PhysicalAddress,
	memory_tags: MemoryAreaIter,
	current_area: &'static MemoryArea,
	current_frame: Frame
}

impl AreaFrameAllocator {
	pub fn new() -> AreaFrameAllocator {
		let mut memory_tags: MemoryAreaIter = get_boot_information().memory_map_tag()
			.expect("memory map from bootloader").memory_areas();

		let memory_size = memory_tags.sum();

		let current_area: &'static MemoryArea = memory_tags.next().expect("available memory");

		let mut allocator = AreaFrameAllocator {
			kernel_start: 0,
			kernel_end: 0,
			multiboot_start: 0,
			multiboot_end: 0,
			memory_tags: memory_tags,
			current_area: current_area,
			current_frame: Frame::containing_address(current_area.base_addr as PhysicalAddress)
		};

		allocator.configure();
		allocator
	}

	fn configure(&mut self){
		let mboot = get_boot_information();

		let mut sec =
			mboot.elf_sections_tag().expect("elf sections must be available").sections();

		let first_section: &'static ElfSection = sec.next().expect("first elf section ");
		let last_section = sec.last().expect("last elf section ");

		self.kernel_start =	first_section.start_address();
		self.kernel_end = last_section.end_address();

		self.multiboot_start = mboot.start_address();
		self.multiboot_end = mboot.end_address();
	}

	fn frame_in_area(&self) -> bool {
		let end_area = self.current_area.base_addr + self.current_area.length as u64;
		self.current_frame.end_address() < end_area as usize
	}

	fn is_frame_valid(&self) -> bool {
		let start = self.current_frame.start_address();
		let end = self.current_frame.end_address();

		(start < self.kernel_start || start >= self.kernel_end) &&
		(end <= self.kernel_start || end > self.kernel_end) &&
		(start < self.multiboot_start || start >= self.multiboot_end) &&
		(end <= self.multiboot_start || end > self.multiboot_end)
	}
}

impl FrameAllocator for AreaFrameAllocator {
	type Frame = Frame;

	fn alloc_frame(&mut self) -> Option<Frame> {
		let mut frame = None;
		while let None = frame {
			if !self.frame_in_area(){
				if let Some(area) = self.memory_tags.next() {
					self.current_area = area;
					self.current_frame =
						Frame::containing_address(self.current_area.base_addr as usize);
				} else {
					break;
				}
			}
			else if self.is_frame_valid() {
				frame = Some(self.current_frame)
			}

			self.current_frame.next_frame();
		}

		frame
	}

	fn free_frame(&mut self, frame: &Frame) {
		unimplemented!();
	}
}
