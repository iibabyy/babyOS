use lazy_static::lazy_static;
use crate::{idt::{self, breakpoint_handler}, shared::PrivilegeRing};
use modular_bitfield::prelude::*;

use crate::shared::PrivilegeRing;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct IdtEntry(RawIdtEntry);

impl IdtEntry {
    pub const fn zeroed() -> Self {
        Self(RawIdtEntry::new())
    }

    /// Sets the handler function and the required options.
    pub fn set_handler_fn(&mut self, handler_address: u32, code_segment_offset: u16) {
        let options = IdtEntryOptions::new()
            .with_is_present(true)
            .with_privilege_level(PrivilegeRing::Kernel)
            .with_is_storage_segment(false) // Must be false for Interrupt/Trap gates
            .with_gate_type(IdtGateType::InterruptGate32); // 0xE

        self.0.set_offset_low_bits(handler_address as u16);
        self.0.set_offset_high_bits((handler_address >> 16) as u16);
        self.0.set_offset(code_segment_offset);
        self.0.set_zero(0);
        self.0.set_options(options);
    }
}

#[bitfield(bytes = 8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RawIdtEntry {
    /// Lower 16 bits of handler function address
    pub offset_low_bits: u16,

    /// Kernel code segment offset (usually 0x08)
    pub offset: u16,

    /// Must always be 0
    pub zero: u8,

    pub options: IdtEntryOptions,

    /// Higher 16 bits of handler function address
    pub offset_high_bits: u16,
}

#[bitfield(bits = 8)]
#[derive(Specifier, Clone, Copy, PartialEq, Eq)]
pub struct IdtEntryOptions {
    /// 0xE for 32-bit Interrupt Gate
    pub gate_type: IdtGateType,
    
    /// Must be 0 for interrupt and trap gates
    pub is_storage_segment: bool,
    
    /// Ring 0 (Kernel) or Ring 3 (User Space)
    pub privilege_level: PrivilegeRing,
    
    /// Must be 1 for the IDT entry to be active
    pub is_present: bool,
}

#[derive(Specifier, Clone, Copy, PartialEq, Eq)]
#[bits = 4]
pub enum IdtGateType {
    // TaskGate = 0x5,
    // InterruptGate16 = 0x6,
    // TrapGate16 = 0x7,
    InterruptGate32 = 0xE, // We use this
    // TrapGate32 = 0xF,
}

#[repr(C, packed)]
pub struct IdtPointer {
    pub limit: u16,
    pub base: u32,
}