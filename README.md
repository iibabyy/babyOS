# babyOS

A small `#![no_std]` x86 (i386) operating system kernel written in Rust. It boots
via GRUB (multiboot), runs under QEMU, and currently brings up VGA text output,
serial (COM1) output, and an interrupt descriptor table with a breakpoint handler.

## Requirements

- Docker (everything runs inside the provided dev container)

## Usage

```sh
make up      # start the dev container (warm caches)
make sh      # drop into a shell in the container
make clean   # remove build artifacts and the Docker container/volumes
```

Once inside the container (`make sh`), build and boot the kernel with:

```sh
make iso     # build the bootable ISO
make run     # build and boot in QEMU
make release # build with optimizations
```

## Layout

- `src/kernel.rs` — kernel entry point (`_entrypoint`) and test runner
- `src/lib.rs` — `baby_lib`: panic handler and module wiring
- `src/io/` — VGA text buffer and serial driver
- `src/interrupts/` — IDT and exception handlers
- `tools/build/` — boot stubs, linker script, target spec, GRUB config
