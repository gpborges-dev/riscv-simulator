mod assembler;

fn main() {
    let args : Vec<String> = std::env::args().collect();
    assembler::assemble(&args[1]);
}