use std::io::Write;

pub struct Reg {
    pub regs: [u32; 33],
}

impl Reg {
    pub fn new() -> Self {
        let mut regs = [0; 33];
        for i in 0..33 {
            regs[i] = 0;
        }
        Reg { regs }
    }

    pub fn get_reg(&self, addr: u8) -> u32 {
        if addr > 32 {
            writeln!(std::io::stderr(), "Reg: invalid register address").unwrap();
            std::process::exit(1);
        }
        self.regs[addr as usize]
    }

    pub fn get_reg_high(&self, addr: u8) -> u16 {
        if addr > 32 {
            writeln!(std::io::stderr(), "Reg: invalid register address").unwrap();
            std::process::exit(1);
        }
        (self.regs[addr as usize] >> 16) as u16
    }

    pub fn get_reg_low(&self, addr: u8) -> u16 {
        if addr > 32 {
            writeln!(std::io::stderr(), "Reg: invalid register address").unwrap();
            std::process::exit(1);
        }
        (self.regs[addr as usize] & 0x0000FFFF) as u16
    }

    pub fn set_reg(&mut self, addr: u8, val: u32) {
        if addr > 32 {
            writeln!(std::io::stderr(), "Reg: invalid register address").unwrap();
            std::process::exit(1);
        }
        if addr == 0 {
            writeln!(std::io::stderr(), "Reg Warning: you are trying to set x0").unwrap();
            return;
        }
        self.regs[addr as usize] = val;
    }

    pub fn set_reg_high(&mut self, addr: u8, val: u16) {
        if addr > 32 {
            writeln!(std::io::stderr(), "Reg: invalid register address").unwrap();
            std::process::exit(1);
        }
        if addr == 0 {
            writeln!(std::io::stderr(), "Reg Warning: you are trying to set x0").unwrap();
            return;
        }
        let mask = (val as u32) << 16;
        self.regs[addr as usize] = (self.regs[addr as usize] & 0x0000FFFF) | mask;
    }

    pub fn set_reg_low(&mut self, addr: u8, val: u16) {
        if addr > 32 {
            writeln!(std::io::stderr(), "Reg: invalid register address").unwrap();
            std::process::exit(1);
        }
        if addr == 0 {
            writeln!(std::io::stderr(), "Reg Warning: you are trying to set x0").unwrap();
            return;
        }
        self.regs[addr as usize] = (self.regs[addr as usize] & 0xFFFF0000) | (val as u32);
    }
}
