// Define usage of the unstable feature for implementing a panic
#![feature(panic_handler)]

// Disable the standard library cause theres no OS for it to rely on
#![no_std]

// Tell the compiler we dont want to use the normal entry point, unless we are testing
#![cfg_attr(not(test), no_main)]

// Also ignore warnings that occur when we compile the test build, due to having no OS.
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[macro_use]
extern crate ostridge;

use core::panic::PanicInfo;


/// This defines the starting function for the executable 
/// No mangle says not to change the name of the function, so that the C runtime can find "_start"
/// Extern C says to use the C runtime to call this function
#[no_mangle]
#[cfg(not(test))]   // only compile when the test flag is not set
pub extern "C" fn _start() -> ! {
    
    // Print to VGA buffer
    println!("Hello World{}", "!");
    
    // Print to serial port
    serial_println!("Hello Host{}", "!");

    // Show use of panic, we dont need this here
    // panic!("At The Disco");


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

