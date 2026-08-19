//! A minimal 32-bit x86 kernel

#![no_main]
#![no_std]

use kernel::alloc::boxed::Box;
use kernel::alloc::rc::Rc;
use kernel::alloc::string::String;
use kernel::alloc::vec::Vec;
use kernel::println;

/// First rust function called by the link.ld file
#[unsafe(no_mangle)]
pub extern "C" fn _entrypoint(magic_number: u32, multiboot_info_ptr: u32) -> ! {
	kernel::init(magic_number, multiboot_info_ptr);

	test_heap_allocation();

	kernel::shell_loop();
}

pub fn test_heap_allocation() {
	// We assume you have a working print! or serial_println! macro.
	// Replace `crate::println!` with whatever you use in babyOS.

	{
		// 1. Test basic allocation
		let heap_value = Box::new(42);
		assert_eq!(*heap_value, 42);
	}
	crate::println!("[-] Box allocation successful.");

	{
		// 2. Test large allocations and reallocations (Growing)
		let mut vec = Vec::with_capacity(500);
		for i in 0..500 {
			vec.push(i);
		}
		assert_eq!(vec.len(), 500);
		assert_eq!(vec[499], 499);
	}
	crate::println!("[-] Vec reallocation and growth successful.");

	{
		// 3. Test Strings (dynamic UTF-8 arrays)
		let mut s = String::from("Kernel ");
		s.push_str("Global Allocator");
		assert_eq!(s, "Kernel Global Allocator");
	}
	crate::println!("[-] String allocation successful.");

	{
		// 4. Test Reference Counting (allocating metadata on the heap)
		let rc = Rc::new(100);
		let rc_clone = Rc::clone(&rc);
		assert_eq!(Rc::strong_count(&rc), 2);
		drop(rc_clone);
		assert_eq!(Rc::strong_count(&rc), 1);
	}
	crate::println!("[-] Rc allocation successful.");

	{
		// 5. Test Memory Churn (Deallocation)
		// If your `dealloc` is broken, this loop will exhaust a small kernel heap quickly.
		for i in 0..10_000 {
			let x = Box::new(i);
			assert_eq!(*x, i);
		}
	}
	crate::println!("[-] Memory churn (allocation/deallocation) successful.");

	crate::println!("[+] All heap allocation tests passed!");
}
