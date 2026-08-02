use modular_bitfield::prelude::*;

use crate::shared::PrivilegeRing;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct GdtEntry(RawGdtEntry);

impl GdtEntry {
    pub const fn zeroed() -> Self {
        Self(RawGdtEntry::new())
    }

    pub fn new(
        base_address: u32,
        limit: u32,
        access_flags: GtdEntryAccessFlags,
        flags: GdtEntryFlags,
    ) -> Self {
        let raw_entry = RawGdtEntry::new()
            .with_access_flags(access_flags)
            .with_flags(flags);

        Self(raw_entry)
            .with_base_address(base_address)
            .with_limit(limit)
    }

    fn with_base_address(mut self, base_address: u32) -> Self {
        let base_low_bits = base_address as u16; // bits 0-15 of the base address
        let base_middle_bits = (base_address >> 16) as u8; // bits 16-23 of the base address
        let base_high_bits = (base_address >> 24) as u8; // bits 24-31 of base address

        self.0.set_base_low_bits(base_low_bits);
        self.0.set_base_middle_bits(base_middle_bits);
        self.0.set_base_high_bits(base_high_bits);
        self
    }

    fn with_limit(mut self, mut limit: u32) -> Self {
        {
            // The maximum limit without granularity is 1MB (0xFFFFF)
            // (with granularity, the CPU uses 4Kb units instead of 1b)
            let granularity = if limit > 0xFFFFF {
                limit /= 4096;
                true
            } else {
                false
            };

            let new_flags = self.0.flags().with_granularity(granularity);

            self.0.set_flags(new_flags);
        }

        let limit_low_bits = limit as u16; // bits 0-15 of the limit value
        let limit_high_bits = (limit >> 16) as u8; // bits 16-20 of the limit value

        self.0.set_limit_low_bits(limit_low_bits);
        self.0.set_limit_high_bits(limit_high_bits);
        self
    }
}

#[bitfield(bytes = 8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RawGdtEntry {
    /// bits 0-15 of the limit value
    pub limit_low_bits: u16,

    /// bits 0-15 of the base address
    pub base_low_bits: u16,

    /// bits 16-23 of the base address
    pub base_middle_bits: u8,

    pub access_flags: GtdEntryAccessFlags,

    /// bits 16-20 of the limit value
    pub limit_high_bits: B4,

    pub flags: GdtEntryFlags,

    /// bits 24-31 of base address
    pub base_high_bits: u8,
}

#[bitfield(bits = 8)]
#[derive(Specifier, Clone, Copy, PartialEq, Eq)]
pub struct GtdEntryAccessFlags {
    /// Set to 1 by the CPU hardware when the segment is accessed.
    pub accessed: bool,

    /// - For Data: 1 allows write access (read is always allowed).
    /// - For Code: 1 allows read access (write is never allowed).
    pub read_write: bool,

    /// - For Data: 0 means segment grows up, 1 means it grows down (stack).
    /// - For Code: 1 means code can be executed from lower privilege levels.
    pub direction: bool,

    /// 1 if the CPU can run code here.
    pub is_executable: bool,

    /// 0 for system segments (like TSS), 1 for code or data segments.
    pub is_code_or_data: bool,

    /// Sets the Ring level (0 to 3) required to access this segment.
    pub privilege_level: PrivilegeRing,

    /// Must be 1 for the segment to be valid.
    pub is_present: bool,
}

#[bitfield(bits = 4)]
#[derive(Specifier, Clone, Copy, PartialEq, Eq)]
pub struct GdtEntryFlags {
    /// Free bit for software/OS use; completely ignored by the CPU hardware.
    pub available: bool,

    /// 1 sets the segment to 64-bit code mode. If this is set, the D/B bit must be 0.
    pub long_mode: bool,

    ///   - For Code: 0 means 16-bit execution, 1 means 32-bit execution.
    ///   - For Data/Stack: 0 means 16-bit stack pointer (SP), 1 means 32-bit stack pointer.
    pub is_32_bit_operation_size: bool,

    ///   - 0 : Segment limit is in byte steps (maximum size of 1 MiB).
    ///   - 1 : Segment limit is in 4 KiB page steps (maximum size of 4 GiB).
    pub granularity: bool,
}

#[repr(C, packed)]
pub struct GdtPointer {
    pub limit: u16,
    pub base: u32,
}
