use std::io::Write;

const MEM_SIZE: usize = 1024; // Tamanho da memória

pub struct Mem {
    sram: [u32; MEM_SIZE],
}

impl Mem {
    pub fn new() -> Self {
        let mut sram = [0; MEM_SIZE];
        for i in 0..MEM_SIZE {
            sram[i] = 0;
        }
        Mem { sram }
    }

    pub fn get_mem(&self, addr: u32) -> u32 {
        if addr >= MEM_SIZE as u32 {
            writeln!(std::io::stderr(), "Mem: invalid memory address").unwrap();
            std::process::exit(1);
        }
        self.sram[addr as usize]
    }

    pub fn get_mem_byte(&self, addr: u32) -> u8 {
        if addr >= MEM_SIZE as u32 {
            writeln!(std::io::stderr(), "Mem: invalid memory address").unwrap();
            std::process::exit(1);
        }
        (self.sram[addr as usize] & 0x000000FF) as u8
    }

    pub fn set_mem(&mut self, addr: u32, val: u32) {
        if addr >= MEM_SIZE as u32 {
            writeln!(std::io::stderr(), "Mem: invalid memory address").unwrap();
            std::process::exit(1);
        }
        self.sram[addr as usize] = val;
    }
}
