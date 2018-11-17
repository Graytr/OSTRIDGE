
pub const PS2_PORT_ADDR: u16 = 0x60; // The port address for the PS/2 controller

#[derive(PartialEq)]
pub enum ScanCodeSet {
    SET1 = 0,
    SET2 = 1,
    SET3 = 2,
}

pub struct PS2ScancodeReader {
    scan_code_set: ScanCodeSet,
}

pub enum ControlKey {
    Escape = 0,
    Backspace = 1,
    Tab = 2,
    Enter = 3,
    LeftCtrl = 4,
    LeftShift = 5,
}

pub struct PS2Key{
    pub key: Option<char>,
    pub control_key: Option<ControlKey>,
    pub pressed: bool   // True on pressed, false on released
}

impl PS2ScancodeReader {

    pub fn new(scan_code_set: ScanCodeSet) -> PS2ScancodeReader {
        PS2ScancodeReader{scan_code_set: scan_code_set}
    }

    pub fn set_scancode_set(&mut self, set: ScanCodeSet){
        self.scan_code_set = set;
    }

    pub fn match_scancode(&mut self, code: u8) -> PS2Key {

        match self.scan_code_set {
            ScanCodeSet::SET1 => self.match_set1_scancode(code),
            _ => PS2Key {key: None, control_key: None, pressed: false}
        }

    }

