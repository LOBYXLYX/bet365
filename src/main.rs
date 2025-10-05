use std::fs;
use std::time::Instant;
mod disassembler;
mod bytecode;


fn main() {
    let bytec: String = fs::read_to_string("src/bytecode/bytecode.txt").expect("not found");

    let start = Instant::now();
    let mut disasm: disassembler::disassembler::Disassembler = disassembler::disassembler::Disassembler::new(bytec);
    disasm.execute();

    println!("disassemble took: {:?}", start.elapsed());
}
