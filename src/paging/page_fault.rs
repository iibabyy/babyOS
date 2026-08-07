use core::arch::asm;

use modular_bitfield::{bitfield, specifiers::B25};

use crate::idt::InterruptStackFrame;

pub extern "x86-interrupt" fn page_fault_interrupt_handler(
	stack_frame: &mut InterruptStackFrame,
	error_code: u32
) {
	// the virtual address that caused the page fault
	let faulting_address = read_cr2();

	let parsed_error = PageFaultErrorCode::from_bytes(error_code.to_le_bytes());

	panic!(
		"EXCEPTION: PAGE FAULT\n\
		Accessed Address: {faulting_address:#010X}\n\
		Error Context: {parsed_error:#?}\n\
		Stack Frame: {stack_frame:#?}",
	)
}

/// Returns the virtual address that caused the page fault (from the CR2 register)
pub fn read_cr2() -> u32 {
	let value: u32;
	unsafe {
		asm!(
			"mov {}, cr2",
			out(reg) value,
		);
	}
	value
}

#[bitfield(bits = 32)]
#[derive(Debug, Clone, Copy)]
pub struct PageFaultErrorCode {
	/// Was the virtual address mapped in the [PageDirectory] ?
	/// 
	/// false = virtual address was mapped
	/// true = virtual address was not mapped
	/// 
	/// [PageDirectory]: crate::paging::page_directory::PageDirectory
	pub is_mapped: bool,

	/// Was the instruction attempting to write memory ?
	/// 
	/// false = read access error
	/// true = write access error
	pub is_write: bool,

	/// Was the CPU executing in Ring 3 (kernel privileges) when the fault happened ?
	/// 
	/// false = kernel mode
	/// true = user mode
	pub is_user_mode: bool,

	/*
	 *	The next fields are not used, as we can rely on the first three for now
	 */

	/// Has a CPU-reserved bit has been modified ?
	pub is_reserved_write: bool,

	/// Has the page fault occured while trying to fetch the next instruction ?
	pub is_instruction_fetch: bool,

	/*
	 *	The meaning of these two fields is too complex for now.
	 *	But we don't need to care about them, as specified above
	 */

	pub is_protection_key: bool,
	pub is_shadow_stack: bool,

	#[skip]
    reserved: B25,
}