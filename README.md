# babyOS

A small `#![no_std]` x86 (i386) kernel written in Rust. It boots
via GRUB (multiboot), and runs under QEMU.

## Requirements

- Docker
- Qemu (`qemu-system-i386` command)

## Quick start

```sh
make # Builds the iso, then runs Qemu
```

## Layout

- `src/kernel.rs` — kernel entry point (`_entrypoint`) and test runner
- `src/lib.rs` — `baby_lib`: panic handler and module wiring
- `src/memory/` — physical/virtual memory initialization and management
- `src/io/` — VGA text buffer and serial driver
- `src/interrupts/` — IDT and exception handlers
- `tools/build/` — boot stubs, linker script, target spec, GRUB config
