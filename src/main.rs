use std::fs::File;
use std::io::{self, BufRead};

use crate::cpu::Cpu;
use crate::cpu::RiscvInstruction;
use crate::cpu::Breg;
use crate::cpu::Memory;

mod assembler;
mod cpu;

fn main() -> io::Result<()> {
    // let args : Vec<String> = std::env::args().collect();
    // assembler::assemble(&args[1]);

    let file = File::open("hello2_text.txt")?;
    let file2 = File::open("hello2_data.txt")?;
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

    while cpu.pc < 0xffc as u32 {
        cpu.fetch();
        cpu.decode(cpu.inst);
        cpu.print_instruction();
        cpu.execute();
        cpu.pc += 1;
    }

    Ok(())
}
