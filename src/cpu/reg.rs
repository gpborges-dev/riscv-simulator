//! Neste arquivo estão as definições do banco de registradores da CPU.
//! O banco de registradores é um vetor de 32 posições, cada posição representa um registrador.


pub struct Breg {
    pub reg: [i32; 32],
}

impl Breg {
    /// A função new() instancia o banco de registradores com 32 registradores de 32 bits.
    pub fn new() -> Self {
        Breg { reg: [0; 32] }
    }
    /// A função get_reg() retorna o valor de um registrador.
    pub fn get_reg(&mut self, addr: u8) -> i32 {
        if addr > 32 {
            println!("Registrador inválido");
            0
        } else {
            self.reg[addr as usize]
        }
    }
    /// A função set_reg() escreve um valor em um registrador.
    pub fn set_reg(&mut self, addr: u8, value: i32) {
        if addr > 32 {
            println!("Registrador inválido");
        } else {
            self.reg[addr as usize] = value;
        }
    }
    /// A função print_reg() imprime o banco de registradores.
    pub fn print_reg(&self) {
        println!("=====================================================================");
        println!("Banco de registradores: ");
        for i in 0..32 {
            println!("registrador x{}: {:x}", i, self.reg[i]);
        }
        println!("=====================================================================");
    }
}