
pub const PS2_PORT_ADDR: u16 = 0x60; // The port address for the PS/2 controller

#[derive(PartialEq)]
pub enum ScanCodeSet {
    SET1 = 0,
    SET2 = 1,
    SET3 = 2,
}

pub struct PS2ScancodeReader {
    scan_code_set: ScanCodeSet,
    double_code: bool,
}

pub enum ControlKey {
    Escape = 0,
    Backspace = 1,
    Tab = 2,
    Enter = 3,
    LeftCtrl = 4,
    LeftShift = 5,
    RightShift = 6,
    LeftAlt = 7,
    CapsLock = 8,
    F1 = 9,
    F2 = 10,
    F3 = 11,
    F4 = 12,
    F5 = 13,
    F6 = 14,
    F7 = 15,
    F8 = 16,
    F9 = 17,
    F10 = 18,
    NumberLock = 19,
    ScrollLock = 20,
    F11 = 21,
    F12 = 22,
    MultimediaPrevTrack = 23,
    MultimediaNextTrack = 24,
    RightCtrl = 25,
    MultimediaMute = 26,
    MultimediaCalculator = 27,
    MultimediaPlay = 28, 
    MultimediaStop = 29,
    MultimediaVolumeDown = 30,
    MultimediaVolumeUp = 31,
    MultimediaWWWHome = 32,
    RightAlt = 33,
    Home = 34,
    CursorUp = 35,
    PageUp = 36,
    CursorLeft = 37,
    CursorRight = 38,
    End = 39,
    CursorDown = 40,
    PageDown = 41,
    Insert = 42,
    Delete = 43,
    LeftGUI = 44,
    RightGUI = 45,
    Apps = 46,
    ACPIPower = 47,
    ACPISleep = 48, 
    ACPIWake = 49,
    MultimediaWWWSearch = 50,
    MultimediaWWWFavourites = 51,
    MultimediaWWWRefresh = 52,
    MultimediaWWWStop = 53,
    MultimediaWWWForward = 54,
    MultimediaWWWBack = 55,
    MultimediaMyComputer = 56,
    MultimediaEmail = 57,
    MultimediaSelect = 58,
    PrintScreen = 59

}

pub struct PS2Key{
    pub key: Option<char>,
    pub control_key: Option<ControlKey>,
    pub pressed: bool,   // True on pressed, false on released
    pub keypad: bool
}

impl PS2ScancodeReader {

    pub fn new(scan_code_set: ScanCodeSet) -> PS2ScancodeReader {
        PS2ScancodeReader{scan_code_set: scan_code_set, double_code: false}
    }

    pub fn set_scancode_set(&mut self, set: ScanCodeSet){
        self.scan_code_set = set;
    }

    pub fn match_scancode(&mut self, code: u8) -> PS2Key {

        match self.scan_code_set {
            ScanCodeSet::SET1 => self.match_set1_scancode(code),
            _ => PS2Key {key: None, control_key: None, pressed: false, keypad: false}
        }

    }

    fn create_key(&mut self, key: char, pressed: bool, keypad: bool) -> PS2Key {
        self.double_code = false;
        PS2Key {key: Some(key), control_key: None, pressed: pressed, keypad: keypad}
    }

    fn create_control_key(&mut self, key: ControlKey, pressed: bool, keypad: bool) -> PS2Key {
        self.double_code = false;
        PS2Key {key: None, control_key: Some(key), pressed: pressed, keypad: keypad}
    }

