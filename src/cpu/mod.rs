//! Este módulo respresenta a CPU do processador Risc-v, onde são utilizados registradores e memória, além das funções de fetch, decode e execute.


mod mem;
mod reg;
pub use mem::Memory;
pub use reg::Breg;
pub use std::process;

// Definição da estrutura de uma instrução Risc-v
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
// Definição da estrutura da CPU, que contém referências para a memória, registradores e instrução, além do contador de programa.
pub struct Cpu<'a> {
    pub pc: u32,
    pub breg: &'a mut Breg,
    pub memory: &'a mut Memory,
    pub instruction: &'a mut RiscvInstruction,
    pub inst: u32,
}
// Definição da implementação da CPU
impl Cpu<'_> {
    /// A função new() instancia a CPU com referências para a memória, registradores e instrução.
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
    /// A função fetch() lê uma instrução da memória de texto.
    pub fn fetch(&mut self) -> () {
        
        self.inst = self.memory.read_text_word(self.pc as usize);
        
    }
    /// A função decode() decodifica uma instrução e atribui os valores aos campos da estrutura RiscvInstruction.
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
            | ((instruction >> 8) & 0xF) << 1) as i32;
    
        let imm_u = ((instruction >> 12) & 0xFFFFF) as i32;

        let imm_j = (((instruction >> 31) & 0x1) << 20
            | (instruction >> 21) & 0x3FF
            | (instruction >> 20) & 0x1
            | (instruction >> 12) & 0xFF) as i32;
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
    /// A função execute() executa uma instrução.
    pub fn execute(&mut self) {
        match self.instruction.opcode {
            0x33 => {
                // A instrução é do tipo R
                match self.instruction.funct3 {
                    0x0 => match self.instruction.funct7 {
                        0x0 => {
                            // O valor do campo funct7 é 0, então a instrução é ADD
                            println!("Instrução ADD");
                            let rs1 = self.breg.get_reg(self.instruction.rs1);
                            let rs2 = self.breg.get_reg(self.instruction.rs2);
                            let rd = rs1 + rs2;
                            self.breg.set_reg(self.instruction.rd, rd);
                        }
                        0x1 => {
                            println!("Instrução MUL");
                            //O valor do campo funct7 é 0x1, então a instrução é MUL
                            let rs1 = self.breg.get_reg(self.instruction.rs1);
                            let rs2 = self.breg.get_reg(self.instruction.rs2);
                            let rd = rs1 * rs2;
                            self.breg.set_reg(self.instruction.rd, rd);
                        }
                        0x20 => {
                            // O valor do campo funct7 é 0x20, então a instrução é SUB
                            println!("Instrução SUB");
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
                       // O valor do campo funct3 é 0x1, então a instrução é SLL
                        println!("Instrução SLL");
                        let rs1 = self.breg.get_reg(self.instruction.rs1);
                        let rs2 = self.breg.get_reg(self.instruction.rs2);
                        let rd = rs1 << rs2;
                        self.breg.set_reg(self.instruction.rd, rd);
                    }
                    0x2 => {
                        // O valor do campo funct3 é 0x2, então a instrução é SLT
                        println!("Instrução SLT");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as i32;
                        let rs2 = self.breg.get_reg(self.instruction.rs2) as i32;
                        let rd = if rs1 < rs2 { 1 } else { 0 };
                        self.breg.set_reg(self.instruction.rd, rd);
                    }
                    0x3 => {
                        //O valor do campo funct3 é 0x3, então a instrução é SLTU
                        println!("Instrução SLTU");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as u32;
                        let rs2 = self.breg.get_reg(self.instruction.rs2) as u32;
                        let rd = if rs1 < rs2 { 1 } else { 0 };
                        self.breg.set_reg(self.instruction.rd, rd);
                    }
                    0x4 => {
                        //O valor do campo funct3 é 0x4, então a instrução é XOR
                        println!("Instrução XOR");
                        let rs1 = self.breg.get_reg(self.instruction.rs1);
                        let rs2 = self.breg.get_reg(self.instruction.rs2);
                        let rd = rs1 ^ rs2;
                        self.breg.set_reg(self.instruction.rd, rd);
                    }
                    0x5 => {
                        // O valor do campo funct3 é 0x5, então a instrução é SRL
                        println!("Instrução SRL");
                        let rs1 = self.breg.get_reg(self.instruction.rs1);
                        let rs2 = self.breg.get_reg(self.instruction.rs2);
                        let rd = rs1 >> rs2;
                        self.breg.set_reg(self.instruction.rd, rd);
                    }
                    0x6 => {
                        // O valor do campo funct3 é 0x6, então a instrução é OR
                        println!("Instrução OR");
                        let rs1 = self.breg.get_reg(self.instruction.rs1);
                        let rs2 = self.breg.get_reg(self.instruction.rs2);
                        let rd = rs1 | rs2;
                        self.breg.set_reg(self.instruction.rd, rd);
                    }
                    0x7 => {    
                        // O valor do campo funct3 é 0x7, então a instrução é AND
                        println!("Instrução AND");
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
                // A instrução é do tipo I
                match self.instruction.funct3 {
                    0x0 => {
                        // O valor do campo funct3 é 0x0, então a instrução é ADDI
                        println!("Instrução ADDI");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as i32;
                        let mut imm_i = self.instruction.imm_i;
                        if(imm_i >> 11) == 1 {
                            imm_i = ((imm_i & 0x800) - (imm_i & 0x7FF)) * (-1);
                        }
                        
                        let rd = rs1 + imm_i;
                        
                        self.breg.set_reg(self.instruction.rd, rd);
                    }
                    0x1 => {
                        // O valor do campo funct3 é 0x1, então a instrução é SLLI
                        println!("Instrução SLLI");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as i32;
                        let mut imm_i = self.instruction.imm_i;
                        if(imm_i >> 11) == 1 {
                            imm_i = ((imm_i & 0x800) - (imm_i & 0x7FF)) * (-1);
                        }
                        let rd = rs1 << imm_i;
                        self.breg.set_reg(self.instruction.rd, rd);
                        
                    }
                    0x2 => {
                        // O valor do campo funct3 é 0x2, então a instrução é SLTI
                        println!("Instrução SLTI");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as i32;
                        let mut imm_i = self.instruction.imm_i;
                        if(imm_i >> 11) == 1 {
                            imm_i = ((imm_i & 0x800) - (imm_i & 0x7FF)) * (-1);
                        }
                        let rd = if rs1 < imm_i { 1 } else { 0 };
                        self.breg.set_reg(self.instruction.rd, rd);
                    }
                    0x3 => {
                        // O valor do campo funct3 é 0x3, então a instrução é SLTIU
                        println!("Instrução SLTIU");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as u32;
                        let imm_i = self.instruction.imm_i as u32;
                        let rd = if rs1 < imm_i { 1 } else { 0 };
                        self.breg.set_reg(self.instruction.rd, rd as i32);
                    }
                    0x4 => {
                        // O valor do campo funct3 é 0x4, então a instrução é XORI
                        println!("Instrução XORI");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as i32;
                        let mut imm_i = self.instruction.imm_i;
                        if(imm_i >> 11) == 1 {
                            imm_i = ((imm_i & 0x800) - (imm_i & 0x7FF)) * (-1);
                        }
                        let rd = rs1 ^ imm_i;
                        self.breg.set_reg(self.instruction.rd, rd);
                    }
                    0x5 => {
                        // O valor do campo funct3 é 0x5, então a instrução é SRLI ou SRAI
                        println!("Instrução SRLI");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as i32;
                        let mut imm_i = self.instruction.imm_i;
                        if(imm_i >> 11) == 1 {
                            imm_i = ((imm_i & 0x800) - (imm_i & 0x7FF)) * (-1);
                        }
                        let rd = rs1 >> imm_i;
                        self.breg.set_reg(self.instruction.rd, rd);
                    }

                    0x6 => {
                        // O valor do campo funct3 é 0x6, então a instrução é ORI
                        println!("Instrução ORI");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as i32;
                        let mut imm_i = self.instruction.imm_i;
                        if(imm_i >> 11) == 1 {
                            imm_i = ((imm_i & 0x800) - (imm_i & 0x7FF)) * (-1);
                        }
                        let rd = rs1 | imm_i;
                        self.breg.set_reg(self.instruction.rd, rd);
                        
                    }
                    0x7 => {
                        // O valor do campo funct3 é 0x7, então a instrução é ANDI
                        println!("Instrução ANDI");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as i32;
                        let mut imm_i = self.instruction.imm_i;
                        if(imm_i >> 11) == 1 {
                            imm_i = ((imm_i & 0x800) - (imm_i & 0x7FF)) * (-1);
                        }
                        let rd = rs1 & imm_i;
                        self.breg.set_reg(self.instruction.rd, rd);
                    }
                    _ => {
                        println!("Instrução não implementada");
                    }
                }
            }
            0x3 => {
                // A instrução é do tipo I
                match self.instruction.funct3 {
                    0x0 => {
                        // O valor do campo funct3 é 0x0, então a instrução é LB
                        println!("Instrução LB");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as i32;
                        let mut imm_i = self.instruction.imm_i;
                        
                        if(imm_i >> 11) == 1 {
                            imm_i = ((imm_i & 0x800) - (imm_i & 0x7FF)) * (-1);
                        }
                        let mut address = rs1 + imm_i;
                        address = address/4 - 2048;

                        let value = self.memory.read_data_word(address as usize);
                        let rd = value as i8 as i32;
                        self.breg.set_reg(self.instruction.rd, rd);
                    }
                    0x1 => {
                        // O valor do campo funct3 é 0x1, então a instrução é LH
                        println!("Instrução LH");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as i32;
                        let mut imm_i = self.instruction.imm_i;
                        if(imm_i >> 11) == 1 {
                            imm_i = ((imm_i & 0x800) - (imm_i & 0x7FF)) * (-1);
                        }
                        let mut address = rs1 + imm_i;
                        address = address/4 - 2048;
                        let value = self.memory.read_data_word(address as usize);
                        let rd = value as i16 as i32;
                        self.breg.set_reg(self.instruction.rd, rd);
                    }
                    0x2 => {
                        // O valor do campo funct3 é 0x2, então a instrução é LW
                        println!("Instrução LW");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as i32;
                        let mut imm_i = self.instruction.imm_i;
                        if(imm_i >> 11) == 1 {
                            imm_i = ((imm_i & 0x800) - (imm_i & 0x7FF)) * (-1);
                        }
                        let mut address = rs1 + imm_i;
                        address = address/4 - 2048;
                        println!("address: {:x}", address);
                        let value = self.memory.read_data_word(address as usize);
                        let rd = value as i32;
                        self.breg.set_reg(self.instruction.rd, rd);
                    }
                    0x4 => {
                        // O valor do campo funct3 é 0x4, então a instrução é LBU
                        println!("Instrução LBU");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as u32;
                        let imm_i = self.instruction.imm_i as u32;
                        let address = rs1 + imm_i;
                        let value = self.memory.read_data_word(address as usize);
                        let rd = value as u8 as u32;
                        self.breg.set_reg(self.instruction.rd, rd as i32);
                    }
                    0x5 => {
                        // O valor do campo funct3 é 0x5, então a instrução é LHU
                        println!("Instrução LHU");
                        let rs1 = self.breg.get_reg(self.instruction.rs1);
                        let imm_i = self.instruction.imm_i;
                        let mut address: i32 = rs1 + imm_i;
                        address = address/4 - 2048;
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
                // A instrução é do tipo S
                match self.instruction.funct3 {
                    0x0 => {
                        // O valor do campo funct3 é 0x0, então a instrução é SB
                        println!("Instrução SB");
                        let rs1 = self.breg.get_reg(self.instruction.rs1);
                        let rs2 = self.breg.get_reg(self.instruction.rs2) as u8;
                        let mut imm_s = self.instruction.imm_s;
                        if(imm_s >> 11) == 1 {
                            imm_s = ((imm_s & 0x800) - (imm_s & 0x7FF)) * (-1);
                        }
                        let mut address = rs1 + imm_s;
                        address = address/4 - 2048;
                        self.memory.write_data_word(address as u32, rs2 as u32);
                    }
                    0x1 => {
                        // O valor do campo funct3 é 0x1, então a instrução é SH
                        println!("Instrução SH");
                        let rs1 = self.breg.get_reg(self.instruction.rs1);
                        let rs2 = self.breg.get_reg(self.instruction.rs2) as u16;
                        let mut imm_s = self.instruction.imm_s;
                        if(imm_s >> 11) == 1 {
                            imm_s = ((imm_s & 0x800) - (imm_s & 0x7FF)) * (-1);
                        }
                        let mut address = rs1 + imm_s;
                        address = address/4 - 2048;
                        self.memory.write_data_word(address as u32, rs2 as u32);
                    }
                    0x2 => {
                        // O valor do campo funct3 é 0x2, então a instrução é SW
                        println!("Instrução SW");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as i32;
                        let rs2 = self.breg.get_reg(self.instruction.rs2) as i32;
                        let mut imm_s = self.instruction.imm_s;
                        if(imm_s >> 11) == 1 {
                            imm_s = ((imm_s & 0x800) - (imm_s & 0x7FF)) * (-1);
                        }
                        let mut address = rs1 + imm_s;
                        address = address/4 - 2048;
                        self.memory.write_data_word(address as u32, rs2 as u32);
                    }
                    _ => {
                        println!("Instrução não implementada");
                    }
                }
            }
            0x63 => {
                // A instrução é do tipo B
                match self.instruction.funct3 {
                    0x0 => {
                        // O valor do campo funct3 é 0x0, então a instrução é BEQ
                        println!("Instrução BEQ");
                        let rs1 = self.breg.get_reg(self.instruction.rs1);
                        let rs2 = self.breg.get_reg(self.instruction.rs2);
                        let mut imm_b = self.instruction.imm_b;
                        if(imm_b >> 11) == 1 {
                            imm_b = ((imm_b & 0x800) - (imm_b & 0x7FF)) * (-1);
                        }
                    
                        if rs1 == rs2 {
                            self.pc = (self.pc as i32 + (imm_b / 4)) as u32;
                            
                        }else{
                            self.pc += 1;
                        }
                    }
                    0x1 => {
                        // O valor do campo funct3 é 0x1, então a instrução é BNE
                        println!("Instrução BNE");
                        let rs1 = self.breg.get_reg(self.instruction.rs1);
                        let rs2 = self.breg.get_reg(self.instruction.rs2);
                        let mut imm_b = self.instruction.imm_b;
                        if(imm_b >> 11) == 1 {
                            imm_b = ((imm_b & 0x800) - (imm_b & 0x7FF)) * (-1);
                        }
                        if rs1 != rs2 {
                            self.pc = (self.pc as i32 + (imm_b / 4)) as u32;
                            
                        }else{
                            self.pc += 1;
                            
                        }
                    }
                    0x4 => {
                        // O valor do campo funct3 é 0x4, então a instrução é BLT
                        println!("Instrução BLT");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as i32;
                        let rs2 = self.breg.get_reg(self.instruction.rs2) as i32;
                        let mut imm_b = self.instruction.imm_b;
                        if(imm_b >> 11) == 1 {
                            imm_b = ((imm_b & 0x800) - (imm_b & 0x7FF)) * (-1);
                        }
                        if rs1 < rs2 {
                            self.pc = (self.pc as i32 + (imm_b / 4)) as u32;
                        }else{
                            self.pc += 1;
                        }
                    }
                    0x5 => {
                        // O valor do campo funct3 é 0x5, então a instrução é BGE
                        println!("Instrução BGE");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as i32;
                        let rs2 = self.breg.get_reg(self.instruction.rs2) as i32;
                        let mut imm_b = self.instruction.imm_b;
                        if(imm_b >> 11) == 1 {
                            imm_b = ((imm_b & 0x800) - (imm_b & 0x7FF)) * (-1);
                        }
                        if rs1 >= rs2 {
                            self.pc = (self.pc as i32 + (imm_b / 4)) as u32;
                        }else{
                            self.pc += 1;
                        }
                    }
                    0x6 => {
                        // O valor do campo funct3 é 0x6, então a instrução é BLTU
                        println!("Instrução BLTU");
                        let rs1 = self.breg.get_reg(self.instruction.rs1) as u32;
                        let rs2 = self.breg.get_reg(self.instruction.rs2) as u32;
                        let imm_b = self.instruction.imm_b;
                        if rs1 < rs2 {
                            self.pc = (self.pc as i32 + (imm_b / 4)) as u32;
                        }else{
                            self.pc += 1;
                        }
                    }
                    _ => {
                        println!("Instrução não implementada");
                    }
                }
            }
            //tipu U e J
            0x17 => {
                // O valor do opcode é 0x17, então a instrução é AUIPC
                println!("Instrução AUIPC");
                let imm_u = (self.instruction.imm_u << 12) as u32;
                let mut rd = self.instruction.rd;
                
                self.breg.set_reg(rd, (imm_u + (self.pc * 4)) as i32);
                println!("valor de rd: {:08x}", self.breg.get_reg(rd));
            }
            0x37 => {
                // O valor do opcode é 0x37, então a instrução é LUI
                println!("Instrução LUI");
                let imm_u = (self.instruction.imm_u) << 12;
                let rd = self.instruction.rd;
                self.breg.set_reg(rd, imm_u);
            }
            0x6F => {
                // O valor do opcode é 0x6F, então a instrução é JAL
                println!("Instrução JAL");
                let mut imm_j = self.instruction.imm_j;
                if(imm_j >> 19) == 1 {
                    imm_j = ((imm_j & 0x800) - (imm_j & 0x7FF)) * (-1);
                }
                
                let rd = self.instruction.rd;
                self.breg.set_reg(rd, (self.pc * 4) as i32);
                self.pc = (self.pc as i32 + (imm_j / 4)) as u32;
                
            }
            0x67 => {
                // O valor do opcode é 0x67, então a instrução é JALR
                println!("Instrução JALR");
                let imm_i = self.instruction.imm_i;
                let rs1 = self.breg.get_reg(self.instruction.rs1);
                let rd = self.instruction.rd;
                self.breg.set_reg(rd, self.pc as i32 + 4);
                self.pc = (rs1 + imm_i) as u32;
            }
            //Instruções de sistema
            0x73 => {
                // O valor do opcode é 0x73, então a instrução é SYSTEM
                match self.instruction.funct3 {
                    0x0 => {
                        println!("Instrução ECALL");
                        match self.breg.get_reg(17) {
                            1 => {
                                //ecall print inteiro
                                println!("Print inteiro");
                                println!("{}", self.breg.get_reg(10));
                            }
                            4 => {
                                
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
                                //ecall print char
                                let value = self.breg.get_reg(10);
                                print!("{}", value as u8 as char);
                            }
                            64 => {
                                //ecall de print string
                                // Write to a filedescriptor from a buffer
                                println!("Print String");
                                let mut address = self.breg.get_reg(11) as usize;
                                address = address/4 - 2048;
                                let mut size = self.breg.get_reg(12) as usize;
                                println!("size: {}", size);
                                println!("address: {}", address);
                                let mut value = self.memory.read_data_word(address);
                                println!("value: {}", value);
                                
                                while(size > 0){
                                    print!("{}", ((value >> 24) & 0x000000FF) as u8 as char);
                                    size -= 1;
                                    if(size <= 0){
                                        break;
                                    }
                                    print!("{}", ((value >> 16) & 0xFF) as u8 as char);
                                    size -= 1;
                                    if(size <= 0){
                                        break;
                                    }
                                    print!("{}", ((value >> 8) & 0xFF) as u8 as char);
                                    size -= 1;
                                    if(size <= 0){
                                        break;
                                    }
                                    print!("{}", (value & 0xFF) as u8 as char);
                                    size -= 1;
                                    if(size <= 0){
                                        break;
                                    }
                                    address += 1;
                                    value = self.memory.read_data_word(address);
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