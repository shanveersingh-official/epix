use core::fmt;
use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color { Black = 0, LightGreen = 10, White = 15 }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);
impl ColorCode { fn new(fg: Color, bg: Color) -> ColorCode { ColorCode((bg as u8) << 4 | (fg as u8)) } }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar { ascii_character: u8, color_code: ColorCode }

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
struct Buffer { chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT] }

pub struct Writer { column_position: usize, color_code: ColorCode, buffer: &'static mut Buffer }
impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            0x08 => { if self.column_position > 0 { self.column_position -= 1; self.write_at_pos(b' ', self.column_position); } }
            byte => {
                if self.column_position >= BUFFER_WIDTH { self.new_line(); }
                self.write_at_pos(byte, self.column_position);
                self.column_position += 1;
            }
        }
    }
    fn write_at_pos(&mut self, byte: u8, col: usize) {
        let row = BUFFER_HEIGHT - 1;
        self.buffer.chars[row][col].write(ScreenChar { ascii_character: byte, color_code: self.color_code });
    }
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar { ascii_character: b' ', color_code: self.color_code };
        for col in 0..BUFFER_WIDTH { self.buffer.chars[row][col].write(blank); }
    }
}
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() { self.write_byte(byte); }
        Ok(())
    }
}
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::LightGreen, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}
pub fn clear_screen() {
    let mut writer = WRITER.lock();
    for row in 0..BUFFER_HEIGHT { writer.clear_row(row); }
}
#[macro_export]
macro_rules! print { ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*))); }
#[macro_export]
macro_rules! println { () => ($crate::print!("\n")); ($($arg:tt)*) => ($crate::print!("{}\\n", format_args!($($arg)*))); }
pub fn _print(args: fmt::Arguments) { use core::fmt::Write; WRITER.lock().write_fmt(args).unwrap(); }