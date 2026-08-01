use modular_bitfield::Specifier;


#[derive(Specifier, Clone, Copy, PartialEq, Eq)]
#[bits = 2]
pub enum PrivilegeRing {
    Kernel = 0,
    UserSpace = 3,
}
