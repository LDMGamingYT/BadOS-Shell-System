use x86_64::structures::paging::{OffsetPageTable, PageTable};
use x86_64::VirtAddr;

/// init new OffsetPageTable
/// 
/// unsafe because the caller must ensure the complete physical memory 
/// is mapped to virutal memory at the passed `phys_mem_offset`
/// 
/// this function must only be called once to avalid aliasing `&mut` references
pub unsafe fn init(phys_mem_offset: VirtAddr) -> OffsetPageTable<'static> {
	let l4_table = active_l4_table(phys_mem_offset);
	OffsetPageTable::new(l4_table, phys_mem_offset)
}

/// returns mutable reference to active l4 table
unsafe fn active_l4_table(phys_mem_offset: VirtAddr) -> &'static mut PageTable {
	use x86_64::registers::control::Cr3;

	let (l4_table_frame, _) = Cr3::read();

	let phys = l4_table_frame.start_address();
	let virt = phys_mem_offset + phys.as_u64();
	let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

	&mut *page_table_ptr
}