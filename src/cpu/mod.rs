mod mem;
mod reg;
pub use mem::Memory;
pub use reg::Breg;
pub use std::process;

pub struct RiscvInstruction {
    pub opcode: u8,
    pub rd: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub funct7: u8,
    pub imm_i: i32,
    pub imm_s: i32,
    pub imm_b: i32,
    pub imm_u: i32,
    pub imm_j: i32,
}

pub struct Cpu<'a> {
    pub pc: u32,
    pub breg: &'a mut Breg,
    pub memory: &'a mut Memory,
    pub instruction: &'a mut RiscvInstruction,
    pub inst: u32,
}
impl Cpu<'_> {
    pub fn new<'a>(
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
    pub fn fetch(&mut self) -> () {
        println!("=============================================");
        println!(
            "retorno do fetch: {:x}",
            self.memory.read_text_word(self.pc as usize)
        );
        self.inst = self.memory.read_text_word(self.pc as usize);
        println!("self.inst: {:032b}", self.inst as u32);
    }
    pub fn decode(&mut self, instruction: u32) -> () {
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
    pub fn print_instruction(&self) {
        println!("=============================================");
        println!("opcode: {:b}", self.instruction.opcode as u32);
        println!("rd: {:b}", self.instruction.rd as u32);
        println!("funct3: {:b}", self.instruction.funct3 as u32);
        println!("rs1: {:b}", self.instruction.rs1 as u32);
        println!("rs2: {:b}", self.instruction.rs2 as u32);
        println!("funct7: {:b}", self.instruction.funct7);
        println!("imm_i: {:b}", self.instruction.imm_i as u32);
        println!("imm_s: {:b}", self.instruction.imm_s as u32);
        println!("imm_b: {:b}", self.instruction.imm_b as u32);
        println!("imm_u: {:b}", self.instruction.imm_u as u32);
        println!("imm_j: {:b}", self.instruction.imm_j as u32);
    }
    pub fn execute(&mut self) {
        match self.instruction.opcode {
            0x33 => {
                // println!("Instrução do tipo R");
                match self.instruction.funct3 {
                    0x0 => match self.instruction.funct7 {
                        0x0 => {
                            // println!("Instrução ADD");
                            let rs1 = self.breg.get_reg(self.instruction.rs1);
                            let rs2 = self.breg.get_reg(self.instruction.rs2);
                            let rd = rs1 + rs2;
                            self.breg.set_reg(self.instruction.rd, rd);
                        }
                        0x1 => {
                            // println!("Instrução MUL");
                            //
                            let rs1 = self.breg.get_reg(self.instruction.rs1);
                            let rs2 = self.breg.get_reg(self.instruction.rs2);
                            let rd = rs1 * rs2;
                            self.breg.set_reg(self.instruction.rd, rd);
                        }
                        0x20 => {
                            // println!("Instrução SUB");
                            let rs1 = self.breg.get_reg(self.instruction.rs1);
                            let rs2 = self.breg.get_reg(self.instruction.rs2);
                            let rd = rs1 - rs2;
                            self.breg.set_reg(self.instruction.rd, rd);
                        }
                        _ => {
                            println!("Instrução não implementada");
                        }
                    },
                    0x1 => {
                        // println!("Instrução SLL");
                        let rs1 = self.breg.get_reg(self.instruction.rs1);
                        let rs2 = self.breg.get_reg(self.instruction.rs2);
                        let rd = rs1 << rs2;
                        self.breg.set_reg(self.instruction.rd, rd);
                    }
                    0x2 => {
                        // println!("Instrução SLT");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as i32;
                        let rs2 = self.breg.get_reg(self.instruction.rs2) as i32;
                        let rd = if rs1 < rs2 { 1 } else { 0 };
                        self.breg.set_reg(self.instruction.rd, rd);
                    }
                    0x3 => {
                        // println!("Instrução SLTU");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as u32;
                        let rs2 = self.breg.get_reg(self.instruction.rs2) as u32;
                        let rd = if rs1 < rs2 { 1 } else { 0 };
                        self.breg.set_reg(self.instruction.rd, rd);
                    }
                    0x4 => {
                        // println!("Instrução XOR");
                        let rs1 = self.breg.get_reg(self.instruction.rs1);
                        let rs2 = self.breg.get_reg(self.instruction.rs2);
                        let rd = rs1 ^ rs2;
                        self.breg.set_reg(self.instruction.rd, rd);
                    }
                    0x5 => {
                        // println!("Instrução SRL");
                        let rs1 = self.breg.get_reg(self.instruction.rs1);
                        let rs2 = self.breg.get_reg(self.instruction.rs2);
                        let rd = rs1 >> rs2;
                        self.breg.set_reg(self.instruction.rd, rd);
                    }
                    0x6 => {
                        // println!("Instrução OR");
                        let rs1 = self.breg.get_reg(self.instruction.rs1);
                        let rs2 = self.breg.get_reg(self.instruction.rs2);
                        let rd = rs1 | rs2;
                        self.breg.set_reg(self.instruction.rd, rd);
                    }
                    0x7 => {
                        // println!("Instrução AND");
                        let rs1 = self.breg.get_reg(self.instruction.rs1);
                        let rs2 = self.breg.get_reg(self.instruction.rs2);
                        let rd = rs1 & rs2;
                        self.breg.set_reg(self.instruction.rd, rd);
                    }
                    _ => {
                        println!("Instrução não implementada");
                    }
                }
            }
            0x13 => {
                // println!("Instrução do tipo I");
                match self.instruction.funct3 {
                    0x0 => {
                        println!("Instrução ADDI");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as i32;
                        let imm_i = self.instruction.imm_i;
                        println!("{}", imm_i >> 31);
                        let rd = rs1 + imm_i;
                        println!("valor de rd: {}", rd as u32);
                        self.breg.set_reg(self.instruction.rd, rd);
                    }
                    0x1 => {
                        // println!("Instrução SLLI");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as i32;
                        let imm_i = self.instruction.imm_i;
                        let rd = rs1 << imm_i;
                        self.breg.set_reg(self.instruction.rd, rd);
                        
                    }
                    0x2 => {
                        // println!("Instrução SLTI");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as i32;
                        let imm_i = self.instruction.imm_i;
                        let rd = if rs1 < imm_i { 1 } else { 0 };
                        self.breg.set_reg(self.instruction.rd, rd);
                    }
                    0x3 => {
                        // println!("Instrução SLTIU");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as u32;
                        let imm_i = self.instruction.imm_i as u32;
                        let rd = if rs1 < imm_i { 1 } else { 0 };
                        self.breg.set_reg(self.instruction.rd, rd as i32);
                    }
                    0x4 => {
                        // println!("Instrução XORI");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as i32;
                        let imm_i = self.instruction.imm_i;
                        let rd = rs1 ^ imm_i;
                        self.breg.set_reg(self.instruction.rd, rd);
                    }
                    0x5 => {
                        // println!("Instrução SRLI");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as i32;
                        let imm_i = self.instruction.imm_i;
                        let rd = rs1 >> imm_i;
                        self.breg.set_reg(self.instruction.rd, rd);
                    }

                    0x6 => {
                        // println!("Instrução ORI");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as i32;
                        let imm_i = self.instruction.imm_i;
                        let rd = rs1 | imm_i;
                        self.breg.set_reg(self.instruction.rd, rd);
                        
                    }
                    0x7 => {
                        // println!("Instrução ANDI");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as i32;
                        let imm_i = self.instruction.imm_i;
                        let rd = rs1 & imm_i;
                        self.breg.set_reg(self.instruction.rd, rd);
                    }
                    _ => {
                        println!("Instrução não implementada");
                    }
                }
            }
            0x3 => {
                // println!("Instrução do tipo S");
                match self.instruction.funct3 {
                    0x0 => {
                        // println!("Instrução LB");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as i32;
                        let imm_i = self.instruction.imm_i;
                        let address = rs1 + imm_i;
                        let value = self.memory.read_data_word(address as usize);
                        let rd = value as i8 as i32;
                        self.breg.set_reg(self.instruction.rd, rd);
                    }
                    0x1 => {
                        // println!("Instrução LH");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as i32;
                        let imm_i = self.instruction.imm_i;
                        let address = rs1 + imm_i;
                        let value = self.memory.read_data_word(address as usize);
                        let rd = value as i16 as i32;
                        self.breg.set_reg(self.instruction.rd, rd);
                    }
                    0x2 => {
                        // println!("Instrução LW");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as i32;
                        let imm_i = self.instruction.imm_i;
                        let address = rs1 + imm_i;
                        let value = self.memory.read_data_word(address as usize);
                        let rd = value as i32;
                        self.breg.set_reg(self.instruction.rd, rd);
                    }
                    0x4 => {
                        // println!("Instrução LBU");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as u32;
                        let imm_i = self.instruction.imm_i as u32;
                        let address = rs1 + imm_i;
                        let value = self.memory.read_data_word(address as usize);
                        let rd = value as u8 as u32;
                        self.breg.set_reg(self.instruction.rd, rd as i32);
                    }
                    0x5 => {
                        // println!("Instrução LHU");
                        let rs1 = self.breg.get_reg(self.instruction.rs1);
                        let imm_i = self.instruction.imm_i;
                        let address = rs1 + imm_i;
                        let value = self.memory.read_data_word(address as usize);
                        let rd = value as u16 as u32;
                        self.breg.set_reg(self.instruction.rd, rd as i32);
                    }
                    _ => {
                        println!("Instrução não implementada");
                    }
                }
            }
            0x23 => {
                // println!("Instrução do tipo S");
                match self.instruction.funct3 {
                    0x0 => {
                        // println!("Instrução SB");
                        let rs1 = self.breg.get_reg(self.instruction.rs1);
                        let rs2 = self.breg.get_reg(self.instruction.rs2) as u8;
                        let imm_s = self.instruction.imm_s;
                        let address = rs1 + imm_s;
                        self.memory.write_data_word(address as u32, rs2 as u32);
                    }
                    0x1 => {
                        // println!("Instrução SH");
                        let rs1 = self.breg.get_reg(self.instruction.rs1);
                        let rs2 = self.breg.get_reg(self.instruction.rs2) as u16;
                        let imm_s = self.instruction.imm_s;
                        let address = rs1 + imm_s;
                        self.memory.write_data_word(address as u32, rs2 as u32);
                    }
                    0x2 => {
                        // println!("Instrução SW");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as i32;
                        let rs2 = self.breg.get_reg(self.instruction.rs2) as i32;
                        let imm_s = self.instruction.imm_s;
                        let address = rs1 + imm_s;
                        self.memory.write_data_word(address as u32, rs2 as u32);
                    }
                    _ => {
                        println!("Instrução não implementada");
                    }
                }
            }
            0x63 => {
                // println!("Instrução do tipo B");
                match self.instruction.funct3 {
                    0x0 => {
                        // println!("Instrução BEQ");
                        let rs1 = self.breg.get_reg(self.instruction.rs1);
                        let rs2 = self.breg.get_reg(self.instruction.rs2);
                        let imm_b = self.instruction.imm_b;
                        if rs1 == rs2 {
                            self.pc = (self.pc as i32 + imm_b) as u32;
                        }
                    }
                    0x1 => {
                        // println!("Instrução BNE");
                        let rs1 = self.breg.get_reg(self.instruction.rs1);
                        let rs2 = self.breg.get_reg(self.instruction.rs2);
                        let imm_b = self.instruction.imm_b;
                        if rs1 != rs2 {
                            self.pc = (self.pc as i32 + imm_b) as u32;
                        }
                    }
                    0x4 => {
                        // println!("Instrução BLT");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as i32;
                        let rs2 = self.breg.get_reg(self.instruction.rs2) as i32;
                        let imm_b = self.instruction.imm_b;
                        if rs1 < rs2 {
                            self.pc = (self.pc as i32 + imm_b) as u32;
                        }
                    }
                    0x5 => {
                        // println!("Instrução BGE");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as i32;
                        let rs2 = self.breg.get_reg(self.instruction.rs2) as i32;
                        let imm_b = self.instruction.imm_b;
                        if rs1 >= rs2 {
                            self.pc = (self.pc as i32 + imm_b) as u32;
                        }
                    }
                    0x6 => {
                        // println!("Instrução BLTU");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as u32;
                        let rs2 = self.breg.get_reg(self.instruction.rs2) as u32;
                        let imm_b = self.instruction.imm_b;
                        if rs1 < rs2 {
                            self.pc = (self.pc as i32 + imm_b) as u32;
                        }
                    }
                    _ => {
                        println!("Instrução não implementada");
                    }
                }
            }
            //tipu U e J
            0x17 => {
                println!("Instrução AUIPC");
                let imm_u = (self.instruction.imm_u << 12) as u32;
                let mut rd = self.instruction.rd;
                
                self.breg.set_reg(rd, (imm_u + (self.pc * 4)) as i32);
                println!("valor de rd: {:08x}", self.breg.get_reg(rd));
            }
            0x37 => {
                // lui 
                let imm_u = (self.instruction.imm_u) << 12;
                let rd = self.instruction.rd;
                self.breg.set_reg(rd, imm_u);
            }
            0x6F => {
                // jal
                let imm_j = self.instruction.imm_j;
                let rd = self.instruction.rd;
                self.breg.set_reg(rd, self.pc as i32 + 4);
                self.pc = (self.pc as i32 + imm_j) as u32;
            }
            0x67 => {
                // jalr
                let imm_i = self.instruction.imm_i;
                let rs1 = self.breg.get_reg(self.instruction.rs1);
                let rd = self.instruction.rd;
                self.breg.set_reg(rd, self.pc as i32 + 4);
                self.pc = (rs1 + imm_i) as u32;
            }
            //Instruções de sistema
            0x73 => {
                // println!("Instrução do tipo I");
                match self.instruction.funct3 {
                    0x0 => {
                        println!("Instrução ECALL");
                        match self.breg.get_reg(17) {
                            1 => {
                                println!("Print inteiro");
                                println!("{}", self.breg.get_reg(10));
                            }
                            4 => {
                                println!("Print string");
                                let mut address = self.breg.get_reg(10) as usize;
                                let mut value = self.memory.read_data_word(address);
                                while value != 0 {
                                    print!("{}", value as u8 as char);
                                    address += 1;
                                    value = self.memory.read_data_word(address);
                                }
                            }
                            10 => {
                                println!("Fim do programa");
                                process::exit(0);
                            }
                            11 => {
                                println!("Print char");
                                let value = self.breg.get_reg(10);
                                print!("{}", value as u8 as char);
                            }
                            64 => {
                                println!("Print String");
                                // Write to a filedescriptor from a buffer
                                let mut address = self.breg.get_reg(10) as usize;
                                println!("address: {}", address);
                                let mut value = self.memory.read_data_word(address);
                                println!("value: {}", value);
                                let mut char = (value & 0xFF) as u8 as char;
                                while char != '\0' {
                                    print!("{}", char);
                                    char = (value & 0xFF00) as u8 as char;
                                    print!("{}", char);
                                    char = (value & 0xFF0000) as u8 as char;
                                    print!("{}", char);
                                    char = (value & 0xFF000000) as u8 as char;
                                    print!("{}", char);
                                    address += 1;
                                    value = self.memory.read_data_word(address);
                                    char = (value & 0xFF) as u8 as char;
                                }

                            }
                            93 => {
                                process::exit(self.breg.get_reg(10) as i32);
                            }
                            _ => {
                                println!("Instrução não implementada");
                            }

                        }
                        
                    }
                    _ => {
                        println!("Instrução não implementada");
                    }
                }
            }
            
            _ => {
                println!("Instrução não implementada");
            }
        }
    }
}