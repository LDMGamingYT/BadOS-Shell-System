use x86_64::{structures::paging::PageTable, VirtAddr};

#[warn(unsafe_op_in_unsafe_fn)]
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

	unsafe { &mut *page_table_ptr }
}