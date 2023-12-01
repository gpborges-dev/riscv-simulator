use std::fs::File;
use std::io::{self, BufRead};

mod assembler;
// mod cpu;


struct RiscvInstruction {
    opcode: u8,
    rd: u8,
    funct3: u8,
    rs1: u8,
    rs2: u8,
    funct7: u8,
    imm_i: i32,
    imm_s: i32,
    imm_b: i32,
    imm_u: i32,
    imm_j: i32
}

struct Breg {
    pc: u32,
    reg: [u32; 32]
}

impl Breg {
    fn new() -> Self {
        Breg {
            pc: 0,
            reg: [0; 32]
        }
    }
    fn get_reg(&self, addr: u8) -> u32 {
        if addr > 32 {
            println!("Registrador inválido");
            0
        } else {
            self.reg[addr as usize]
        }
    }
    fn set_reg(&mut self, addr: u8, value: u32) {
        if addr > 32 {
            println!("Registrador inválido");
        } else {
            self.reg[addr as usize] = value;
        }
    }
    fn get_pc(&self) -> u32 {
        self.pc
    }
    fn set_pc(&mut self, value: u32) {
        self.pc = value;
    }
    fn print_reg(&self) {
        for (i, &word) in self.reg.iter().enumerate() {
            if(word != 0){
                println!("{:x}: {:x}", i, word);
            }
        }
    }


}


struct Memory {
    text_segment: Vec<u32>,
    data_segment: Vec<u32>
}

impl Memory {
    fn new() -> Self {
        Memory {
            text_segment: vec![0; 0x1000],
            data_segment: vec![0; 0x1000]
        }
    }

    fn write_data_word(&mut self, address: u32, value: u32) {
        let address = address as usize;
        if address < self.data_segment.len() {
            self.data_segment[address] = value;
        } else {
            println!("Endereço fora dos limites da memória de dados.");
        }
    }
    fn write_text_word(&mut self, address: u32, value: u32) {
        let address = address as usize;
        if address < self.text_segment.len() {
            self.text_segment[address] = value;
        } else {
            println!("Endereço fora dos limites da memória de dados.");
        }
    }

    fn read_word(&self, address: usize) -> u32 {
        if address < self.data_segment.len() {
            self.data_segment[address]
        } else {
            println!("Endereço fora dos limites da memória de dados.");
            0 
        }
    }
    fn print_memory(&self) {
        println!("Memória de texto: ");
        self.print_text_segment();
        println!("Memória de dados: ");
        self.print_data_segment();
    }
    fn print_text_segment(&self) {
        for (k, &word) in self.text_segment.iter().enumerate() {
            if(word != 0){
                println!("{:x}: {:x}", k, word);
            }            
        }
    }
    fn print_data_segment(&self) {
        for (j, &word) in self.data_segment.iter().enumerate() {
            if(word != 0){
                println!("{:x}: {:x}", j, word);
            }
        }
    }
}

fn main() -> io::Result<()>{
    // let args : Vec<String> = std::env::args().collect();
    // assembler::assemble(&args[1]);

    let file = File::open("hello_text")?;
    let file2 = File::open("hello_data")?;
    let reader = io::BufReader::new(file);
    let reader2 = io::BufReader::new(file2);
    //instanciação da memória e dos registradores
    let mut memory = Memory::new();
    let mut pc = 0;


    // Carregando a memória de texto
    let mut i = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let value = u32::from_str_radix(&line, 2);
        match value {
            Ok(value) => {
                memory.write_text_word(i, value);
            }
            Err(_) => {
                println!("Erro ao converter valor para inteiro");
            }
        }
        i += 1;
    }
    let mut i = 0;
    for line in reader2.lines() {
        let line = line.unwrap();
        let value = u32::from_str_radix(&line, 2);
        match value {
            Ok(value) => {
                memory.write_data_word(i, value);
            }
            Err(_) => {
                println!("Erro ao converter valor para inteiro");
            }
        }
        i += 1;
    }
    //memória de texto e de dados carregada
    memory.print_memory();

    //fetch -> vamos começar com o PC = 0

    
    


    Ok(())
}