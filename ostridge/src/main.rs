// Define usage of the unstable feature for implementing a panic
#![feature(panic_handler)]

// Disable the standard library cause theres no OS for it to rely on
#![no_std]

// Tell the compiler we dont want to use the normal entry point
#![no_main]

use core::panic::PanicInfo;

/// This defines the starting function for the executable 
/// No mangle says not to change the name of the function, so that the C runtime can find "_start"
/// Extern C says to use the C runtime to call this function
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

/// Define function to call on panic
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}