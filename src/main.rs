// Para executar o simulador, basta usar o comando cargo run <nome do arquivo assembly> no terminal.
// O arquivo assembly deve estar na pasta raiz do projeto.
// Os arquivos binários gerados pelo montador estarão na pasta raiz do projeto.
// O arquivo bin_data.txt contém os dados do programa e o arquivo bin_text.txt contém as instruções do programa.




use std::fs::File;
use std::io::{self, BufRead};


use crate::cpu::Cpu;
use crate::cpu::RiscvInstruction;
use crate::cpu::Breg;
use crate::cpu::Memory;


mod assembler;
mod cpu;

fn main() -> io::Result<()> {
    // Chamando o montador para gerar os arquivos binários
    let args : Vec<String> = std::env::args().collect();
    assembler::assemble(&args[1]);
    println!("Arquivo binário gerado com sucesso!");
    
    // Carregando o arquivo binário gerado pelo montador
    let file = File::open("teste_aritmetico_text.txt")?;
    let file2 = File::open("bin_data.txt")?;
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
    // Carregando a memória de dados
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
    // Mostrando memória de texto e de dados carregada
    cpu.memory.print_memory();
    // Executando o simulador
    while cpu.pc < 0xffc as u32 {
        
        cpu.fetch();
        cpu.decode(cpu.inst);
        cpu.breg.print_reg();
        cpu.execute();
        if(cpu.instruction.opcode != 0x67 && cpu.instruction.opcode != 0x63 && cpu.instruction.opcode != 0x6f){
            cpu.pc += 1;
        }
    }

    Ok(())
}
