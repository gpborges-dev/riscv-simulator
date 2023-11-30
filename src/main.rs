use std::fs::File;
use std::io::{self, BufRead};

mod assembler;

fn main() -> io::Result<()>{
    // let args : Vec<String> = std::env::args().collect();
    // assembler::assemble(&args[1]);

    let file = File::open("arquivo.txt")?;
    let reader = io::BufReader::new(file);

    // Itera sobre cada linha do arquivo e imprime na tela
    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}