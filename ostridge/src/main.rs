// Define usage of the unstable feature for implementing a panic
#![feature(panic_handler)]

// Disable the standard library cause theres no OS for it to rely on
#![no_std]

// Tell the compiler we dont want to use the normal entry point
#![no_main]

extern crate bootloader_precompiled;
extern crate volatile;
#[macro_use]    // Import lazy_static! macro
extern crate lazy_static;
extern crate spin;  // Spinlocks


use core::panic::PanicInfo;

#[macro_use]
mod vga_buffer;

/// This defines the starting function for the executable 
/// No mangle says not to change the name of the function, so that the C runtime can find "_start"
/// Extern C says to use the C runtime to call this function
#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    println!("Hello Workd{}", "!");
    panic!("Some panic message");

    loop {}
}

/// Define function to call on panic
#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}