    fn match_set1_scancode(&mut self, code: u8) -> PS2Key {
        let key = match code {
            // KEY PRESSES
            0x01 => PS2Key {key: None, control_key: Some(ControlKey::Escape), pressed: true},
            0x02 => PS2Key {key: Some('1'), control_key: None, pressed: true},
            0x03 => PS2Key {key: Some('2'), control_key: None, pressed: true},
            0x04 => PS2Key {key: Some('3'), control_key: None, pressed: true},
            0x05 => PS2Key {key: Some('4'), control_key: None, pressed: true},
            0x06 => PS2Key {key: Some('5'), control_key: None, pressed: true},
            0x07 => PS2Key {key: Some('6'), control_key: None, pressed: true},
            0x08 => PS2Key {key: Some('7'), control_key: None, pressed: true},
            0x09 => PS2Key {key: Some('8'), control_key: None, pressed: true},
            0x0a => PS2Key {key: Some('9'), control_key: None, pressed: true},
            0x0b => PS2Key {key: Some('0'), control_key: None, pressed: true},
            0x0c => PS2Key {key: Some('-'), control_key: None, pressed: true},
            0x0d => PS2Key {key: Some('='), control_key: None, pressed: true},
            0x0e => PS2Key {key: None, control_key: Some(ControlKey::Backspace), pressed: true},
            0x0f => PS2Key {key: None, control_key: Some(ControlKey::Tab), pressed: true},
            0x10 => PS2Key {key: Some('q'), control_key: None, pressed: true},
            0x11 => PS2Key {key: Some('w'), control_key: None, pressed: true},
            0x12 => PS2Key {key: Some('e'), control_key: None, pressed: true},
            0x13 => PS2Key {key: Some('r'), control_key: None, pressed: true},
            0x14 => PS2Key {key: Some('t'), control_key: None, pressed: true},
            0x15 => PS2Key {key: Some('y'), control_key: None, pressed: true},
            0x16 => PS2Key {key: Some('u'), control_key: None, pressed: true},
            0x17 => PS2Key {key: Some('i'), control_key: None, pressed: true},
            0x18 => PS2Key {key: Some('o'), control_key: None, pressed: true},
            0x19 => PS2Key {key: Some('p'), control_key: None, pressed: true},
            0x1a => PS2Key {key: Some('['), control_key: None, pressed: true},
            0x1b => PS2Key {key: Some(']'), control_key: None, pressed: true},
            0x1c => PS2Key {key: None, control_key: Some(ControlKey::Enter), pressed: true},
            0x1d => PS2Key {key: None, control_key: Some(ControlKey::LeftCtrl), pressed: true},
            0x1e => PS2Key {key: Some('a'), control_key: None, pressed: true},
            0x1f => PS2Key {key: Some('s'), control_key: None, pressed: true},
            0x20 => PS2Key {key: Some('d'), control_key: None, pressed: true},
            0x21 => PS2Key {key: Some('f'), control_key: None, pressed: true},
            0x22 => PS2Key {key: Some('g'), control_key: None, pressed: true},
            0x23 => PS2Key {key: Some('h'), control_key: None, pressed: true},
            0x24 => PS2Key {key: Some('j'), control_key: None, pressed: true},
            0x25 => PS2Key {key: Some('k'), control_key: None, pressed: true},
            0x26 => PS2Key {key: Some('l'), control_key: None, pressed: true},
            0x27 => PS2Key {key: Some(';'), control_key: None, pressed: true},
            0x28 => PS2Key {key: Some('\''), control_key: None, pressed: true},
            0x29 => PS2Key {key: Some('`'), control_key: None, pressed: true},
            0x2a => PS2Key {key: None, control_key:Some(ControlKey::LeftShift), pressed: true},
            0x2b => PS2Key {key: Some('\\'), control_key: None, pressed: true},
            0x2c => PS2Key {key: Some('z'), control_key: None, pressed: true},
            0x2d => PS2Key {key: Some('x'), control_key: None, pressed: true},
            0x2e => PS2Key {key: Some('c'), control_key: None, pressed: true},
            0x2f => PS2Key {key: Some('v'), control_key: None, pressed: true},
            // TODO: Below here has not been implemented yet
            0x30 => PS2Key {key: None, control_key: None, pressed: true},
            0x31 => PS2Key {key: None, control_key: None, pressed: true},
            0x32 => PS2Key {key: None, control_key: None, pressed: true},
            0x33 => PS2Key {key: None, control_key: None, pressed: true},
            0x34 => PS2Key {key: None, control_key: None, pressed: true},
            0x35 => PS2Key {key: None, control_key: None, pressed: true},
            0x36 => PS2Key {key: None, control_key: None, pressed: true},
            0x37 => PS2Key {key: None, control_key: None, pressed: true},
            0x38 => PS2Key {key: None, control_key: None, pressed: true},
            0x39 => PS2Key {key: None, control_key: None, pressed: true},
            0x3a => PS2Key {key: None, control_key: None, pressed: true},
            0x3b => PS2Key {key: None, control_key: None, pressed: true},
            0x3c => PS2Key {key: None, control_key: None, pressed: true},
            0x3d => PS2Key {key: None, control_key: None, pressed: true},
            0x3e => PS2Key {key: None, control_key: None, pressed: true},
            0x3f => PS2Key {key: None, control_key: None, pressed: true},
            0x40 => PS2Key {key: None, control_key: None, pressed: true},
            0x41 => PS2Key {key: None, control_key: None, pressed: true},
            0x42 => PS2Key {key: None, control_key: None, pressed: true},
            0x43 => PS2Key {key: None, control_key: None, pressed: true},
            0x44 => PS2Key {key: None, control_key: None, pressed: true},
            0x45 => PS2Key {key: None, control_key: None, pressed: true},
            0x46 => PS2Key {key: None, control_key: None, pressed: true},
            0x47 => PS2Key {key: None, control_key: None, pressed: true},
            0x48 => PS2Key {key: None, control_key: None, pressed: true}, 
            0x49 => PS2Key {key: None, control_key: None, pressed: true},
            0x4a => PS2Key {key: None, control_key: None, pressed: true},
            0x4b => PS2Key {key: None, control_key: None, pressed: true},
            0x4c => PS2Key {key: None, control_key: None, pressed: true},
            0x4d => PS2Key {key: None, control_key: None, pressed: true},
            0x4e => PS2Key {key: None, control_key: None, pressed: true},
            0x4f => PS2Key {key: None, control_key: None, pressed: true},
            0x50 => PS2Key {key: None, control_key: None, pressed: true},
            0x51 => PS2Key {key: None, control_key: None, pressed: true},
            0x52 => PS2Key {key: None, control_key: None, pressed: true},
            0x53 => PS2Key {key: None, control_key: None, pressed: true},
            
            // Break in code sequence
            0x58 => PS2Key {key: None, control_key: None, pressed: true},
            
            // KEY RELEASES
            0x80 => PS2Key {key: None, control_key: None, pressed: false},
            0x81 => PS2Key {key: None, control_key: None, pressed: false},
            0x82 => PS2Key {key: None, control_key: None, pressed: false},
            0x83 => PS2Key {key: None, control_key: None, pressed: false},
            0x84 => PS2Key {key: None, control_key: None, pressed: false},
            0x85 => PS2Key {key: None, control_key: None, pressed: false},
            0x86 => PS2Key {key: None, control_key: None, pressed: false},
            0x87 => PS2Key {key: None, control_key: None, pressed: false},
            0x88 => PS2Key {key: None, control_key: None, pressed: false},
            0x89 => PS2Key {key: None, control_key: None, pressed: false},
            0x8a => PS2Key {key: None, control_key: None, pressed: false},
            0x8b => PS2Key {key: None, control_key: None, pressed: false},
            0x8c => PS2Key {key: None, control_key: None, pressed: false},
            0x8d => PS2Key {key: None, control_key: None, pressed: false},
            0x8e => PS2Key {key: None, control_key: None, pressed: false},
            0x8f => PS2Key {key: None, control_key: None, pressed: false},
            0x90 => PS2Key {key: None, control_key: None, pressed: false},
            0x91 => PS2Key {key: None, control_key: None, pressed: false},
            0x92 => PS2Key {key: None, control_key: None, pressed: false},
            0x93 => PS2Key {key: None, control_key: None, pressed: false},
            0x94 => PS2Key {key: None, control_key: None, pressed: false},
            0x95 => PS2Key {key: None, control_key: None, pressed: false},
            0x96 => PS2Key {key: None, control_key: None, pressed: false},
            0x97 => PS2Key {key: None, control_key: None, pressed: false},
            0x98 => PS2Key {key: None, control_key: None, pressed: false},
            0x99 => PS2Key {key: None, control_key: None, pressed: false},
            0x9a => PS2Key {key: None, control_key: None, pressed: false},
            0x9b => PS2Key {key: None, control_key: None, pressed: false},
            0x9c => PS2Key {key: None, control_key: None, pressed: false},
            0x9d => PS2Key {key: None, control_key: None, pressed: false},
            0x9e => PS2Key {key: None, control_key: None, pressed: false},
            0x9f => PS2Key {key: None, control_key: None, pressed: false},
            0xa0 => PS2Key {key: None, control_key: None, pressed: false},
            0xa1 => PS2Key {key: None, control_key: None, pressed: false},
            0xa2 => PS2Key {key: None, control_key: None, pressed: false},
            0xa3 => PS2Key {key: None, control_key: None, pressed: false},
            0xa4 => PS2Key {key: None, control_key: None, pressed: false},
            0xa5 => PS2Key {key: None, control_key: None, pressed: false},
            0xa6 => PS2Key {key: None, control_key: None, pressed: false},
            0xa7 => PS2Key {key: None, control_key: None, pressed: false},
            0xa8 => PS2Key {key: None, control_key: None, pressed: false},
            0xa9 => PS2Key {key: None, control_key: None, pressed: false},
            0xaa => PS2Key {key: None, control_key: None, pressed: false},
            0xab => PS2Key {key: None, control_key: None, pressed: false},
            0xac => PS2Key {key: None, control_key: None, pressed: false},
            0xad => PS2Key {key: None, control_key: None, pressed: false},
            0xae => PS2Key {key: None, control_key: None, pressed: false},
            0xaf => PS2Key {key: None, control_key: None, pressed: false},
            0xb0 => PS2Key {key: None, control_key: None, pressed: false},
            0xb1 => PS2Key {key: None, control_key: None, pressed: false},
            0xb2 => PS2Key {key: None, control_key: None, pressed: false},
            0xb3 => PS2Key {key: None, control_key: None, pressed: false},
            0xb4 => PS2Key {key: None, control_key: None, pressed: false},
            0xb5 => PS2Key {key: None, control_key: None, pressed: false},
            0xb6 => PS2Key {key: None, control_key: None, pressed: false},
            0xb7 => PS2Key {key: None, control_key: None, pressed: false},
            0xb8 => PS2Key {key: None, control_key: None, pressed: false},
            0xb9 => PS2Key {key: None, control_key: None, pressed: false},
            0xba => PS2Key {key: None, control_key: None, pressed: false},
            0xbb => PS2Key {key: None, control_key: None, pressed: false},
            0xbc => PS2Key {key: None, control_key: None, pressed: false},
            0xbd => PS2Key {key: None, control_key: None, pressed: false},
            0xbe => PS2Key {key: None, control_key: None, pressed: false},
            0xbf => PS2Key {key: None, control_key: None, pressed: false}, 
            0xc0 => PS2Key {key: None, control_key: None, pressed: false},
            0xc1 => PS2Key {key: None, control_key: None, pressed: false},
            0xc2 => PS2Key {key: None, control_key: None, pressed: false},
            0xc3 => PS2Key {key: None, control_key: None, pressed: false},
            0xc4 => PS2Key {key: None, control_key: None, pressed: false},
            0xc5 => PS2Key {key: None, control_key: None, pressed: false},
            0xc6 => PS2Key {key: None, control_key: None, pressed: false},
            0xc7 => PS2Key {key: None, control_key: None, pressed: false},
            0xc8 => PS2Key {key: None, control_key: None, pressed: false},
            0xc9 => PS2Key {key: None, control_key: None, pressed: false},
            0xca => PS2Key {key: None, control_key: None, pressed: false},
            0xcb => PS2Key {key: None, control_key: None, pressed: false},
            0xcc => PS2Key {key: None, control_key: None, pressed: false},
            0xcd => PS2Key {key: None, control_key: None, pressed: false},
            0xce => PS2Key {key: None, control_key: None, pressed: false},
            0xcf => PS2Key {key: None, control_key: None, pressed: false},
            0xd0 => PS2Key {key: None, control_key: None, pressed: false},
            0xd1 => PS2Key {key: None, control_key: None, pressed: false},
            0xd2 => PS2Key {key: None, control_key: None, pressed: false},
            0xd3 => PS2Key {key: None, control_key: None, pressed: false},
            
            0xd7 => PS2Key {key: None, control_key: None, pressed: false},
            0xd8 => PS2Key {key: None, control_key: None, pressed: false},
           
            _ => PS2Key {key: None, control_key: None, pressed: false}
        };

        key
    }

}
