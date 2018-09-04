// Define usage of the unstable feature for implementing a panic
#![feature(panic_handler)]

// Disable the standard library cause theres no OS for it to rely on
#![no_std]

// Tell the compiler we dont want to use the normal entry point, unless we are testing
#![cfg_attr(not(test), no_main)]

// Also ignore warnings that occur when we compile the test build, due to having no OS.
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

// Use the standard libray in tests
#[cfg(test)]
extern crate std;

// Use this for tests too
#[cfg(test)]
extern crate array_init;

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
#[cfg(not(test))]   // only compile when the test flag is not set
pub extern "C" fn _start() -> ! {
    
    println!("Hello World{}", "!");
    panic!("At The Disco");

    loop {}
}

/// Define function to call on panic
#[cfg(not(test))]   // only compile when the test flag is not set
#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}