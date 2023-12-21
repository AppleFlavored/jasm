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

    /// Moves a register or 8-bit immediate value from `src` into a register `dest`.
    ///
    /// # Panics
    ///
    /// Panics if the `src` operand is not `RegisterOrImm::Reg()` or `RegisterOrImm:Imm8()`.
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

    /// Loads a 8-bit value from memory into a register `dest`.
    /// If `src` is `Some(...)`, the value passed in `src` is used as the source address.
    /// If `src` is `None`, the value in the `HL` register is used as the source address.
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

    /// Stores an 8-bit value from a register `src` into memory.
    /// If `dest` is `Some(...)`, the value passed in `dest` is used as the destination address.
    /// If `dest` is `None`, the value in the `HL` register is used as the destination address.
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

    /// Pushes a register or 8-bit immediate value onto the stack.
    ///
    /// # Panics
    ///
    /// Panics if the `src` operand is not `RegisterOrImm::Reg()` or `RegisterOrImm:Imm8()`.
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

    /// Pops a value from the stack and loads it into a register `dest`.
    pub fn pop(&mut self, dest: Register) {
        self.buffer.push(0x04 << 4 | 0x08 | dest as u8);
    }

    /// Loads a 16-bit value from memory into the `HL` register.
    pub fn lda(&mut self, addr: u16) {
        self.buffer.push(5 << 4);
        self.emit_u16(addr);
    }

    /// Jumps to the address specified by the `HL` register if the immediate value `src` or the
    /// value in the register `src` is not zero.
    ///
    /// # Panics
    ///
    /// Panics if the `src` operand is not `RegisterOrImm::Reg()` or `RegisterOrImm:Imm8()`.
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

    /// Reads a byte from the port specified by `src` into a register `dest`.
    ///
    /// # Panics
    ///
    /// Panics if the `src` operand is not `RegisterOrImm::Reg()` or `RegisterOrImm:Imm8()`.
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

    /// Writes a byte from a register `src` to the port specified by `dest`.
    ///
    /// # Panics
    ///
    /// Panics if the `dest` operand is not `RegisterOrImm::Reg()` or `RegisterOrImm:Imm8()`.
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

    /// Adds a register or 8-bit immediate value `src` and the value in a register `dest`.
    /// The result is stored in the register `dest`.
    ///
    /// # Panics
    ///
    /// Panics if the `src` operand is not `RegisterOrImm::Reg()` or `RegisterOrImm:Imm8()`.
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

    /// Adds with carry a register or 8-bit immediate value `src` and the value in a register `dest`.
    /// The result is stored in the register `dest`.
    ///
    /// # Panics
    ///
    /// Panics if the `src` operand is not `RegisterOrImm::Reg()` or `RegisterOrImm:Imm8()`.
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

    /// Performs a bitwise AND operating on a register or 8-bit immediate value `src` and the value in a register `dest`.
    /// The result is stored in the register `dest`.
    ///
    /// # Panics
    ///
    /// Panics if the `src` operand is not `RegisterOrImm::Reg()` or `RegisterOrImm:Imm8()`.
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

    /// Performs a bitwise OR operating on a register or 8-bit immediate value `src` and the value in a register `dest`.
    /// The result is stored in the register `dest`.
    ///
    /// # Panics
    ///
    /// Panics if the `src` operand is not `RegisterOrImm::Reg()` or `RegisterOrImm:Imm8()`.
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

    /// Performs a bitwise NOR operating on a register or 8-bit immediate value `src` and the value in a register `dest`.
    /// The result is stored in the register `dest`.
    ///
    /// # Panics
    ///
    /// Panics if the `src` operand is not `RegisterOrImm::Reg()` or `RegisterOrImm:Imm8()`.
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

    /// Compares a register or 8-bit immediate value `src` and the value in a register `dest`.
    /// Sets the flags register accordingly.
    ///
    /// # Panics
    ///
    /// Panics if the `src` operand is not `RegisterOrImm::Reg()` or `RegisterOrImm:Imm8()`.
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

    /// Subtracts a register or 8-bit immediate value `src` from the value in a register `dest`.
    /// The result is stored in the register `dest`.
    ///
    /// # Panics
    ///
    /// Panics if the `src` operand is not `RegisterOrImm::Reg()` or `RegisterOrImm:Imm8()`.
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

    #[inline(always)]
    fn emit_u16(&mut self, value: u16) {
        self.buffer.push((value & 0xFF) as u8);
        self.buffer.push((value >> 8) as u8);
    }
}
