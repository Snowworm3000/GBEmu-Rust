mod cpu;
use cpu::CPU;
// use crate::cpu;

pub fn run() {
    CPU::new();
}

#[cfg(test)]
mod tests {
    use crate::cpu::registers::Registers;

    #[test]
    fn set_register(){
        let mut registers = Registers::new_empty();
        registers.set_af(0b00110011);
        let a = registers.a;
        println!("register a contains the value: {a}");
        assert_eq!(0b00110011, registers.get_af());
    }
}