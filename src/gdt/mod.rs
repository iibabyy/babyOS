use core::arch::asm;

use crate::{
    gdt::entry::{GdtEntry, GdtEntryFlags, GdtPointer, GtdEntryAccessFlags}, shared::PrivilegeRing,
};

pub mod dump;
pub mod entry;

const GDT_BASE: usize = 0x00000800;
const GDT_LEN: usize = 7;
const GDT_SIZE: usize = core::mem::size_of::<[GdtEntry; GDT_LEN]>();

const GDT_ADDRESS: *mut GdtEntry = GDT_BASE as *mut GdtEntry;
// we place the pointer at the end of the gdt array
const GDT_PTR_ADDRESS: *mut GdtPointer = (GDT_BASE + GDT_SIZE) as *mut GdtPointer;

pub fn init_gdt() {
        let gdt = unsafe { core::slice::from_raw_parts_mut(GDT_ADDRESS, GDT_LEN) };

		gdt[0] = GdtEntry::zeroed();						// Must be Null
		gdt[1] = kernel_gdt(MemoryType::Code);	// Kernel Code
		gdt[2] = kernel_gdt(MemoryType::Data);	// Kernel Data
		gdt[3] = kernel_gdt(MemoryType::Data);	// Kernel Stack
		gdt[4] = user_gdt(MemoryType::Code);		// User Code
		gdt[5] = user_gdt(MemoryType::Data);		// User Data
		gdt[6] = user_gdt(MemoryType::Data);		// User Stack

		let gdt_ptr = unsafe { &mut *GDT_PTR_ADDRESS };
		gdt_ptr.base = GDT_BASE as u32;
		gdt_ptr.limit = GDT_SIZE as u16 - 1;

		// offset of kernel code/data GDT entries, relative to ptr.base (GDT_BASE)
	    let kcode_offset = core::mem::size_of::<GdtEntry>() as u32;
        let kdata_offset = 2 * core::mem::size_of::<GdtEntry>() as u16;
		let kstack_offset = 3 * core::mem::size_of::<GdtEntry>() as u16;

		unsafe {
			load_gdt(
				GDT_PTR_ADDRESS,
				kcode_offset,
				kdata_offset,
				kstack_offset,
        	)
		};
}

// SAFETY: ptr must point to a valid GdtPointer, and other args must be correct
unsafe fn load_gdt(
	ptr: *const GdtPointer,

	// offset of kernel code/data GDT entries, relative to ptr.base (GDT_BASE)
	kcode_offset: u32,
	kdata_offset: u16,
	kstack_offset: u16
) {
    unsafe {
        asm!(
            // Load the GDT pointer
			// ':e' tells asm! to use a 32-bit register
            "lgdt [{ptr}]",

            // Load data segments
			// ':x' tells asm! to use a 16-bit register
            "mov ds, {data:x}", // data segment
            "mov ss, {stack:x}", // stack segment
            "mov es, {data:x}",
            "mov fs, {data:x}",
            "mov gs, {data:x}",

            // The CPU forbids updating the code segment (cs register) directly with 'mov'
            // To do so, we use retf
            // This instruction is used to:
            //  - jump to a location/label (by updating eip, the register that holds the next line that the CPU will execute)
            //  - and load a new code segment (by updating cs) at the same time
            // Since we don't want to change the location, we use a mock label
            "push {code:e}", // the code segment to load
            "lea {tmp}, [2f]", // load the memory address of label 2 into tmp
            "push {tmp}",
            "retf",
            "2:", // the label where retf will jump

            // args
            ptr = in(reg) ptr,
            data = in(reg) kdata_offset,
            stack = in(reg) kstack_offset,
            code = in(reg) kcode_offset,
            tmp = out(reg) _
        );
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MemoryType {
    Code,
    Data
}

fn kernel_gdt(mem_type: MemoryType) -> GdtEntry {
    let access_flags = GtdEntryAccessFlags::new()
        .with_is_present(true)
        .with_privilege_level(PrivilegeRing::Kernel)
        .with_is_code_or_data(true)
        .with_is_executable(mem_type == MemoryType::Code)
        .with_read_write(true);

    let flags = GdtEntryFlags::new().with_is_32_bit_operation_size(true);

    GdtEntry::new(0, 0xFFFFFFFF, access_flags, flags)
}

fn user_gdt(mem_type: MemoryType) -> GdtEntry {
    let access_flags = GtdEntryAccessFlags::new()
        .with_is_present(true)
        .with_privilege_level(PrivilegeRing::UserSpace)
        .with_is_code_or_data(true)
        .with_is_executable(mem_type == MemoryType::Code)
        .with_read_write(true);

    let flags = GdtEntryFlags::new().with_is_32_bit_operation_size(true);

    GdtEntry::new(0, 0xFFFFFFFF, access_flags, flags)
}
