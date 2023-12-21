# JASM
An assembler for the [JDH-8](https://github.com/jdah/jdh-8) architecture written in Rust.

## Assembler API
The assembler is available as a library on [crates.io](https://crates.io/crates/jasm).
Here is a basic example of how to use the API:
```rs
fn main() {
    let mut assembler = Assembler::new();

    // mw a, 4
    // mw b, 3
    // add a, b
    assembler.mw(Register::A, RegisterOrImm::Imm8(4));
    assembler.mw(Register::B, RegisterOrImm::Imm8(3));
    assembler.add(Register::A, RegisterOrImm::Reg(Register::B));

    let bytes = assembler.bytes();
}
```

## License
JASM is licensed under the [MIT License](https://opensource.org/license/mit/).
See [LICENSE](LICENSE) for more details.