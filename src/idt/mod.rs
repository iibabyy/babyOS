use core::arch::asm;

use crate::{keyboard::keyboard_interrupt_handler, println};

pub mod entry;
use entry::{IdtEntry, IdtPointer};

static mut IDT: [IdtEntry; 256] = [IdtEntry::zeroed(); 256];
static mut IDT_PTR: IdtPointer = IdtPointer { limit: 0, base: 0 };

#[derive(Debug)]
#[repr(C)]
pub struct InterruptStackFrame {
    pub instruction_pointer: u32,
    pub code_segment: u32,
    pub cpu_flags: u32,
}

#[expect(static_mut_refs)]
pub fn init_idt() {
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

pub fn init_interrupt_handlers() {
    register_interrupt_handler(Interrupt::Breakpoint, breakpoint_interrupt_handler);
    register_interrupt_handler(Interrupt::Keyboard, keyboard_interrupt_handler);
}

pub fn register_interrupt_handler(
    interrupt: Interrupt,
    handler: extern "x86-interrupt" fn(&mut InterruptStackFrame),
) {
    const CODE_SEGMENT_OFFSET: u16 = core::mem::size_of::<IdtEntry>() as u16;

    let handler_address = handler as *const () as u32;

    unsafe {
        IDT[interrupt as usize].set_handler_fn(handler_address, CODE_SEGMENT_OFFSET);
    }
}

// The Breakpoint Handler
extern "x86-interrupt" fn breakpoint_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

pub enum Interrupt {
    Breakpoint = 3,
    Keyboard = 33,
}

/// enables CPU hardware interrupts
pub fn enable() {
    unsafe {
        core::arch::asm!("sti");
    }
}

// /// disables CPU hardware interrupts
// pub fn disable() {
//     unsafe {
//         core::arch::asm!("cli");
//     }
// }
