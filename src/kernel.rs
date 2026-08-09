//! A minimal 32-bit x86 kernel

#![no_main]
#![no_std]

use kernel::{
	println,
	vmalloc,
	vmalloc_size,
	vfree,
};

/// First rust function called by the link.ld file
#[unsafe(no_mangle)]
pub extern "C" fn _entrypoint(magic_number: u32, multiboot_info_ptr: u32) -> ! {
	kernel::init(magic_number, multiboot_info_ptr);
	test_allocation_integrity();
	// test_memory_leaks();
	// test_double_free_trap();
	// test_use_after_free();
	kernel::shell_loop();
}

pub fn test_ring0_panic() {
	println!("TEST: VIP breaking the rules. Reading 0xDEADBEEF from Ring 0...");

	// We create a raw pointer to an unmapped address
	let bad_ptr = 0x80000000 as *const u32;

	unsafe {
		// The CPU will attempt to read this memory.
		// It will fail the Page Table lookup, trigger Exception 14.
		// The error code will show U/S = 0 (Supervisor).
		// Your handler MUST print a massive FATAL error and halt.
		let _value = bad_ptr.read_volatile();
	}

	core::hint::black_box(bad_ptr);
}

pub fn test_allocation_integrity() {
	println!("Running Test 1: Allocation Integrity...");

	// 1. Ask for 8192 bytes (Exactly 2 pages)
	let requested_size = 8192;
	let ptr = vmalloc(requested_size);
	assert!(!ptr.is_null(), "vmalloc returned null!");

	// 2. Test the hidden size header
	let tracked_size = vmalloc_size(ptr);
	assert!(
		tracked_size == requested_size,
		"Size mismatch! Expected {}, got {}",
		requested_size,
		tracked_size
	);

	// 3. Write data into the first page
	unsafe {
		core::ptr::write_volatile(ptr, 42);
	}

	// 4. Write data deep into the second page (crossing the 4096 boundary)
	unsafe {
		core::ptr::write_volatile(ptr.add(5000), 84);
	}

	// 5. Read it back to prove the hardware MMU didn't lose our data
	let val1 = unsafe { core::ptr::read_volatile(ptr) };
	let val2 = unsafe { core::ptr::read_volatile(ptr.add(5000)) };

	assert!(val1 == 42 && val2 == 84, "Memory corruption detected!");

	// 6. Clean up
	vfree(ptr);
	println!("Test 1 Passed: Virtual boundaries crossed successfully!");
}

pub fn test_memory_leaks() {
	println!("Running Test 2: Stress / Leak Test...");

	// Loop enough times that we would DEFINITELY run out of physical RAM
	// if kfree wasn't working. (e.g., 50,000 pages = ~200MB)
	for _ in 0..50_000 {
		let ptr = vmalloc(4096);

		// Write a quick byte to force the CPU to touch the physical RAM
		unsafe {
			core::ptr::write_volatile(ptr, 0xff);
		}

		// Free it immediately
		vfree(ptr);
	}

	println!("Test 2 Passed: Survived 50,000 allocations without running out of RAM!");
}

pub fn test_double_free_trap() {
	println!("Running Test 3: Intentional Double Free...");

	let ptr = vmalloc(4096);

	println!("Freeing once...");
	vfree(ptr);

	println!("Freeing twice... The kernel should panic now!");
	vfree(ptr); // <--- KERNEL MUST PANIC HERE

	println!("FAIL: If you see this, your double-free check is broken.");
}

pub fn test_use_after_free() {
	println!("Running Test 4: Use-After-Free Page Fault...");

	let ptr = vmalloc(4096);

	// Write some data
	unsafe {
		core::ptr::write_volatile(ptr, 99);
	}

	// Demolish the room and flush the TLB cache
	vfree(ptr);

	println!("Attempting to read from a demolished room...");

	// Try to read the old data. Because the TLB is flushed and the PTE is 0,
	// the CPU's hardware should instantly throw Exception 14.
	let _ghost_data = unsafe { core::ptr::read_volatile(ptr) }; // <--- PAGE FAULT HERE

	println!("FAIL: If you see this, you forgot to flush the TLB with invlpg!");
}
