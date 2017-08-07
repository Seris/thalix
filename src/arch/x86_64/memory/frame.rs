use super::PAGE_SIZE;
use memory;
use memory::PhysicalAddress;

#[derive(Debug, Copy, Clone)]
pub struct Frame {
	number: usize
}

impl Frame {
	/// Return next frame whether it's valid or not
	pub fn next_frame(&mut self) {
		self.number += 1;
	}
}

impl memory::traits::Frame for Frame {
	fn containing_address(address: PhysicalAddress) -> Frame {
		Frame {
			number: address / PAGE_SIZE
		}
	}

	// Start address is inclusive (the address is contained in the frame)
	fn start_address(&self) -> PhysicalAddress {
		PAGE_SIZE * self.number
	}

	/// End address is not inclusive (not inside the frame).
	/// This address represent in fact the address of the next frame
	fn end_address(&self) -> PhysicalAddress {
		 PAGE_SIZE * (self.number + 1)
	}
}
