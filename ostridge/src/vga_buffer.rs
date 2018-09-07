//! vga_buffer
//!
//! The vga_buffer is used to handle printing ascii characters to the screen
//!

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
    /// Create a new colour code with the desired foreground and background colours
    ///
    /// # Arguments
    ///
    /// * `foreground` - A Colour enum representing one of the predefined colours as an unsigned int, used 
    ///                  to set the colour of a printed character
    /// * `background` - A Colour enum representing one of the predefined colours as an unsinged int, used
    ///                  to set the background colour of a printed character

    fn new(foreground: Colour, background: Colour) -> ColourCode {
        ColourCode((background as u8) << 4 | (foreground as u8))
    } 
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]  // Make sure the struct is represented exactly like a C struct, keeps ordering
/// A character that can be displayed on the screen
struct ScreenChar {
    /// A character is represented by an 8-bit unsigned integer, for ascii-codes
    ascii_character: u8,
    /// What's life without a bit of colour?
    colour_code: ColourCode,
}

// The standard size of the buffer
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

/// Represents a 2D array for characters to be written to the screen
struct Buffer {
    // Volatile allows us to specify that there are side effects to reads and writes, and should not be optimized
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// A writer that writes to the last line and shifts up when full or newline is encountered
pub struct Writer {
    /// Store the current position in the last row
    column_position: usize, 
    /// The colour code for the foreground and background
    colour_code: ColourCode,  
    /// Reference to the buffer that lasts the entire runtime
    buffer: &'static mut Buffer,    
}

impl Writer {
    /// Write a new byte to the buffer
    ///
    /// # Arguments
    ///
    /// * `byte` - An unsigned integer byte representing the character to write
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

    /// Write a new string to the buffer
    ///
    /// # Arguments
    ///
    /// * `s` - A string to write to the screen
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
    ///
    /// # Arguments
    ///
    /// * `row` - A usize value representing the row in the buffer to clear
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            colour_code: self.colour_code,
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    /// Change the current colour of the writer
    ///
    /// # Arguments
    /// * `new_colour_code` - A ColourCode representing the colour the Writer should use to create characters
    fn change_colours(&mut self, new_colour_code: ColourCode) {
        self.colour_code = new_colour_code;
    }

    /// Get the current colour the writer is using to write characters
    fn get_colour(&mut self) -> ColourCode {
        self.colour_code
    }
}

// Implementation of formatted writing for our Writer
// This should allow us to use Rust formatting macros like write! and writeln!
impl fmt::Write for Writer {
    /// Definition of a formatted write for the writer
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

/// A static writer so we don't need to keep dealing with a Writer instance
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        colour_code: ColourCode::new(Colour::Yellow, Colour::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },  // 0xb8000 is the memory address of the VGA buffer
    });
}

/// Macro for printing using our vga_buffer
///
/// # Examples
/// 
/// This example will print "This is a test!" to the static (screen) buffer
/// ```rust, no_run
/// # #[macro_use] extern crate ostridge;
/// # fn main() {
/// print!("This is a test!");
/// # }
/// ```
///
/// This example will print "This is also a test!" to the static (screen) buffer
/// ```rust, no_run
/// # #[macro_use] extern crate ostridge;
/// # fn main() {
/// print!("This is {}", "also a test!");
/// # }
/// ```
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::print(format_args!($($arg)*)));
}

/// Macro for printing with newlines
///
/// # Examples
///
/// This example will print "This is a test!" to the static (screen) buffer, including a newline
/// ```rust, no_run
/// # #[macro_use] extern crate ostridge;
/// # fn main() {
/// print!("This is a test!");
/// # }
/// ```
///
/// This example will print "This is also a test!" to the static (screen) buffer, including a newline
/// ```rust, no_run
/// # #[macro_use] extern crate ostridge;
/// # fn main() {
/// print!("This is {}", "also a test!");
/// # }
/// ```
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

/// Used for printing using the Static Writer
///
/// # Arguments
///
/// * `args` - A formatted arguments object containing the inputs to print
pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

/// Macro for printing in colour
///
/// # Examples
///
/// This example will print "This is colourful a test!" to the static (screen) buffer in a red text with blue background
/// ```rust, no_run
/// # #[macro_use] extern crate ostridge;
/// # fn main() {
/// use ostridge::vga_buffer::Colour;
///
/// let foreground = Colour::Red;
/// let background = Colour::Blue;
/// colour_print!(foreground, background, "This is colourful a test!");
/// # }
/// ```
#[macro_export]
macro_rules! colour_print {
    ($foreground:tt, $background:tt, $($arg:tt)*) => ($crate::vga_buffer::colour_print(format_args!($($arg)*), $foreground, $background));
}


/// Function to allow printing in colour other than the default
///
/// # Arguments
///
/// * 'args' - The formatted string arguments to print to the screen
/// * `foreground_colour` - The Colour enum to use as the colour of the characters
/// * `background_colour` - The Colour enum to use as the colour for the background
pub fn colour_print(args: fmt::Arguments, foreground_colour: Colour, background_colour: Colour) {
    let old_colour_code = WRITER.lock().get_colour();
    let new_colour_code = ColourCode::new(foreground_colour, background_colour);
    WRITER.lock().change_colours(new_colour_code);
    print(args);
    WRITER.lock().change_colours(old_colour_code);
}

