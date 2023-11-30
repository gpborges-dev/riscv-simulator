
    const ALU_OP_ADD: u8 = 0b00000000; // 0
    const ALU_OP_SLL: u8 = 0b00000001; // 1
    const ALU_OP_XOR: u8 = 0b00000100; // 4
    const ALU_OP_OR: u8 = 0b00000110; // 6
    const ALU_OP_AND: u8 = 0b00000111; // 7
    const ALU_OP_SRL: u8 = 0b00000101; // 5
    const ALU_OP_SUB: u8 = 0b00001000; // 8
    const ALU_OP_SRA: u8 = 0b00001101; // 13
    const ALU_OP_SLT: u8 = 0b00000010; // 2
    const ALU_OP_SLTU: u8 = 0b00000011; // 3
    const ALU_OP_SLLI: u8 = 0b00010001; // 17
    const ALU_OP_SRLI: u8 = 0b00010101; // 21
    const ALU_OP_SRAI: u8 = 0b00011101; // 29
    const ALU_OP_ADDI: u8 = 0b00100000; // 32
    const ALU_OP_XORI: u8 = 0b00100100; // 36
    const ALU_OP_ORI: u8 = 0b00100110; // 38
    const ALU_OP_ANDI: u8 = 0b00100111; // 39
    const ALU_OP_SLTI: u8 = 0b00100010; // 34
    const ALU_OP_SLTIU: u8 = 0b00100011; // 35
    
    pub struct ALU {
    
    }
    impl ALU{
        pub fn new() -> Self {
            ALU {}
        }
    
        pub fn calculate(&self, in1: u32, in2: u32, op: u8) -> u32 {
            calculate(in1, in2, op)
        }
    }
    fn calculate(in1: u32, in2: u32, op: u8) -> u32 {
        match op {
            ALU_OP_ADD => in1.wrapping_add(in2),
            ALU_OP_SLL => in1.wrapping_shl(in2),
            ALU_OP_XOR => in1 ^ in2,
            ALU_OP_OR => in1 | in2,
            ALU_OP_AND => in1 & in2,
            ALU_OP_SRL => in1.wrapping_shr(in2), // Shift Right Logical
            ALU_OP_SUB => in1.wrapping_sub(in2),
            ALU_OP_SRA => (in1 as i32).wrapping_shr(in2 as u32) as u32, // Shift Right Arithmetic
            ALU_OP_SLT => if (in1 as i32) < (in2 as i32) { 1 } else { 0 }, // Set If Less Than (Signed)
            ALU_OP_SLTU => if in1 < in2 { 1 } else { 0 }, // Set If Less Than (Unsigned)
            ALU_OP_SLLI => in1.wrapping_shl(in2),
            ALU_OP_SRLI => in1.wrapping_shr(in2),
            ALU_OP_SRAI => (in1 as i32).wrapping_shr(in2 as u32) as u32,
            ALU_OP_ADDI => in1.wrapping_add(in2),
            ALU_OP_XORI => in1 ^ in2,
            ALU_OP_ORI => in1 | in2,
            ALU_OP_ANDI => in1 & in2,
            ALU_OP_SLTI => if (in1 as i32) < (in2 as i32) { 1 } else { 0 },
            ALU_OP_SLTIU => if in1 < in2 { 1 } else { 0 },
    
            _ => {
                println!("ALU: cannot recognize the operating instruction");
                0
            }
        }
    }


