use pic8259_simple::ChainedPics;    // Represents a primary/secondary PIC layout
use spin;
use x86_64::structures::idt::{InterruptDescriptorTable, ExceptionStackFrame};

use ps_2_scancodes::{PS2ScancodeReader, PS2_PORT_ADDR, ScanCodeSet, ControlKey};

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
        idt.divide_by_zero.set_handler_fn(divide_by_zero_handler);
        idt.debug.set_handler_fn(debug_handler);
        idt.non_maskable_interrupt.set_handler_fn(non_maskable_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.overflow.set_handler_fn(overflow_handler);
        idt.bound_range_exceeded.set_handler_fn(bound_range_exceeded_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        idt.device_not_available.set_handler_fn(device_not_available_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        // idt.invalid_tss.set_handler_fn(invalid_TSS_handler); TODO: FIX THIS
        // idt.segment_not_present.set_handler_fn(segment_not_present_handler); TODO: FIX THIS
        // idt.stack_segment_fault.set_handler_fn(stack_segment_fault_handler); TODO: FIX THIS
        // idt.general_protection_fault.set_handler_fn(general_protection_fault_handler); TODO: FIX THIS
        // idt.page_fault.set_handler_fn(page_fault_handler); TODO: FIX THIS
        idt.x87_floating_point.set_handler_fn(x87_floating_point_handler);
        // idt.alignment_check.set_handler_fn(alignment_check_handler); TODO: FIX THIS
        idt.machine_check.set_handler_fn(machine_check_handler);
        idt.simd_floating_point.set_handler_fn(simd_floating_point_handler);
        idt.virtualization.set_handler_fn(virtualization_handler);
        //idt.security_exception.set_handler_fn(security_exception_handler); TODO: FIX THIS
        // TODO: Triple Fault
        idt[usize::from(TIMER_INTERRUPT_ID)]
            .set_handler_fn(timer_interrupt_handler); // Add the timer interrupt
        idt[usize::from(KEYBOARD_INTERRUPT_ID)]
            .set_handler_fn(keyboard_interrupt_handler);  // Add the keyboard interrupt


        idt
    };
}


lazy_static! {
    static ref SCANCODE_READER: spin::Mutex<PS2ScancodeReader> = spin::Mutex::new(PS2ScancodeReader::new(ScanCodeSet::SET1));
}

/// Initialize the Interrupt Descriptor Table for handling exceptions
pub fn init_idt() {
    IDT.load();
}

/// Exception handler for divide by zero
extern "x86-interrupt" fn divide_by_zero_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("EXCEPTION: DIVIDE BY ZERO\n{:#?}", stack_frame);
}

/// Exception handler for debug exceptions
extern "x86-interrupt" fn debug_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("EXCEPTION: DEBUG\n{:#?}", stack_frame);
}

/// Exception handler for non-maskable exceptions
extern "x86-interrupt" fn non_maskable_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("EXCEPTION: NON-MASKABLE\n{:#?}", stack_frame);
}

/// Add an exception handler for breakpoints
extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

/// Exception handler for overflow exceptions
extern "x86-interrupt" fn overflow_handler(stack_frame: &mut ExceptionStackFrame){
    println!("EXCEPTION: OVERFLOW\n{:#?}", stack_frame);
}

/// Bound Range Exceeded exception handler
extern "x86-interrupt" fn bound_range_exceeded_handler(stack_frame: &mut ExceptionStackFrame){
    println!("EXCEPTION: BOUND RANGE EXCEEDED\n{:#?}", stack_frame);
}

/// Invalid Opcode handler
extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: &mut ExceptionStackFrame){
    println!("EXCEPTION: INVALID OPCODE\n{:#?}", stack_frame);
}

/// Device not available handler
extern "x86-interrupt" fn device_not_available_handler(stack_frame: &mut ExceptionStackFrame){
    println!("EXCEPTION: DEVICE NOT AVAILABLE\n{:#?}", stack_frame);
}

/// Add an exception handler for double faults, this prevents triple faults
extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut ExceptionStackFrame, _error_code: u64){
    println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    hlt_loop();
}

/// Invalid TSS handler
extern "x86-interrupt" fn invalid_TSS_handler(stack_frame: &mut ExceptionStackFrame){
    println!("EXCEPTION: INVALID TSS\n{:#?}", stack_frame);
}

/// Segment not present handler
extern "x86-interrupt" fn segment_not_present_handler(stack_frame: &mut ExceptionStackFrame){
    println!("EXCEPTION: SEGMENT NOT PRESENT\n{:#?}", stack_frame);
}

/// Stack Segment Fault handler
extern "x86-interrupt" fn stack_segment_fault_handler(stack_frame: &mut ExceptionStackFrame){
    println!("EXCEPTION: STACK SEGMENT FAULT\n{:#?}", stack_frame);
}

/// General Protection Fault Handler
extern "x86-interrupt" fn general_protection_fault_handler(stack_frame: &mut ExceptionStackFrame){
    println!("EXCEPTION: GENERAL PROTECTION FAULT\n{:#?}", stack_frame);
}

/// Page Fault Handler
extern "x86-interrupt" fn page_fault_handler(stack_frame: &mut ExceptionStackFrame){
    println!("EXCEPTION: PAGE FAULT\n{:#?}", stack_frame);
}

/// x87 Floating Point Handler
extern "x86-interrupt" fn x87_floating_point_handler(stack_frame: &mut ExceptionStackFrame){
    println!("EXCEPTION: x87 FLOATING POINT\n{:#?}", stack_frame);
}

/// Alignment Check handler
extern "x86-interrupt" fn alignment_check_handler(stack_frame: &mut ExceptionStackFrame){
    println!("EXCEPTION: ALIGNMENT CHECK\n{:#?}", stack_frame);
}

