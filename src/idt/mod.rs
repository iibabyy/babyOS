pub mod entry;
pub mod interrupts;

use core::arch::asm;

use self::interrupts::init_interrupt_handlers;
use self::entry::{
	IdtEntry,
	IdtPointer,
};

static mut IDT: [IdtEntry; 256] = [IdtEntry::zeroed(); 256];
static mut IDT_PTR: IdtPointer = IdtPointer {
	limit: 0,
	base: 0,
};

/// The stack frame that interrupt handlers receive as arguments when they are
/// called
#[derive(Debug)]
#[repr(C)]
pub struct InterruptStackFrame {
	pub instruction_pointer: u32,
	pub code_segment: u32,
	pub cpu_flags: u32,
}

/// Initialize the IDT table
///
/// SAFETY: GDT must be initialized
#[expect(static_mut_refs)]
pub unsafe fn init_idt() {
	unsafe {
		init_interrupt_handlers();

		// Tell the CPU the memory address and size of our IDT array
		IDT_PTR.limit = (core::mem::size_of::<[IdtEntry; 256]>() - 1) as u16;
		IDT_PTR.base = IDT.as_ptr() as u32;

		// loads the idt
		asm!(
			"lidt [{ptr}]",
			ptr = in(reg) &IDT_PTR
		);
	}
}
