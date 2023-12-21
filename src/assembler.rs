#[derive(Copy, Clone)]
pub enum Register {
    A,
    B,
    C,
    D,
    H,
    Z,
}

pub enum RegisterOrImm {
    Reg(Register),
    Imm8(u8),
    Imm16(u16),
}

pub struct Assembler {
    buffer: Vec<u8>,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler { buffer: Vec::new() }
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.buffer.clone()
    }

    pub fn mw(&mut self, dest: Register, src: RegisterOrImm) {
        match src {
            RegisterOrImm::Reg(reg) => {
                self.buffer.push(0x08 | dest as u8);
                self.buffer.push(reg as u8);
            }
            RegisterOrImm::Imm8(value) => {
                self.buffer.push(dest as u8);
                self.buffer.push(value);
            }
            _ => panic!("invalid source operand"),
        }
    }

    pub fn lw(&mut self, dest: Register, src: Option<u16>) {
        match src {
            Some(addr) => {
                self.buffer.push(0x01 << 4 | dest as u8);
                self.emit_u16(addr);
            }
            None => {
                self.buffer.push(0x01 << 4 | 0x08 | dest as u8);
            }
        }
    }

    pub fn sw(&mut self, dest: Option<u16>, src: Register) {
        match dest {
            Some(addr) => {
                self.buffer.push(0x02 << 4 | src as u8);
                self.emit_u16(addr);
            }
            None => {
                self.buffer.push(0x02 << 4 | 0x08 | src as u8);
            }
        }
    }

    pub fn push(&mut self, src: RegisterOrImm) {
        match src {
            RegisterOrImm::Reg(reg) => {
                self.buffer.push(0x03 << 4 | 0x08 | reg as u8);
            }
            RegisterOrImm::Imm8(value) => {
                self.buffer.push(0x03 << 4);
                self.buffer.push(value);
            }
            _ => panic!("invalid source operand"),
        }
    }

    pub fn pop(&mut self, dest: Register) {
        self.buffer.push(0x04 << 4 | 0x08 | dest as u8);
    }

    pub fn lda(&mut self, addr: u16) {
        self.buffer.push(5 << 4);
        self.emit_u16(addr);
    }

    pub fn jnz(&mut self, src: RegisterOrImm) {
        match src {
            RegisterOrImm::Reg(reg) => {
                self.buffer.push(0x06 << 4 | 0x08 | reg as u8);
            }
            RegisterOrImm::Imm8(value) => {
                self.buffer.push(0x06 << 4);
                self.buffer.push(value);
            }
            _ => panic!("invalid source operand"),
        }
    }

    pub fn inb(&mut self, dest: Register, src: RegisterOrImm) {
        match src {
            RegisterOrImm::Reg(reg) => {
                self.buffer.push(0x07 << 4 | 0x08 | dest as u8);
                self.buffer.push(reg as u8);
            }
            RegisterOrImm::Imm8(value) => {
                self.buffer.push(0x07 << 4 | dest as u8);
                self.buffer.push(value);
            }
            _ => panic!("invalid source operand"),
        }
    }

    pub fn outb(&mut self, dest: RegisterOrImm, src: Register) {
        match dest {
            RegisterOrImm::Reg(reg) => {
                self.buffer.push(0x08 << 4 | 0x08 | src as u8);
                self.buffer.push(reg as u8);
            }
            RegisterOrImm::Imm8(value) => {
                self.buffer.push(0x08 << 4 | src as u8);
                self.buffer.push(value);
            }
            _ => panic!("invalid destination operand"),
        }
    }

    pub fn add(&mut self, dest: Register, src: RegisterOrImm) {
        match src {
            RegisterOrImm::Reg(reg) => {
                self.buffer.push(0x09 << 4 | 0x08 | dest as u8);
                self.buffer.push(reg as u8);
            }
            RegisterOrImm::Imm8(value) => {
                self.buffer.push(0x09 << 4 | dest as u8);
                self.buffer.push(value);
            }
            _ => panic!("invalid source operand"),
        }
    }

    pub fn adc(&mut self, dest: Register, src: RegisterOrImm) {
        match src {
            RegisterOrImm::Reg(reg) => {
                self.buffer.push(0x0A << 4 | 0x08 | dest as u8);
                self.buffer.push(reg as u8);
            }
            RegisterOrImm::Imm8(value) => {
                self.buffer.push(0x0A << 4 | dest as u8);
                self.buffer.push(value);
            }
            _ => panic!("invalid source operand"),
        }
    }

    pub fn and(&mut self, dest: Register, src: RegisterOrImm) {
        match src {
            RegisterOrImm::Reg(reg) => {
                self.buffer.push(0x0B << 4 | 0x08 | dest as u8);
                self.buffer.push(reg as u8);
            }
            RegisterOrImm::Imm8(value) => {
                self.buffer.push(0x0B << 4 | dest as u8);
                self.buffer.push(value);
            }
            _ => panic!("invalid source operand"),
        }
    }

    pub fn or(&mut self, dest: Register, src: RegisterOrImm) {
        match src {
            RegisterOrImm::Reg(reg) => {
                self.buffer.push(0x0C << 4 | 0x08 | dest as u8);
                self.buffer.push(reg as u8);
            }
            RegisterOrImm::Imm8(value) => {
                self.buffer.push(0x0C << 4 | dest as u8);
                self.buffer.push(value);
            }
            _ => panic!("invalid source operand"),
        }
    }

    pub fn nor(&mut self, dest: Register, src: RegisterOrImm) {
        match src {
            RegisterOrImm::Reg(reg) => {
                self.buffer.push(0x0D << 4 | 0x08 | dest as u8);
                self.buffer.push(reg as u8);
            }
            RegisterOrImm::Imm8(value) => {
                self.buffer.push(0x0D << 4 | dest as u8);
                self.buffer.push(value);
            }
            _ => panic!("invalid source operand"),
        }
    }

    pub fn cmp(&mut self, left: Register, right: RegisterOrImm) {
        match right {
            RegisterOrImm::Reg(reg) => {
                self.buffer.push(0x0E << 4 | 0x08 | left as u8);
                self.buffer.push(reg as u8);
            }
            RegisterOrImm::Imm8(value) => {
                self.buffer.push(0x0E << 4 | left as u8);
                self.buffer.push(value);
            }
            _ => panic!("invalid source operand"),
        }
    }

    pub fn sbb(&mut self, dest: Register, src: RegisterOrImm) {
        match src {
            RegisterOrImm::Reg(reg) => {
                self.buffer.push(0x0F << 4 | 0x08 | dest as u8);
                self.buffer.push(reg as u8);
            }
            RegisterOrImm::Imm8(value) => {
                self.buffer.push(0x0F << 4 | dest as u8);
                self.buffer.push(value);
            }
            _ => panic!("invalid source operand"),
        }
    }

    fn emit_u16(&mut self, value: u16) {
        self.buffer.push((value & 0xFF) as u8);
        self.buffer.push((value >> 8) as u8);
    }
}
