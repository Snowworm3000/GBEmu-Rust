
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
}

impl Registers{
    pub fn new_empty() -> Registers{
        Registers {a: 0, b: 0, c: 0, d: 0, e: 0, f: 0, h:0, l:0}
    }
    pub fn get_af(&self) -> u16{
        (self.a as u16) << 8 | self.f as u16
    }
    pub fn get_bc(&self) -> u16{
        (self.b as u16) << 8 | self.c as u16
    }
    pub fn get_de(&self) -> u16{
        (self.d as u16) << 8 | self.e as u16
    }
    pub fn get_hl(&self) -> u16{
        (self.h as u16) << 8 | self.l as u16
    }
    pub fn set_af(&mut self, v:u16) {
        self.a = (v >> 8) as u8;
        self.f = v as u8;
    }
    pub fn set_bc(&mut self, v:u16) {
        self.b = (v >> 8) as u8;
        self.c = v as u8;
    }
    pub fn set_de(&mut self, v:u16) {
        self.d = (v >> 8) as u8;
        self.e = v as u8;
    }
    pub fn set_hl(&mut self, v:u16) {
        self.h = (v >> 8) as u8;
        self.l = v as u8;
    }

    
}


mod POSITION {
    pub const Z: u8 = 7;
    pub const N: u8 = 6;
    pub const H: u8 = 5;
    pub const C: u8 = 4;
}

enum Flags { // Flags correspond to the last nibble in the f register.
    Z = (1 << POSITION::Z), // Zero
    N = (1 << POSITION::N), // Subtract
    H = (1 << POSITION::H), // Half carry
    C = (1 << POSITION::C), // Carry
}
