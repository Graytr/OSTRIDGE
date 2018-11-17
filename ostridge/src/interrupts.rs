use pic8259_simple::ChainedPics;    // Represents a primary/secondary PIC layout
use spin;
use x86_64::structures::idt::{InterruptDescriptorTable, ExceptionStackFrame};


use gdt;
use hlt_loop;

// Set the exception numbers to range from 32 - 47, because 0-32 are taken by our exceptions
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

// Set the ids for interrupts
pub const TIMER_INTERRUPT_ID: u8 = PIC_1_OFFSET;
pub const KEYBOARD_INTERRUPT_ID: u8 = PIC_1_OFFSET + 1;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });


lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        // TODO: divide by zero
        // TODO: debug
        // TODO: Non-maskable
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        // TODO: Overflow
        // TODO: Bound Range Exceeded
        // TODO: Invalid Opcode
        // TODO: Device Not Available
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        // TODO: Invalid TSS
        // TODO: Segment Not Present
        // TODO: Stack-Segment Fault
        // TODO: General Protection Fault
        // TODO: Page Fault
        // TODO: x87 Floating Point Exception
        // TODO: Alignment Check
        // TODO: Machine Check
        // TODO: SIMD Floating Point Exception
        // TODO: Virtualization Exception
        // TODO: Security Exception
        // TODO: Triple Fault
        idt[usize::from(TIMER_INTERRUPT_ID)]
            .set_handler_fn(timer_interrupt_handler); // Add the timer interrupt
        idt[usize::from(KEYBOARD_INTERRUPT_ID)]
            .set_handler_fn(keyboard_interrupt_handler);  // Add the keyboard interrupt


        idt
    };
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
    hlt_loop();
}

/// Interrupt handler for timer
extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: &mut ExceptionStackFrame)
{
    print!(".");
    
    // Signal the PIC that we are done with this interrupt
    unsafe { PICS.lock().notify_end_of_interrupt(TIMER_INTERRUPT_ID) }
}

/// Interrupt handler for keyboard input
extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: &mut ExceptionStackFrame)
{
    use ps_2_scancodes;
    use x86_64::instructions::port::Port;


    let port = Port::new(ps_2_scancodes::PS2_PORT_ADDR);
    let scancode: u8 = unsafe { port.read() };  // Read the data port of the PS/2 controller


    let mut scancode_reader = ps_2_scancodes::PS2ScancodeReader::new(ps_2_scancodes::ScanCodeSet::SET1);

    let key_code = scancode_reader.match_scancode(scancode);

    if let Some(key) = key_code.key {
        print!("{}", key);
    }else if let Some(_control) = key_code.control_key{
        print!("A control key was pressed.")
    }

    unsafe { PICS.lock().notify_end_of_interrupt(KEYBOARD_INTERRUPT_ID) }
}