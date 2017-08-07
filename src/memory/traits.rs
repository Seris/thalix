use super::PhysicalAddress;

pub trait FrameAllocator {
	type Frame: Sized;

	fn alloc_frame(&mut self) -> Option<Self::Frame>;
	fn free_frame(&mut self, &Self::Frame);
}

pub trait Frame {
	fn containing_address(address: PhysicalAddress) -> Self;
	fn start_address(&self) -> PhysicalAddress;
	fn end_address(&self) -> PhysicalAddress;
}