    fn match_set1_scancode(&mut self, code: u8) -> PS2Key {
        let key = match code {
            // KEY PRESSES
            0x01 => self.create_control_key(ControlKey::Escape, true, false),
            0x02 => self.create_key('1', true, false),
            0x03 => self.create_key('2', true, false),
            0x04 => self.create_key('3', true, false),
            0x05 => self.create_key('4', true, false),
            0x06 => self.create_key('5', true, false),
            0x07 => self.create_key('6', true, false),
            0x08 => self.create_key('7', true, false),
            0x09 => self.create_key('8', true, false),
            0x0a => self.create_key('9', true, false),
            0x0b => self.create_key('0', true, false),
            0x0c => self.create_key('-', true, false),
            0x0d => self.create_key('=', true, false),
            0x0e => self.create_control_key(ControlKey::Backspace, true, false),
            0x0f => self.create_control_key(ControlKey::Tab, true, false),
            0x10 => {
                if self.double_code {
                    self.double_code = false;
                    self.create_control_key(ControlKey::MultimediaPrevTrack, true, false)
                }else{
                    self.create_key('q', true, false)
                }
            },
            0x11 => self.create_key('w', true, false),
            0x12 => self.create_key('e', true, false),
            0x13 => self.create_key('r', true, false),
            0x14 => self.create_key('t', true, false),
            0x15 => self.create_key('y', true, false),
            0x16 => self.create_key('u', true, false),
            0x17 => self.create_key('i', true, false),
            0x18 => self.create_key('o', true, false),
            0x19 => {
                if self.double_code {
                    self.double_code = false;
                    self.create_control_key(ControlKey::MultimediaNextTrack, true, false)
                }else{
                    self.create_key('p', true, false)
                }
            },
            0x1a => self.create_key('[', true, false),
            0x1b => self.create_key(']', true, false),
            0x1c => {
                if self.double_code {
                    self.double_code = false;
                    self.create_control_key(ControlKey::Enter, true, true)
                }else{
                    self.create_control_key(ControlKey::Enter, true, false)
                }
            },
            0x1d => {
                if self.double_code {
                    self.double_code = false;
                    self.create_control_key(ControlKey::RightCtrl, true, false)
                }else{
                    self.create_control_key(ControlKey::LeftCtrl, true, false)
                }
            },
            0x1e => self.create_key('a', true, false),
            0x1f => self.create_key('s', true, false),
            0x20 => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaMute, true, false)
                }else{
                    self.create_key('d', true, false)
                }
            },
            0x21 => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaCalculator, true, false)
                }else{
                    self.create_key('f', true, false)
                }
            },
            0x22 => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaPlay, true, false)
                }else{
                    self.create_key('g', true, false)
                }
            },
            0x23 => self.create_key('h', true, false),
            0x24 => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaStop, true, false)
                }else{
                    self.create_key('j', true, false)
                }
            },
            0x25 => self.create_key('k', true, false),
            0x26 => self.create_key('l', true, false),
            0x27 => self.create_key(';', true, false),
            0x28 => self.create_key('\'', true, false),
            0x29 => self.create_key('`', true, false),
            0x2a => {
                if self.double_code {
                    self.create_control_key(ControlKey::PrintScreen, true, false)
                }else{
                    self.create_control_key(ControlKey::LeftShift, true, false)
                }
            },
            0x2b => self.create_key('\\', true, false),
            0x2c => self.create_key('z', true, false),
            0x2d => self.create_key('x', true, false),
            0x2e => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaVolumeDown, true, false)
                }else{
                    self.create_key('c', true, false)
                }
            },
            0x2f => self.create_key('v', true, false),
            0x30 => { 
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaVolumeUp, true, false)
                } else {
                    self.create_key('b', true, false)
                }
            },
            0x31 => self.create_key('n', true, false),
            0x32 => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaWWWHome, true, false)
                } else {
                    self.create_key('m', true, false)       
                }
            },
            0x33 => self.create_key(',', true, false),
            0x34 => self.create_key('.', true, false),
            0x35 => {
                if self.double_code {
                    self.create_key('/', true, true)
                } else {
                    self.create_key('/', true, false)
                }
            },
            0x36 => self.create_control_key(ControlKey::RightShift, true, false),
            0x37 => self.create_key('*', true, true),
            0x38 => {
                if self.double_code {
                    self.create_control_key(ControlKey::RightAlt, true, false)
                } else {
                    self.create_control_key(ControlKey::LeftAlt, true, false)
                }    
            },
            0x39 => self.create_key(' ', true, false),
            0x3a => self.create_control_key(ControlKey::CapsLock, true, false),
            0x3b => self.create_control_key(ControlKey::F1, true, false),
            0x3c => self.create_control_key(ControlKey::F2, true, false),
            0x3d => self.create_control_key(ControlKey::F3, true, false),
            0x3e => self.create_control_key(ControlKey::F4, true, false),
            0x3f => self.create_control_key(ControlKey::F5, true, false),
            0x40 => self.create_control_key(ControlKey::F6, true, false),
            0x41 => self.create_control_key(ControlKey::F7, true, false),
            0x42 => self.create_control_key(ControlKey::F8, true, false),
            0x43 => self.create_control_key(ControlKey::F9, true, false),
            0x44 => self.create_control_key(ControlKey::F10, true, false),
            0x45 => self.create_control_key(ControlKey::NumberLock, true, false),
            0x46 => self.create_control_key(ControlKey::ScrollLock, true, false),
            0x47 => {
                if self.double_code {
                    self.create_control_key(ControlKey::Home, true, false)
                } else {
                    self.create_key('7', true, true)
                }
            },
            0x48 => {
                if self.double_code {
                    self.create_control_key(ControlKey::CursorUp, true, false)
                } else {
                    self.create_key('8', true, true)
                }
            }, 
            0x49 => {
                if self.double_code {
                    self.create_control_key(ControlKey::PageUp, true, false)
                } else {
                    self.create_key('9', true, true)
                }
            },
            0x4a => self.create_key('-', true, true),
            0x4b => {
                if self.double_code {
                    self.create_control_key(ControlKey::CursorLeft, true, false)
                } else {
                    self.create_key('4', true, true)
                }
            },
            0x4c => self.create_key('5', true, true),
            0x4d => {
                if self.double_code {
                    self.create_control_key(ControlKey::CursorRight, true, false)
                } else {
                    self.create_key('6', true, true)
                }
            },
            0x4e => self.create_key('+', true, true),
            0x4f => {
                if self.double_code {
                    self.create_control_key(ControlKey::End, true, false)
                } else {
                    self.create_key('1', true, true)
                }
            },
            0x50 => {
                if self.double_code {
                    self.create_control_key(ControlKey::CursorDown, true, false)
                } else {
                    self.create_key('2', true, true)
                }
            },
            0x51 => {
                if self.double_code {
                    self.create_control_key(ControlKey::PageDown, true, false)
                } else {
                    self.create_key('3', true, true)
                }   
            },
            0x52 => {
                if self.double_code {
                    self.create_control_key(ControlKey::Insert, true, false)
                } else {
                    self.create_key('0', true, true)
                }
            },
            0x53 => {
                if self.double_code {
                    self.create_control_key(ControlKey::Delete, true, false)
                } else {
                    self.create_key('.', true, true)
                }
            },

            // Break in code sequence
            0x57 =>self.create_control_key(ControlKey::F11, true, false),
            0x58 => self.create_control_key(ControlKey::F12, true, false),

            0x5B => {
                if self.double_code{
                    self.create_control_key(ControlKey::LeftGUI, true, false)
                } else {
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },
            0x5C => {
                if self.double_code{
                    self.create_control_key(ControlKey::RightGUI, true, false)
                } else {
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },
            0x5D => {
                if self.double_code {
                    self.create_control_key(ControlKey::Apps, true, false)
                } else {
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },
            0x5E => {
                if self.double_code {
                    self.create_control_key(ControlKey::ACPIPower, true, false)
                } else {
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },
            0x5F => {
                if self.double_code {
                    self.create_control_key(ControlKey::ACPISleep, true, false)
                } else {
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },
            0x63 => {
                if self.double_code {
                    self.create_control_key(ControlKey::ACPIWake, true, false)
                } else {
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },
            0x65 => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaWWWSearch, true, false)
                } else {
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },
            0x66 => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaWWWFavourites, true, false)
                } else {
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },
            0x67 => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaWWWRefresh, true, false)
                } else {
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },
            0x68 => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaWWWStop, true, false)
                } else {
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },
            0x69 => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaWWWForward, true, false)
                } else {
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },
            0x6A => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaWWWBack, true, false)
                } else {
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },
            0x6B => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaMyComputer, true, false)
                } else {
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },
            0x6C => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaEmail, true, false)
                } else {
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },
            0x6D => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaSelect, true, false)
                } else {
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },

            // KEY RELEASES
            0x81 => self.create_control_key(ControlKey::Escape, false, false),
            0x82 => self.create_key('1', false, false),
            0x83 => self.create_key('2', false, false),
            0x84 => self.create_key('3', false, false),
            0x85 => self.create_key('4', false, false),
            0x86 => self.create_key('5', false, false),
            0x87 => self.create_key('6', false, false),
            0x88 => self.create_key('7', false, false),
            0x89 => self.create_key('8', false, false),
            0x8a => self.create_key('9', false, false),
            0x8b => self.create_key('0', false, false),
            0x8c => self.create_key('-', false, false),
            0x8d => self.create_key('=', false, false),
            0x8e => self.create_control_key(ControlKey::Backspace, false, false),
            0x8f => self.create_control_key(ControlKey::Tab, false, false),
            0x90 => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaPrevTrack, false, false)
                }else{
                    self.create_key('q', false, false)
                }
            },
            0x91 => self.create_key('w', false, false),
            0x92 => self.create_key('e', false, false),
            0x93 => self.create_key('r', false, false),
            0x94 => self.create_key('t', false, false),
            0x95 => self.create_key('y', false, false),
            0x96 => self.create_key('u', false, false),
            0x97 => self.create_key('i', false, false),
            0x98 => self.create_key('o', false, false),
            0x99 => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaNextTrack, false, false)
                }else{
                    self.create_key('p', false, false)
                }
            },
            0x9a => self.create_key('[', false, false),
            0x9b => self.create_key(']', false, false),
            0x9c => {
                if self.double_code {
                    self.create_control_key(ControlKey::Enter, false, true)
                }else{
                    self.create_control_key(ControlKey::Enter, false, false)
                }
            },
            0x9d => {
                if self.double_code {
                    self.create_control_key(ControlKey::RightCtrl, false, false)
                }else{
                    self.create_control_key(ControlKey::LeftCtrl, false, false)
                }
            },
            0x9e => self.create_key('a', false, false),
            0x9f => self.create_key('s', false, false),
            0xa0 => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaMute, false, false)
                }else{
                    self.create_key('d', false, false)
                }
            },
            0xa1 => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaCalculator, false, false)
                }else{
                    self.create_key('f', false, false)
                }
            },
            0xa2 => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaPlay, false, false)
                }else{
                    self.create_key('g', false, false)
                }
            },
            0xa3 => self.create_key('h', false, false),
            0xa4 => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaStop, false, false)
                }else{
                    self.create_key('j', false, false)
                }
            },
            0xa5 => self.create_key('k', false, false),
            0xa6 => self.create_key('l', false, false),
            0xa7 => self.create_key(';', false, false),
            0xa8 => self.create_key('\'',false, false),
            0xa9 => self.create_key('`', false, false),
            0xaa =>self.create_control_key(ControlKey::LeftShift, false, false),
            0xab => self.create_key('\\',false, false),
            0xac => self.create_key('z', false, false),
            0xad => self.create_key('x', false, false),
            0xae => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaVolumeDown, false, false)
                }else{
                    self.create_key('c', false, false)
                }
            },
            0xaf => self.create_key('v', false, false),
            0xb0 => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaVolumeUp, false, false)
                }else{
                    self.create_key('b', false, false)
                }
            },
            0xb1 => self.create_key('n', false, false),
            0xb2 => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaWWWHome, false, false)
                }else{
                    self.create_key('m', false, false)
                }
            },
            0xb3 => self.create_key(',', false, false),
            0xb4 => self.create_key('.', false, false),
            0xb5 => {
                if self.double_code {
                    self.create_key('/', false, true)
                }else{
                    self.create_key('/', false, false)
                }
            },
            0xb6 => self.create_control_key(ControlKey::RightShift, false,  false),
            0xb7 => {
                if self.double_code {
                    self.create_control_key(ControlKey::PrintScreen, false, false)
                }else{
                    self.create_key('*', false, true)
                }
            },
            0xb8 => {
                if self.double_code {
                    self.create_control_key(ControlKey::RightAlt, false, false)
                }else{
                    self.create_control_key(ControlKey::LeftAlt, false, false)
                }
            },
            0xb9 => self.create_key(' ', false, false),
            0xba => self.create_control_key(ControlKey::CapsLock, false, false),
            0xbb => self.create_control_key(ControlKey::F1, false, false),
            0xbc => self.create_control_key(ControlKey::F2, false, false),
            0xbd => self.create_control_key(ControlKey::F3, false, false),
            0xbe => self.create_control_key(ControlKey::F4, false, false),
            0xbf => self.create_control_key(ControlKey::F5, false, false),
            0xc0 => self.create_control_key(ControlKey::F6, false, false),
            0xc1 => self.create_control_key(ControlKey::F7, false, false),
            0xc2 => self.create_control_key(ControlKey::F8, false, false),
            0xc3 => self.create_control_key(ControlKey::F9, false, false),
            0xc4 => self.create_control_key(ControlKey::F10, false, false),
            0xc5 => self.create_control_key(ControlKey::NumberLock, false, false),
            0xc6 => self.create_control_key(ControlKey::ScrollLock, false, false),
            0xc7 => {
                if self.double_code {
                    self.create_control_key(ControlKey::Home, false, false)
                }else{
                    self.create_key('7', false, true)
                }
            },
            0xc8 => {
                if self.double_code {
                    self.create_control_key(ControlKey::CursorUp, false, false)
                }else{
                    self.create_key('8', false, true)
                }
            },
            0xc9 => {
                if self.double_code {
                    self.create_control_key(ControlKey::PageUp, false, false)
                }else{
                    self.create_key('9', false, true)
                }
            },
            0xca => self.create_key('-', false, true),
            0xcb => {
                if self.double_code {
                    self.create_control_key(ControlKey::CursorLeft, false, false)
                } else {
                    self.create_key('4', false, true)
                }
            },
            0xcc => self.create_key('5', false, true),
            0xcd => {
                if self.double_code {
                    self.create_control_key(ControlKey::CursorRight, false, false)
                }else{
                    self.create_key('6', false, true)
                }
            },
            0xce => self.create_key('+', false, true),
            0xcf => {
                if self.double_code {
                    self.create_control_key(ControlKey::End, false, false)
                }else{
                    self.create_key('1', false, true)
                }
            },
            0xd0 => {
                if self.double_code {
                    self.create_control_key(ControlKey::CursorDown, false, false)
                }else{
                    self.create_key('2', false, true)
                }
            },
            0xd1 => {
                if self.double_code {
                    self.create_control_key(ControlKey::PageDown, false, false)
                }else{
                    self.create_key('3', false, true)
                }
            },
            0xd2 => {
                if self.double_code {
                    self.create_control_key(ControlKey::Insert, false, false)
                }else{
                    self.create_key('0', false, true)
                }
            },
            0xd3 => {
                if self.double_code {
                    self.create_control_key(ControlKey::Delete, false, false)
                }else{
                    self.create_key('.', false, true)
                }
            },
            
            // Break in code sequence
            0xd7 =>self.create_control_key(ControlKey::F11, false, false),
            0xd8 => self.create_control_key(ControlKey::F12, false, false),

            0xdb => {
                if self.double_code {
                    self.create_control_key(ControlKey::LeftGUI, false, false)
                }else{
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },
            0xdc => {
                if self.double_code {
                    self.create_control_key(ControlKey::RightGUI, false, false)
                }else{
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },
            0xdd => {
                if self.double_code {
                    self.create_control_key(ControlKey::Apps, false, false)
                }else{
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },
            0xde => {
                if self.double_code {
                    self.create_control_key(ControlKey::ACPIPower, false, false)
                }else{
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },
            0xdf => {
                if self.double_code {
                    self.create_control_key(ControlKey::ACPISleep, false, false)
                }else{
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },
            0xe3 => {
                if self.double_code {
                    self.create_control_key(ControlKey::ACPIWake, false, false)
                }else{
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },
            0xe5 => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaWWWSearch, false, false)
                }else{
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },
            0xe6 => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaWWWFavourites, false, false)
                }else{
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },
            0xe7 => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaWWWRefresh, false, false)
                }else{
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },
            0xe8 => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaWWWStop, false, false)
                }else{
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },
            0xe9 => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaWWWForward, false, false)
                }else{
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },
            0xea => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaWWWBack, false, false)
                }else{
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },
            0xeb => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaMyComputer, false, false)
                }else{
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },
            0xec => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaEmail, false, false)
                }else{
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },
            0xed => {
                if self.double_code {
                    self.create_control_key(ControlKey::MultimediaSelect, false, false)
                }else{
                    PS2Key {key: None, control_key: None, pressed: false, keypad: false}
                }
            },


            
           // Double codes
           0xe0 => {
                self.double_code = true;
                print!("Double Code!");
                PS2Key {key: None, control_key: None, pressed: false, keypad: false}
           },
            _ => PS2Key {key: None, control_key: None, pressed: false, keypad: false}
        };

        key
    }

    


}
