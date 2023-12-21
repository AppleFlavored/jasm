use jasm::{Assembler, Register, RegisterOrImm};
use std::{fs::File, io::Write};

fn main() {
    let mut assembler = Assembler::new();
    assembler.mw(Register::A, RegisterOrImm::Imm8(3));
    assembler.mw(Register::B, RegisterOrImm::Imm8(4));
    assembler.add(Register::A, RegisterOrImm::Reg(Register::B));

    assembler.lda(0xcafe);

    let bytes = assembler.bytes();

    let mut file = File::create("output.bin").expect("open file");
    file.write(&bytes).expect("write bytes");
}
