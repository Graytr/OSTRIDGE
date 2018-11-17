#![feature(abi_x86_interrupt)]

// Disable the standard library cause theres no OS for it to rely on
#![no_std]

// Use the standard libray in tests
#[cfg(test)]
extern crate std;

// Use this for tests too
#[cfg(test)]
extern crate array_init;

extern crate uart_16550;

extern crate bootloader_precompiled;
extern crate volatile;
#[macro_use]    // Import lazy_static! macro
extern crate lazy_static;
extern crate spin;  // Spinlocks
extern crate x86_64;    // For I/O port writing
extern crate pic8259_simple; // For A simple programmable interrupt controller (hardware exceptions)

#[macro_use]
pub mod vga_buffer;

pub mod serial;

pub mod gdt;

pub mod interrupts;

pub mod ps_2_scancodes;

pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}

pub fn hlt_loop() -> !{
    loop{
        // Halt the cpu rather than looping endlessly
        x86_64::instructions::hlt();
    }
}