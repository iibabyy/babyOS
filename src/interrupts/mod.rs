use crate::println;

pub mod idt;

#[derive(Debug)]
#[repr(C)]
pub struct InterruptStackFrame {
    pub instruction_pointer: u32,
    pub code_segment: u32,
    pub cpu_flags: u32,
}

// The Breakpoint Handler
extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    // Assuming you have a println! macro implemented
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
