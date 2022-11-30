
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

impl Registers{
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