use x86::io::{inb, outb};

pub fn io_wait() {
    unsafe { outb(0x80, 0) }
}

const PIC1_CMD: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC1_OFFSET: u8 = 32;

const PIC2_CMD: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;
const PIC2_OFFSET: u8 = 40;

const PIC_EOI: u8 = 0x20;

pub fn init_pics() {
    unsafe {
        // Start initialization
        outb(PIC1_CMD, 0x11);
        io_wait();
        outb(PIC2_CMD, 0x11);
        io_wait();

        // Remap offsets    
        outb(PIC1_DATA, PIC1_OFFSET);
        io_wait();
        outb(PIC2_DATA, PIC2_OFFSET);
        io_wait();

        outb(PIC1_DATA, 0x04); // Tell Master there is a slave PIC at IRQ2
        io_wait();
        outb(PIC1_DATA, 0x02); // Tell slave it's a slave
        io_wait();

        // x86 mode
        outb(PIC1_DATA, 0x01);
        io_wait();
        outb(PIC2_DATA, 0x01);
        io_wait();

        // Disabling everything on both PICs
        outb(PIC1_DATA, 0xFF); 
        outb(PIC2_DATA, 0xFF);

        enable_irq(Irq::Keyboard);
    }
}

// SAFETY: pics must be initialized
pub unsafe fn enable_irq(irq: Irq) {
    let port;
    let mut relative_id = irq as u8;

    if irq.is_from_slave_pic() {
        port = PIC2_DATA;
        relative_id -= 8; // so it matches with the PIC2's internal checklist
    } else {
        port = PIC1_DATA;
    }

    let mut mask = unsafe { inb(port) };
    mask &= !(1 << relative_id);

    unsafe { outb(port, mask) };
}

pub fn send_end_of_interrupt(irq: Irq) {
    // If the interrupt came from the Slave (interrupts 40-47), we thank the Slave
    if irq.is_from_slave_pic() {
        unsafe { outb(PIC2_CMD, PIC_EOI); }
    }

    // We always thank the Master since the slave is plugged into it
    unsafe { outb(PIC1_CMD, PIC_EOI); }
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Irq {
    Keyboard = 1
}

impl Irq {
    pub const fn is_from_slave_pic(self) -> bool {
        self as u8 >= 8
    }
}