use std::fs::File;
use std::io::{self, BufRead};


mod assembler;
// mod cpu;

// struct RiscvInstruction {
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
    imm_j: i32,
}

struct Cpu<'a> {
    pc: u32,
    breg: &'a mut Breg,
    memory: &'a mut Memory,
    instruction: &'a mut RiscvInstruction,
    inst: u32,
}
impl Cpu<'_> {
    fn new<'a>(
        breg: &'a mut Breg,
        memory: &'a mut Memory,
        instruction: &'a mut RiscvInstruction,
    ) -> Cpu<'a> {
        Cpu {
            pc: 0,
            breg: breg,
            memory: memory,
            instruction: instruction,
            inst: 0,
        }
    }
    fn fetch(&mut self) -> () {
        println!("=============================================");
        println!("retorno do fetch: {:x}", self.memory.read_text_word(self.pc as usize));
        self.inst = self.memory.read_text_word(self.pc as usize);
        println!("self.inst: {:b}", self.inst as u32);
    }
    fn decode(&mut self, instruction: u32) -> () {
        let opcode = instruction & 0x7F;
        let rd = (instruction >> 7) & 0x1F;
        let funct3 = (instruction >> 12) & 0x7;
        let rs1 = (instruction >> 15) & 0x1F;
        let rs2 = (instruction >> 20) & 0x1F;
        let funct7 = (instruction >> 25) & 0x7F;
        let imm_i = ((instruction >> 20) & 0xFFF) as i32;
        let imm_s = (((instruction >> 25) & 0x7F) << 5 | ((instruction >> 7) & 0x1F)) as i32;
        let imm_b = (((instruction >> 31) & 0x1) << 12
            | ((instruction >> 7) & 0x1) << 11
            | ((instruction >> 25) & 0x3F) << 5
            | ((instruction >> 8) & 0xF)) as i32;
        let imm_u = ((instruction >> 12) & 0xFFFFF) as i32;
        let imm_j = (((instruction >> 31) & 0x1) << 20
            | ((instruction >> 21) & 0x3FF) << 1
            | ((instruction >> 20) & 0x1) << 11
            | ((instruction >> 12) & 0xFF)) as i32;
        //carregando o campo instruction com a instrução decodificada
        self.instruction.opcode = opcode as u8;
        self.instruction.rd = rd as u8;
        self.instruction.funct3 = funct3 as u8;
        self.instruction.rs1 = rs1 as u8;
        self.instruction.rs2 = rs2 as u8;
        self.instruction.funct7 = funct7 as u8;
        self.instruction.imm_i = imm_i;
        self.instruction.imm_s = imm_s;
        self.instruction.imm_b = imm_b;
        self.instruction.imm_u = imm_u;
        self.instruction.imm_j = imm_j;
    }
    fn print_instruction(&self) {
        println!("=============================================");
        println!("opcode: {:b}", self.instruction.opcode as u32);
        println!("rd: {:b}", self.instruction.rd as u32);
        println!("funct3: {:b}", self.instruction.funct3 as u32);
        println!("rs1: {:b}", self.instruction.rs1 as u32) ;
        println!("rs2: {:b}", self.instruction.rs2 as u32);
        println!("funct7: {:b}", self.instruction.funct7);
        println!("imm_i: {:b}", self.instruction.imm_i as u32);
        println!("imm_s: {:b}", self.instruction.imm_s as u32);
        println!("imm_b: {:b}", self.instruction.imm_b as u32);
        println!("imm_u: {:b}", self.instruction.imm_u as u32);
        println!("imm_j: {:b}", self.instruction.imm_j as u32);
    }
}

struct Breg {
    reg: [u32; 32],
}

impl Breg {
    fn new() -> Self {
        Breg {
            reg: [0; 32],
        }
    }
    fn get_reg(&mut self, addr: u8) -> u32 {
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
    fn print_reg(&self) {
        for (i, &word) in self.reg.iter().enumerate() {
            if word != 0 {
                println!("{:x}: {:x}", i, word);
            }
        }
    }
}

struct Memory {
    text_segment: Vec<u32>,
    data_segment: Vec<u32>,
}

impl Memory {
    fn new() -> Self {
        Memory {
            text_segment: vec![0; 0x1000],
            data_segment: vec![0; 0x1000],
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

    fn read_data_word(&mut self, address: usize) -> u32 {
        if address < self.data_segment.len() {
            self.data_segment[address]
        } else {
            println!("Endereço fora dos limites da memória de dados.");
            0
        }
    }
    fn read_text_word(&mut self, address: usize) -> u32 {
        if address < self.text_segment.len() {
            self.text_segment[address]
        } else {
            println!("Endereço fora dos limites da memória de dados.");
            0
        }
    }
    fn print_memory(&mut self) {
        println!("Memória de texto: ");
        self.print_text_segment();
        println!("Memória de dados: ");
        self.print_data_segment();
    }
    fn print_text_segment(&mut self) {
        for (k, &word) in self.text_segment.iter().enumerate() {
            if (word != 0) {
                println!("{:x}: {:x}", k, word);
            }
        }
    }
    fn print_data_segment(&mut self) {
        for (j, &word) in self.data_segment.iter().enumerate() {
            if (word != 0) {
                println!("{:x}: {:x}", j, word);
            }
        }
    }
}

fn main() -> io::Result<()> {
    // let args : Vec<String> = std::env::args().collect();
    // assembler::assemble(&args[1]);

    let file = File::open("hello_text")?;
    let file2 = File::open("hello_data")?;
    let reader = io::BufReader::new(file);
    let reader2 = io::BufReader::new(file2);
    //instanciação da memória e dos registradores
    let mut memory = Memory::new();
    let mut breg = Breg::new();
    let mut instruction = RiscvInstruction {
        opcode: 0,
        rd: 0,
        funct3: 0,
        rs1: 0,
        rs2: 0,
        funct7: 0,
        imm_i: 0,
        imm_s: 0,
        imm_b: 0,
        imm_u: 0,
        imm_j: 0,
    };
    let mut cpu = Cpu::new(&mut breg, &mut memory, &mut instruction);

    // Carregando a memória de texto
    let mut i = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let value = u32::from_str_radix(&line, 2);
        match value {
            Ok(value) => {
                cpu.memory.write_text_word(i, value);
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
                cpu.memory.write_data_word(i, value);
            }
            Err(_) => {
                println!("Erro ao converter valor para inteiro");
            }
        }
        i += 1;
    }
    //memória de texto e de dados carregada
    cpu.memory.print_memory();

    //fetch -> vamos começar com o PC = 0
    cpu.fetch();
    //decode
    cpu.decode(cpu.inst);
    // mostrando a instrução decodificada
    cpu.print_instruction();
    // cpu.fetch();

    // cpu.decode(cpu.fetch());

    Ok(())
}