/// Machine Check Handler
extern "x86-interrupt" fn machine_check_handler(stack_frame: &mut ExceptionStackFrame){
    println!("EXCEPTION: MACHINE CHECK\n{:#?}", stack_frame);
}

/// simd Floating Point Handler
extern "x86-interrupt" fn simd_floating_point_handler(stack_frame: &mut ExceptionStackFrame){
    println!("EXCEPTION: simd FLOATING POINT\n{:#?}", stack_frame);
}

/// virtualization Handler
extern "x86-interrupt" fn virtualization_handler(stack_frame: &mut ExceptionStackFrame){
    println!("EXCEPTION: VIRTUALIZATION\n{:#?}", stack_frame);
}

/// Security Exception Handler
extern "x86-interrupt" fn security_exception_handler(stack_frame: &mut ExceptionStackFrame){
    println!("EXCEPTION: SECURITY EXCEPTION\n{:#?}", stack_frame);
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
    
    use x86_64::instructions::port::Port;


    let port = Port::new(PS2_PORT_ADDR);
    let scancode: u8 = unsafe { port.read() };  // Read the data port of the PS/2 controller
    
    let key_code = SCANCODE_READER.lock().match_scancode(scancode);

    // POSSIBLE TODO:  I think we need to keep track of things like capslock and numlock here?

    if let Some(key) = key_code.key {
        print!("{}", key);
    }else if let Some(control) = key_code.control_key{
        match control {
            ControlKey::Escape => print!("Escape"),
            ControlKey::Backspace => print!("Backspace"),
            ControlKey::Tab => print!("Tab"),
            ControlKey::Enter => print!("Enter"),
            ControlKey::LeftCtrl => print!("LeftCtrl"),
            ControlKey::LeftShift => print!("LeftShift"),
            ControlKey::RightShift => print!("RightShift"),
            ControlKey::LeftAlt => print!("LeftAlt"),
            ControlKey::CapsLock => print!("CapsLock"),
            ControlKey::F1 => print!("F1"),
            ControlKey::F2 => print!("F2"),
            ControlKey::F3 => print!("F3"),
            ControlKey::F4 => print!("F4"),
            ControlKey::F5 => print!("F5"),
            ControlKey::F6 => print!("F6"),
            ControlKey::F7 => print!("F7"),
            ControlKey::F8 => print!("F8"),
            ControlKey::F9 => print!("F9"),
            ControlKey::F10 => print!("F10"),
            ControlKey::NumberLock => print!("NumberLock"),
            ControlKey::ScrollLock => print!("ScrollLock"),
            ControlKey::F11 => print!("F11"),
            ControlKey::F12 => print!("F12"),
            ControlKey::MultimediaPrevTrack => print!("Prev Track"),
            ControlKey::MultimediaNextTrack => print!("Next Track"),
            ControlKey::RightCtrl => print!("RightCtrl"),
            ControlKey::MultimediaMute => print!("MultimediaMute"),
            ControlKey::MultimediaCalculator => print!("MultimediaCalculator"),
            ControlKey::MultimediaPlay  => print!("MultimediaPlay"), 
            ControlKey::MultimediaStop  => print!("MultimediaStop"),
            ControlKey::MultimediaVolumeDown => print!("MultimediaVolumeDown"),
            ControlKey::MultimediaVolumeUp  => print!("MultimediaVolumeUp"),
            ControlKey::MultimediaWWWHome  => print!("MultimediaWWWHome"),
            ControlKey::RightAlt  => print!("RightAlt"),
            ControlKey::Home  => print!("Home"),
            ControlKey::CursorUp => print!("CursorUp"),
            ControlKey::PageUp  => print!("PageUp"),
            ControlKey::CursorLeft  => print!("CursorLeft"),
            ControlKey::CursorRight  => print!("CursorRight"),
            ControlKey::End  => print!("End"),
            ControlKey::CursorDown  => print!("CursorDown"),
            ControlKey::PageDown  => print!("PageDown"),
            ControlKey::Insert  => print!("Insert"),
            ControlKey::Delete  => print!("Delete"),
            ControlKey::LeftGUI  => print!("LeftGUI"),
            ControlKey::RightGUI  => print!("RightGUI"),
            ControlKey::Apps  => print!("Apps"),
            ControlKey::ACPIPower  => print!("ACPIPower"),
            ControlKey::ACPISleep  => print!("ACPISleep"), 
            ControlKey::ACPIWake  => print!("ACPIWake"),
            ControlKey::MultimediaWWWSearch  => print!("MultimediaWWWSearch"),
            ControlKey::MultimediaWWWFavourites  => print!("MultimediaWWWFavourites"),
            ControlKey::MultimediaWWWRefresh  => print!("MultimediaWWWRefresh"),
            ControlKey::MultimediaWWWStop  => print!("MultimediaWWWStop"),
            ControlKey::MultimediaWWWForward  => print!("MultimediaWWWForward"),
            ControlKey::MultimediaWWWBack  => print!("MultimediaWWWBack"),
            ControlKey::MultimediaMyComputer  => print!("MultimediaMyComputer"),
            ControlKey::MultimediaEmail  => print!("MultimediaEmail"),
            ControlKey::MultimediaSelect  => print!("MultimediaSelect"),
            ControlKey::PrintScreen => print!("Print Screen")
        }
    }else{
        // TODO: Check double interrupt scancodes for keys (will need to store this scancode and wait for the next interrupt)
        // TODO: Also do we need to handle combined key presses here? ex. SHIFT+CTRL or SHIFT+A.
    }

    unsafe { PICS.lock().notify_end_of_interrupt(KEYBOARD_INTERRUPT_ID) }
}