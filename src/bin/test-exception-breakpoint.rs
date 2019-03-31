#![no_std] // don't link the Rust standard library
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]
#![feature(abi_x86_interrupt)]

// add the library as dependency (same crate name as executable)
#[macro_use]
extern crate blog_os;
extern crate x86_64;
#[macro_use]
extern crate lazy_static;

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use core::panic::PanicInfo;
use blog_os::exit_qemu;
use core::sync::atomic::{AtomicUsize, Ordering};

static BREAKPOINT_HANDLER_CALLED: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init_test_idt();

    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();

    match BREAKPOINT_HANDLER_CALLED.load(Ordering::SeqCst) {
        1 => serial_println!("ok"),
        0 => {
            serial_println!("failed");
            serial_println!("Breakpoint handler was not called.");
        }
        other => {
            serial_println!("failed");
            serial_println!("Breakpoint handler was called {} times", other);
        }
    }

    unsafe { exit_qemu(); }
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(
    _stack_frame: &mut InterruptStackFrame)
{
    BREAKPOINT_HANDLER_CALLED.fetch_add(1, Ordering::SeqCst);
}