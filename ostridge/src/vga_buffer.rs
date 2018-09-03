use volatile::Volatile;
use core::fmt;
use spin::Mutex;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)] // This ensures the values are stored as 8-bit unsigned integers
/// Represent all possible Colours for the VGA Buffer
pub enum Colour {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A structure to hold a colour code 
struct ColourCode(u8);

impl ColourCode {
    // Create a new colour code with the desired foreground and background colours
    fn new(foreground: Colour, background: Colour) -> ColourCode {
        ColourCode((background as u8) << 4 | (foreground as u8))
    } 
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]  // Make sure the struct is represented exactly like a C struct, keeps ordering
/// A character that can be displayed on the screen
struct ScreenChar {
    ascii_character: u8,
    colour_code: ColourCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

struct Buffer {
    // Volatile allows us to specify that there are side effects to reads and writes, and should not be optimized
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// A writer that writes to the last line and shifts up when full or newline is encountered
pub struct Writer {
    column_position: usize, // Stire the current position in the last row
    colour_code: ColourCode,  // The colour code for the foreground and background
    buffer: &'static mut Buffer,    // Reference to the buffer that lasts the entire runtime
}

impl Writer {
    /// Write a new byte to the buffer
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let colour_code = self.colour_code;

                // Using a volatile write to the memory address for the buffer
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    colour_code: colour_code,
                });

                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // Printable ASCII or newline
                0x20...0x7e | b'\n' => self.write_byte(byte),
                // Not printable ASCII
                _ => self.write_byte(0xfe),
            }
        }
    }

    /// Create a new line in the buffer
    fn new_line(&mut self) { 
        // Iterate over all characters and move each one row up
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    /// Replace all chars in a row with blanks
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            colour_code: self.colour_code,
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

/// Implementation of formatted writing for our Writer
/// This should allow us to use Rust formatting macros like write! and writeln!
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

/// A static writer so we don't need too keep dealing with a Writer instance
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        colour_code: ColourCode::new(Colour::Yellow, Colour::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

// Macro for printing using our vga_buffer
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::print(format_args!($($arg)*)));
}

// Macro for printing with newlines
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

/// Used for printing using the Static Writer
pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}