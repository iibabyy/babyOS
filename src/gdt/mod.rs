use core::arch::asm;

use crate::{gdt::entry::{GdtEntry, GdtEntryFlags, GdtPointer, GtdEntryAccessFlags}, shared::PrivilegeRing};

pub mod entry;

static mut GDT: [GdtEntry; 3] = [GdtEntry::zeroed(); 3];
static mut GDT_PTR: GdtPointer = GdtPointer { limit: 0, base: 0 };

#[expect(static_mut_refs)]
pub fn init_gdt() {
    unsafe {
        // Entry 0: Null Descriptor (Already zeroed)
        GDT[1] = kernel_gdt(true); // Entry 1: Kernel Code
        GDT[2] = kernel_gdt(false); // Entry 2: Kernel Data

        // Give the CPU the memory address and size of our GDT array
        GDT_PTR.limit = (core::mem::size_of::<[GdtEntry; 3]>() - 1) as u16;
        GDT_PTR.base = GDT.as_ptr() as u32;

        // Segments offset relatively to GDT_PTR
        let code_segment_offset = core::mem::size_of::<GdtEntry>() as u16;
        let data_segment_offset = 2 * core::mem::size_of::<GdtEntry>() as u16;

        load_gdt(
            &GDT_PTR as *const _,
            code_segment_offset,
            data_segment_offset
        );
    }
}

unsafe fn load_gdt(ptr: *const GdtPointer, code_segment_offset: u16, data_segment_offset: u16) {
    unsafe {
        asm!(
            // Load the GDT pointer
            "lgdt [{ptr:e}]", // ':e' tells asm! to use a 32-bit register

            // Load data segments
            "mov ds, {data:x}", // ':x' tells asm! to use a 16-bit register
            "mov es, {data:x}",
            "mov fs, {data:x}",           
            "mov gs, {data:x}",           
            "mov ss, {data:x}",

        /*
            The CPU forbids updating the code segment (cs register) directly with 'mov'
            To do so, we use retf
            This instruction is used to:
             - jump to a location/label (by updating eip, the register that holds the next line that the CPU will execute) 
             - and to load a new code segment (by updating cs)
            Since we don't want to change the location, we use a mock label
        */

            // Load code segment
            "push {code:e}", // the code segment to load
            "lea {tmp}, [2f]", // load the memory address of label 2 into tmp
            "push {tmp}",
            "retf", 
            "2:", // the label where retf will jump

            // args
            ptr = in(reg) ptr,
            data = in(reg) data_segment_offset,
            code = in(reg) code_segment_offset as u32,
            tmp = out(reg) _
        );
    }
}

fn kernel_gdt(executable: bool) -> GdtEntry {
    let access_flags = GtdEntryAccessFlags::new()
        .with_is_present(true)
        .with_privilege_level(PrivilegeRing::Kernel)
        .with_is_code_or_data(true)
        .with_is_executable(executable)
        .with_read_write(true);

    let flags = GdtEntryFlags::new()
        .with_is_32_bit_operation_size(true);
    
    GdtEntry::new(
        0,
        0xFFFFFFFF,
        access_flags,
        flags
    )
}
