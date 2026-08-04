use core::arch::asm;

use modular_bitfield::Specifier;

/// x86 privilege ring level
#[derive(Specifier, Clone, Copy, PartialEq, Eq)]
#[bits = 2]
pub enum PrivilegeRing {
    Kernel = 0,
    UserSpace = 3,
}

/// Write 16 bits to `port`
#[inline]
#[expect(unsafe_op_in_unsafe_fn)]
pub unsafe fn outb(port: u16, val: u8) {
    asm!(
        "out dx, al",
        in("al") val,
        in("dx") port,
        options(nostack, preserves_flags) // for compiler optimisations
    );
}

/// Read 8 bits from `port`
#[inline]
#[expect(unsafe_op_in_unsafe_fn)]
pub unsafe fn inb(port: u16) -> u8 {
    let ret: u8;
    asm!(
        "in al, dx",
        in("dx") port,
        out("al") ret,
        options(nostack, preserves_flags) // for compiler optimisations
    );
    ret
}

