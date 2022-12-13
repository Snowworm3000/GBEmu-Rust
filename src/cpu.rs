// pub use crate::registers::Registers;
pub mod registers;
pub struct CPU {
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            status: 0,
            program_counter: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;
        loop {
            let opscode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opscode{
                // 0x
                _ => todo!()
            }
        }
    }
}


