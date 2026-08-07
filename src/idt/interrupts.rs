use crate::idt::entry::IdtEntry;
use crate::idt::{
	IDT,
	InterruptStackFrame,
};
use crate::keyboard::keyboard_interrupt_handler;
use crate::memory::page_fault::page_fault_interrupt_handler;
use crate::println;

/// Initialize our interrupt handlers
pub fn init_interrupt_handlers() {
	register_standard_interrupt(Interrupt::Breakpoint, breakpoint_interrupt_handler);
	register_standard_interrupt(Interrupt::Keyboard, keyboard_interrupt_handler);

	register_error_code_interrupt(Interrupt::PageFault, page_fault_interrupt_handler);
}

/// register the `handler` [InterruptHandler::Standard] for the `interrupt` [Interrupt]
pub fn register_standard_interrupt(
	interrupt: Interrupt,
	handler: extern "x86-interrupt" fn(&mut InterruptStackFrame),
) {
	register_interrupt_handler(interrupt, InterruptHandler::Standard(handler));
}

/// register the `handler` [InterruptHandler::WithErrorCode] for the `interrupt` [Interrupt]
pub fn register_error_code_interrupt(
	interrupt: Interrupt,
	handler: extern "x86-interrupt" fn(&mut InterruptStackFrame, u32),
) {
	register_interrupt_handler(interrupt, InterruptHandler::WithErrorCode(handler));
}

/// register the `handler` [InterruptHandler] for the `interrupt` [Interrupt] into the [IDT]
pub fn register_interrupt_handler(interrupt: Interrupt, handler: InterruptHandler) {
	const CODE_SEGMENT_OFFSET: u16 = core::mem::size_of::<IdtEntry>() as u16;

	unsafe {
		IDT[interrupt as usize].set_handler_fn(handler.address(), CODE_SEGMENT_OFFSET);
	}
}

extern "x86-interrupt" fn breakpoint_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
	println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

pub enum InterruptHandler {
	Standard(extern "x86-interrupt" fn(&mut InterruptStackFrame)),
	WithErrorCode(extern "x86-interrupt" fn(&mut InterruptStackFrame, u32)),
}

impl InterruptHandler {
	pub fn address(self) -> u32 {
		match self {
			Self::Standard(f) => f as *const () as u32,
			Self::WithErrorCode(f) => f as *const () as u32,
		}
	}
}

/// Interrupt IDs
pub enum Interrupt {
	Breakpoint = 3,
	PageFault = 14,
	Keyboard = 33,
}

/// Enables CPU hardware interrupts (e.g. keyboard keys)
///
/// SAFETY: IDT must be initialized
pub unsafe fn enable_hardware_interrupts() {
	unsafe {
		core::arch::asm!("sti");
	}
}

/// Disables CPU hardware interrupts
pub fn disable_hardware_interrupts() {
	unsafe {
		core::arch::asm!("cli");
	}
}
