use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;
use lazy_static::lazy_static;
#[cfg(test)]
use crate::{serial_print, serial_println};

// set up IDT in lazy_static as we need IDT to have 'static lifetime
// we cannot create it inside a function as any function will drop if off
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

// load the IDT after configuring any needed exception handlers
pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    serial_print!("test_breakpoint_exception...");
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
    serial_println!("[ok]");
}
