ENTRY(_start)
SECTIONS {

    . = 0x00100000;

    kernel_start = .;

    .multiboot : { KEEP(*(.multiboot)) }

    .text : { *(.text*) }

    .rodata : { *(.rodata*) }

    .data : { *(.data*) }

    .bss : { *(COMMON) *(.bss*) }

    kernel_end = .;
}