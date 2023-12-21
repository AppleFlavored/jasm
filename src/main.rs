use assembler::{Assembler, Register, RegisterOrImm};
use std::{fs::File, io::Write};

mod assembler;

fn main() {
    let mut assembler = Assembler::new();
    assembler.mw(Register::A, RegisterOrImm::Imm8(3));
    assembler.mw(Register::B, RegisterOrImm::Imm8(4));
    assembler.add(Register::A, RegisterOrImm::Reg(Register::B));

    let bytes = assembler.bytes();

    let mut file = File::create("output.bin").expect("open file");
    file.write(&bytes).expect("write bytes");
}