#[cfg(test)]
mod test {
    use super::*;

    fn construct_writer() -> Writer {
        use std::boxed::Box;

        let buffer = construct_buffer();
        
        Writer {
            column_position: 0,
            colour_code: ColourCode::new(Colour::Blue, Colour::Magenta),
            buffer: Box::leak(Box::new(buffer)),
        }
    }

    fn construct_buffer() -> Buffer {
        use array_init::array_init;

        Buffer {
            chars: array_init(|_| array_init(|_| Volatile::new(empty_char()))),
        }
    }

    fn empty_char() -> ScreenChar {
        ScreenChar {
            ascii_character: b' ',
            colour_code: ColourCode::new(Colour::Green, Colour::Brown),
        }
    }

    #[test]
    fn write_byte() {
        let mut writer = construct_writer();
        writer.write_byte(b'X');
        writer.write_byte(b'Y');

        for (i, row) in writer.buffer.chars.iter().enumerate() {
            for (j, screen_char) in row.iter().enumerate() {
                let screen_char = screen_char.read();

                if i == BUFFER_HEIGHT - 1 && j == 0 {
                    assert_eq!(screen_char.ascii_character, b'X');
                    assert_eq!(screen_char.colour_code, writer.colour_code);
                } else if i == BUFFER_HEIGHT -1 && j == 1 {
                    assert_eq!(screen_char.ascii_character, b'Y');
                    assert_eq!(screen_char.colour_code, writer.colour_code);
                } else {
                    assert_eq!(screen_char, empty_char());
                }
            }
        }
    }

    #[test]
    fn write_formatted() {
        use core::fmt::Write;

        let mut writer = construct_writer();
        writeln!(&mut writer, "a").unwrap();
        writeln!(&mut writer, "b{}", "c").unwrap();

        for (i, row) in writer.buffer.chars.iter().enumerate() {
            for (j, screen_char) in row.iter().enumerate() {
                let screen_char = screen_char.read();
                if i == BUFFER_HEIGHT - 3 && j == 0 {
                    assert_eq!(screen_char.ascii_character, b'a');
                    assert_eq!(screen_char.colour_code, writer.colour_code);
                } else if i == BUFFER_HEIGHT - 2 && j == 0 {
                    assert_eq!(screen_char.ascii_character, b'b');
                    assert_eq!(screen_char.colour_code, writer.colour_code);
                } else if i == BUFFER_HEIGHT - 2 && j == 1 {
                    assert_eq!(screen_char.ascii_character, b'c');
                    assert_eq!(screen_char.colour_code, writer.colour_code);
                } else if i >= BUFFER_HEIGHT - 2 {
                    assert_eq!(screen_char.ascii_character, b' ');
                    assert_eq!(screen_char.colour_code, writer.colour_code);
                } else {
                    assert_eq!(screen_char, empty_char());
                }
            }
        }
    }

    #[test]
    fn change_colours() {
        use core::fmt::Write;

        let mut writer = construct_writer();
        let old_colour_code = writer.colour_code;
        let new_colour_code = ColourCode::new(Colour::Red, Colour::Pink);

        writeln!(&mut writer, "a").unwrap();

        // Change colours of the writer
        writer.colour_code = new_colour_code;
        assert_eq!(writer.colour_code, new_colour_code);

        writeln!(&mut writer, "b").unwrap();

        for (i, row) in writer.buffer.chars.iter().enumerate() {
            for (j, screen_char) in row.iter().enumerate() {
                let screen_char = screen_char.read();
                if i == BUFFER_HEIGHT - 3 && j == 0 {
                    assert_eq!(screen_char.ascii_character, b'a');
                    assert_eq!(screen_char.colour_code, old_colour_code);
                } else if i == BUFFER_HEIGHT - 2 && j == 0 {
                    assert_eq!(screen_char.ascii_character, b'b');
                    assert_eq!(screen_char.colour_code, new_colour_code);
                } else if i >= BUFFER_HEIGHT - 2 {
                    assert_eq!(screen_char.ascii_character, b' ');
                } else {
                    assert_eq!(screen_char, empty_char());
                }
            }
        }
    }
    
    #[test]
    fn top_line_shift() {
        use core::fmt::Write;

        let mut writer = construct_writer();

        for _ in 0..BUFFER_HEIGHT {
            writeln!(&mut writer, "Test");
        }

        for (i, row) in writer.buffer.chars.iter().enumerate() {
            for (_j, screen_char) in row.iter().enumerate() {
                let screen_char = screen_char.read();
                if i == BUFFER_HEIGHT - 1 {
                    assert_eq!(screen_char.ascii_character, b' ');
                    assert_eq!(screen_char.colour_code, writer.colour_code);
                } 
            }
        }
    }
    
    #[test]
    fn non_ascii_chars() {
        use core::fmt::Write;

        let mut writer = construct_writer();

        writeln!(&mut writer, "ö");
        for (i, row) in writer.buffer.chars.iter().enumerate() {
            for (j, screen_char) in row.iter().enumerate() {
                let screen_char = screen_char.read();
                if i == BUFFER_HEIGHT - 2 && j == 0 {
                    assert_eq!(screen_char.ascii_character, 0xfe);
                    assert_eq!(screen_char.colour_code, writer.colour_code);
                } 
            }
        }

    }
}