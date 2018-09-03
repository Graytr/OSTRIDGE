// Define usage of the unstable feature for implementing a panic
#![feature(panic_handler)]

// Disable the standard library cause theres no OS for it to rely on
#![no_std]

// Tell the compiler we dont want to use the normal entry point
#![no_main]

extern crate bootloader_precompiled;

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

/// This defines the starting function for the executable 
/// No mangle says not to change the name of the function, so that the C runtime can find "_start"
/// Extern C says to use the C runtime to call this function
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;    // The memory address of the VGA buffer

    // Write each byte of the Hello World String to the VGA buffer
    for (i, &byte) in HELLO.iter().enumerate(){
        unsafe {
            // This writes the character to the buffer
            *vga_buffer.offset(i as isize * 2) = byte;
            // This sets the colour of the character
            *vga_buffer.offset(i as isize *2 + 1) = 0xb;
        }
    }

    loop {}
}

/// Define function to call on panic
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}