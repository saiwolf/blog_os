#![no_std] // don't link the Rust standard library
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[macro_use]
extern crate blog_os;
extern crate x86_64;
extern crate pc_keyboard;

use core::panic::PanicInfo;

/// This function is the entry point, since the linker looks for a function
/// named `_start` by default.
#[cfg(not(test))]
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // import the Cr3 register functions
    use x86_64::registers::control::Cr3;

    // Import the `PageTable` type from the x86_64 crate
    use x86_64::structures::paging::PageTable;

    // Set `level_4_table_ptr` to the last virtual address
    // space address: 0xFFFF_FFFF_FFFF_F000 casted as a pointer
    // to the `PageTable` type. This allows us to bypass using
    // unsafe raw pointers.   
    let level_4_table_ptr = 0xffff_ffff_ffff_f000 as *const PageTable;

    // Set `level_4_table` using an unsafe reference to
    // `level_4_table_ptr`
    let level_4_table = unsafe {&*level_4_table_ptr};

    // `For` loop that will print out the first ten entries in
    // the page table by using .offset()
    for i in 0..10 {        
        println!("Entry {}: {:?}", i, level_4_table[i]);
    }

    // Import the PICS functions for our interrupts
    use blog_os::interrupts::PICS;    

    // set up the IDT first, otherwise we would enter a boot loop instead of
    // invoking our page fault handler
    blog_os::gdt::init();
    blog_os::interrupts::init_idt();
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    println!("It did not crash!");
    blog_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::hlt_loop();
}