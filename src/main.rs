#![no_std] // don't link the Rust standard library
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points, if not testing
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]
#![feature(extern_crate_item_prelude)] // use of extern prelude names introduced with `extern crate` items is unstable (see issue #54658)

#[cfg(test)]
extern crate array_init;
extern crate bootloader_precompiled;
#[macro_use]
extern crate lazy_static;
extern crate spin;
extern crate volatile;

/* For testing only */
#[cfg(test)]
extern crate std;

/* Import macros from our VGA module */
#[macro_use]
mod vga_buffer;

use core::panic::PanicInfo;

/// This function is called on panic.
#[cfg(not(test))] // only compile when the test flag is NOT set
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(not(test))]
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");    
    loop {}
}