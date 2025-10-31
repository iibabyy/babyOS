section .multiboot
align 4
    dd 0x1BADB002 ; MAGIC
    dd (1 << 0) | (1 << 1) ; FLAGS (ALIGN | MEMINFO)
    dd -(0x1BADB002 + ((1 << 0) | (1 << 1))) ; CHECKSUM

section .bss
align 16
stack_bottom:
    resb 16384
stack_top:

section .text
global _start
extern _entrypoint

_start:
    mov esp, stack_top
    call _entrypoint

.hang:
    cli
    hlt
    jmp .hang