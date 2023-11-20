use std::collections::HashMap;

struct Regs {
    x0: u32,
    x1: u32,
    x2: u32,
    x3: u32,
    x4: u32,
    x5: u32,
    x6: u32,
    x7: u32,
    x8: u32,
    x9: u32,
    x10: u32,
    x11: u32,
    x12: u32,
    x13: u32,
    x14: u32,
    x15: u32,
    x16: u32,
    x17: u32,
    x18: u32,
    x19: u32,
    x20: u32,
    x21: u32,
    x22: u32,
    x23: u32,
    x24: u32,
    x25: u32,
    x26: u32,
    x27: u32,
    x28: u32,
    x29: u32,
    x30: u32,
    x31: u32,
    x32: u32,
    pc: u32,
}

fn split_segments(code: &String) -> (String, String) {
    let data_index: Option<usize> = code.find(".data");
    let text_index: Option<usize> = code.find(".text");
    // ver qual é o primeiro e qual é o segundo
    // colocar o código correspondente a cada segmento
    match (data_index, text_index) {
        (Some(data_index), Some(text_index)) => {
            if data_index < text_index {
                (code[data_index+5..text_index].trim().to_string(), code[text_index+5..].trim().to_string())
            } else {
                (code[data_index+5..].trim().to_string(), code[text_index+5..data_index].trim().to_string())
            }
        },
        (None, Some(text_index)) => ("".to_string(), code[text_index+5..].trim().to_string()),
        (None, None) => ("".to_string(), code.trim().to_string()),
        _ => {
            println!("O segmento .text não foi encontrado");
            ("".to_string(), "".to_string())
        }
    }
}

fn sub_labels(data: &String, text: &String) -> (HashMap<String, i32>, HashMap<String, i32>) {
    let data_hash: HashMap<String, i32> = HashMap::new();
    let text_hash: HashMap<String, i32> = HashMap::new();
    (data_hash, text_hash)
}


fn read_file(path: &String) {
    let content: String = std::fs::read_to_string(path).expect("Couldn't read file");
    let (data, text) = split_segments(&content);
    println!("Data: {data}");
    println!("Text: {text}");
    let (data_hash, text_hash) = sub_labels(&data, &text);
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    read_file(&args[1]);
}
