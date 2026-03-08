use crate::{println, vga_buffer};
use x86_64::instructions::port::Port;

pub fn execute(cmd: &[u8]) {
    match cmd {
        b"help" => println!("EPIX COMMANDS: help, clear, reboot, cpu"),
        b"clear" => vga_buffer::clear_screen(),
        b"reboot" => unsafe { Port::new(0x64).write(0xFEu8); },
        b"cpu" => println!("CPU: Intel Atom (NP-100 Legacy Mode)"),
        b"" => {} 
        _ => println!("Rash: Unknown Command"),
    }
}
