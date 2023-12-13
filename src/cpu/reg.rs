pub struct Breg {
    pub reg: [i32; 32],
}

impl Breg {
    pub fn new() -> Self {
        Breg { reg: [0; 32] }
    }
    pub fn get_reg(&mut self, addr: u8) -> i32 {
        if addr > 32 {
            println!("Registrador inválido");
            0
        } else {
            self.reg[addr as usize]
        }
    }
    pub fn set_reg(&mut self, addr: u8, value: i32) {
        if addr > 32 {
            println!("Registrador inválido");
        } else {
            self.reg[addr as usize] = value;
        }
    }
    pub fn print_reg(&self) {
        println!("=====================================================================");
        println!("Banco de registradores: ");
        for i in 0..32 {
            println!("registrador x{}: {:x}", i, self.reg[i]);
        }
        println!("=====================================================================");
    }
}