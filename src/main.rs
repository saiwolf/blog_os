#![no_std] // don't link the Rust standard library
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[macro_use]
extern crate blog_os;
extern crate x86_64;
extern crate pc_keyboard;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);
#[cfg(not(test))]
fn kernel_main(boot_info: &'static BootInfo) -> ! {    

    // Import the PICS functions for our interrupts
    use blog_os::interrupts::PICS;    

    // set up the IDT first, otherwise we would enter a boot loop instead of
    // invoking our page fault handler
    blog_os::gdt::init();
    blog_os::interrupts::init_idt();
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    use blog_os::memory;
    use x86_64::{structures::paging::Page, VirtAddr};

    // initialize a mapper
    let mut mapper = unsafe { memory::init(boot_info.physical_memory_offset) };
    let mut frame_allocator = memory::init_frame_allocator(&boot_info.memory_map);

    // map a previously unmapped page
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

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