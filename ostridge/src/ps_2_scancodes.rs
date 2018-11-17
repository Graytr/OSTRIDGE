
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
    ESCAPE = 0,
    BACKSPACE = 1,
    TAB = 2,
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
            0x01 => PS2Key {key: None, control_key: Some(ControlKey::ESCAPE), pressed: true},
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
            0x0e => PS2Key {key: None, control_key: Some(ControlKey::BACKSPACE), pressed: true},
            0x0f => PS2Key {key: None, control_key: Some(ControlKey::TAB), pressed: true},
            _ => PS2Key {key: None, control_key: None, pressed: false}
        };

        key
    }

}
