// Define usage of the unstable feature for implementing a panic
#![feature(panic_handler)]

#![feature(abi_x86_interrupt)]

// Disable the standard library cause theres no OS for it to rely on
#![no_std]

// Tell the compiler we dont want to use the normal entry point, unless we are testing
#![cfg_attr(not(test), no_main)]

// Also ignore warnings that occur when we compile the test build, due to having no OS.
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[macro_use]
extern crate ostridge;

#[macro_use]
extern crate lazy_static;

extern crate x86_64;

use core::panic::PanicInfo;
use ostridge::vga_buffer::Colour;

use x86_64::structures::idt::{InterruptDescriptorTable, ExceptionStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(ostridge::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}


/// This defines the starting function for the executable 
/// No mangle says not to change the name of the function, so that the C runtime can find "_start"
/// Extern C says to use the C runtime to call this function
#[no_mangle]
#[cfg(not(test))]   // only compile when the test flag is not set
pub extern "C" fn _start() -> ! {

    ostridge::gdt::init();
    init_idt();

    let mut foreground = Colour::Green;
    let background = Colour::Black;

    colour_print!(foreground, background, "Welcome to OSTRIDGE!\n\n");
    
    foreground = Colour::Brown;

    colour_print!(foreground, background, "Keep your head in the sand.\n\n");
    println!("READY");

    // Print to serial port
    serial_println!("Hello Host{}", "!");

    // invoke a breakpoint exception
    x86_64::instructions::int3();

    println!("It did not crash!");
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
/// Initialize the Interrupt Descriptor Table for handling exceptions
pub fn init_idt() {
    IDT.load();
}

/// Add an exception handler for breakpoints
extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

/// Add an exception handler for double faults, this prevents triple faults
extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut ExceptionStackFrame, _error_code: u64){
    println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    loop{}
}
