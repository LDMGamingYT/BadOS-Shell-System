use x86_64::{structures::paging::PageTable, PhysAddr, VirtAddr};

/// returns mutable reference to active l4 table
/// 
/// unsafe because the caller must ensure the complete physical memory 
/// is mapped to virutal memory at the passed `phys_mem_offset`
/// 
/// this function must only be caleld once to avalid aliasing `&mut` references
pub unsafe fn active_l4_table(phys_mem_offset: VirtAddr) -> &'static mut PageTable {
	use x86_64::registers::control::Cr3;

	let (l4_table_frame, _) = Cr3::read();

	let phys = l4_table_frame.start_address();
	let virt = phys_mem_offset + phys.as_u64();
	let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

	&mut *page_table_ptr
}

pub unsafe fn translate_addr(addr: VirtAddr, phys_mem_offset: VirtAddr) -> Option<PhysAddr> {
	translate_addr_safely(addr, phys_mem_offset)
}

/// called by `translate_addr` to limit the scope of the unsafe block
fn translate_addr_safely(addr: VirtAddr, phys_mem_offset: VirtAddr) -> Option<PhysAddr> {
	use x86_64::structures::paging::page_table::FrameError;
	use x86_64::registers::control::Cr3;

	// read active l4 frame from CR3 register
	let (l4_table_frame, _) = Cr3::read();
	let table_indexes = [
		addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index()
	];
	let mut frame = l4_table_frame;

	// traverse multi-level page table
	for &i in &table_indexes {
		// convert frame to page table reference
		let virt = phys_mem_offset + frame.start_address().as_u64();
		let table_ptr: *const PageTable = virt.as_ptr();
		let table = unsafe { &*table_ptr };

		// read page table entry and update frame
		let entry = &table[i];
		frame = match entry.frame() {
			Ok(frame) => frame,
			Err(FrameError::FrameNotPresent) => return None,
			Err(FrameError::HugeFrame) => panic!("huge pages are not supported"),
		};
	}

	Some(frame.start_address() + u64::from(addr.page_offset()))
}