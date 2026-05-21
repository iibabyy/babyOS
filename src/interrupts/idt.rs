use lazy_static::lazy_static;
use x86::{dtables::{DescriptorTablePointer, lidt}, segmentation::cs};

use crate::interrupts::breakpoint_handler;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // Breakpoint is exception vector 3
        idt.0[3].set_handler_fn(breakpoint_handler as *const () as u32);

        idt
    };
}

pub fn init_idt() {
    let idt_pointer = DescriptorTablePointer::new_from_slice(&IDT.0);

    // Load it using the x86 crate
    unsafe {
        lidt(&idt_pointer);
    }
}

pub struct InterruptDescriptorTable(pub [IdtEntry; 256]);

impl InterruptDescriptorTable {
    pub const fn new() -> Self {
        Self([IdtEntry::empty(); 256])
    }
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C, packed)]
pub struct IdtEntry {
    offset_low: u16,      // Lower 16 bits of handler function address
    selector: u16,        // Kernel code segment selector (usually 0x08)
    zero: u8,             // Always 0
    type_attr: u8,        // Gate type and attributes (0x8E for 32-bit interrupt gate)
    offset_high: u16,     // Higher 16 bits of handler function address
}

impl IdtEntry {
	pub const fn empty() -> Self {
		Self {
			offset_low: 0,
			selector: 0,
			zero: 0,
			type_attr: 0,
			offset_high: 0,
		}		
	}

	pub fn set_handler_fn(&mut self, handler_ptr: u32) {
		self.offset_low = handler_ptr as u16;
		self.offset_high = (handler_ptr >> 16) as u16;

		// Dynamically fetch the actual Code Segment register
        self.selector = cs().bits();
		self.type_attr = 0x8E;
	}
}
