use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::{print, println, gdt, shell};
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin::Mutex;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;
pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

static COMMAND_BUFFER: Mutex<[u8; 80]> = Mutex::new([0; 80]);
static BUFFER_PTR: Mutex<usize> = Mutex::new(0);

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe { idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX); }
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}
pub fn init_idt() { IDT.load(); }
extern "x86-interrupt" fn double_fault_handler(sf: InterruptStackFrame, _err: u64) -> ! { panic!("DOUBLE FAULT\n{:#?}", sf); }
extern "x86-interrupt" fn keyboard_interrupt_handler(_sf: InterruptStackFrame) {
    use x86_64::instructions::port::Port;
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore));
    }
    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(ev)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(ev) {
            match key {
                DecodedKey::Unicode(c) => match c {
                    '\n' => {
                        println!("");
                        let mut ptr = BUFFER_PTR.lock();
                        shell::execute(&COMMAND_BUFFER.lock()[0..*ptr]);
                        *ptr = 0;
                        print!("rash> ");
                    }
                    '\x08' => {
                        let mut ptr = BUFFER_PTR.lock();
                        if *ptr > 0 { *ptr -= 1; print!("{}", c); }
                    }
                    _ => {
                        let mut ptr = BUFFER_PTR.lock();
                        if *ptr < 79 { print!("{}", c); COMMAND_BUFFER.lock()[*ptr] = c as u8; *ptr += 1; }
                    }
                },
                _ => {}
            }
        }
    }
    unsafe { PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8()); }
}
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex { Keyboard = PIC_1_OFFSET + 1 }
impl InterruptIndex {
    fn as_u8(self) -> u8 { self as u8 }
    fn as_usize(self) -> usize { self as usize }
}