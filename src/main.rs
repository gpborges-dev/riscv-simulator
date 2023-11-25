use core::panic;
use std::collections::HashMap;

const DATA_ADDR : u32 = 0x10010000; // tem que ser unsigned? 
const TEXT_ADDR : u32 = 0x00400000; // qual é o tamanho máximo?

struct SymbolTable {
    data : HashMap<String, u32>,
    text : HashMap<String, u32>
}

struct Content<'a> {
    data : Vec<&'a str>,
    text: Vec<&'a str>
}

struct Memory {
    data : Vec<i32>,
    text : Vec<i32>
}

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

fn format_code(code: &str) -> String {
    let mut formated_code : String = String::new();
    for line in code.split("\r\n") {
        if line.contains(":") {
            formated_code.push_str(line.trim());
            let line_vec : Vec<&str> = line.trim().split(" ").collect();
            if line_vec.len() > 1 {
                formated_code.push_str("\n");
            } else {
                formated_code.push_str(" ");
            }
        } else if line != "" {
            formated_code.push_str(line.trim());
            formated_code.push_str("\n");
        }
    }
    formated_code
}

fn split_segments(code: &str) -> (Vec<&str>, Vec<&str>) {
    let data_index: Option<usize> = code.find(".data");
    let text_index: Option<usize> = code.find(".text");
    match (data_index, text_index) {
        (Some(data_index), Some(text_index)) => {
            if data_index < text_index {
                (code[data_index+5..text_index].trim().split("\n").collect(), code[text_index+5..].trim().split("\n").collect())
            } else {
                (code[data_index+5..].trim().split("\n").collect(), code[text_index+5..data_index].trim().split("\n").collect())
            }
        },
        (None, Some(text_index)) => (vec![], code[text_index+5..].trim().split("\n").collect()),
        (None, None) => (vec![], code.trim().split("\n").collect()),
        _ => panic!("Code segment not found")
    }
}

fn is_label(label : &str) -> bool {
    if label.starts_with(|c: char| c.is_numeric()) {
        false // a label começa com número
    } else {
        if label[1..].chars().all(char::is_alphanumeric) {
            true
        } else {
            false // a label tem caracteres especiais no meio
        }
    }
}

fn get_data_size(data : &mut &str) -> u32 {
    let line : Vec<&str> = data.split(" ").collect();
    if line.len() == 2 {
        if line[0] == ".string" || line[0] == ".asciiz" {
            let mut string: &str = line[1];
            if string.chars().nth(0).unwrap() == '"' && string.chars().nth(string.len()-1).unwrap() == '"'{
                string = &string[1..string.len()-1];
                println!("{string}");
                (string.len() + 1).try_into().unwrap() // tem que considerar o \0
            } else {
                panic!("Invalid {} : {}", line[0], string)
            } 
        } else if line[0] == ".ascii" {
            let mut string: &str = line[1];
            if string.chars().nth(0).unwrap() == '"' && string.chars().nth(string.len()-1).unwrap() == '"'{
                string = &string[1..string.len()-1];
                string.len().try_into().unwrap() // não considera o \0
            } else {
                panic!("Invalid {} : {}", line[0], string)
            }
        } else if line[0] == ".byte" {
            if line[1].starts_with("0x") {
                if let Ok(_) = <u8>::from_str_radix(line[1].trim_start_matches("0x"), 16) {
                    1
                } else {
                    panic!("{} is a invalid type or is out of bounds", line[1])
                }
            } else {
                if let Ok(_) = line[1].parse::<u8>() {
                    1
                } else {
                    panic!("{} is a invalid type or is out of bounds", line[1])
                }
            }
        } else if line[0] == ".half" {
            if line[1].starts_with("0x") {
                if let Ok(_) = <u16>::from_str_radix(line[1].trim_start_matches("0x"), 16) {
                    2
                } else {
                    panic!("{} is a invalid type or is out of bounds", line[1])
                }
            } else {
                if let Ok(_) = line[1].parse::<u16>() {
                    2
                } else {
                    panic!("{} is a invalid type or is out of bounds", line[1])
                }
            }
        } else if line[0] == ".word" {
            if line[1].starts_with("0x") {
                if let Ok(_) = <u32>::from_str_radix(line[1].trim_start_matches("0x"), 16) {
                    4
                } else {
                    panic!("{} is a invalid type or is out of bounds", line[1])
                }
            } else {
                if let Ok(_) = line[1].parse::<u32>() {
                    4
                } else {
                    panic!("{} is a invalid type or is out of bounds", line[1])
                }
            }
        } else if line[0] == ".dword" {
            if line[1].starts_with("0x") {
                if let Ok(_) = <u64>::from_str_radix(line[1].trim_start_matches("0x"), 16) {
                    8
                } else {
                    panic!("{} is a invalid type or is out of bounds", line[1])
                }
            } else {
                if let Ok(_) = line[1].parse::<u64>() {
                    8
                } else {
                    panic!("{} is a invalid type or is out of bounds", line[1])
                }
            }
        } else {
            panic!("{} is from a invalid type", line[0])
        }
    } else if line.len() == 1 {
        if data.starts_with("0x") {
            if let Ok(_) = <u8>::from_str_radix(data.trim_start_matches("0x"), 16) {
                let new_data = format!(".byte {}", *data);
                *data = Box::leak(new_data.into_boxed_str());
                1
            } else if let Ok(_) = <u16>::from_str_radix(data.trim_start_matches("0x"), 16) {
                let new_data = format!(".half {}", *data);
                *data = Box::leak(new_data.into_boxed_str());
                2
            } else if let Ok(_) = <u32>::from_str_radix(data.trim_start_matches("0x"), 16) {
                let new_data = format!(".word {}", *data);
                *data = Box::leak(new_data.into_boxed_str());
                4
            } else if let Ok(_) = <u64>::from_str_radix(data.trim_start_matches("0x"), 16) {
                let new_data = format!("dword. {}", *data);
                *data = Box::leak(new_data.into_boxed_str());
                8
            } else {
                panic!("{data} is a invalid type or is out of bounds")
            }
        } else {
            if let Ok(_) = data.parse::<u8>() {
                1
            } else if let Ok(_) = data.parse::<u16>() {
                2
            } else if let Ok(_) = data.parse::<u32>() {
                4
            } else if let Ok(_) = data.parse::<u64>() {
                8
            } else {
                panic!("{data} is a invalid type or is out of bounds")
            }
        }
    } else {
        panic!("{data} is from a invalid type")
    }

    
}

fn sub_labels(data : &mut Vec<&str>, text : &mut Vec<&str>) -> (HashMap<String, u32>, HashMap<String, u32>) {
    let mut data_hash : HashMap<String, u32> = HashMap::new();
    let mut text_hash : HashMap<String, u32> = HashMap::new();
    
    for (index, line) in (0u32..).zip(text) {
        let delimiter: Option<usize> = line.rfind(':');
        let mut labels : Vec<&str> = Vec::new();
        if let Some(d) = delimiter {
            if d == line.len() - 1 {
                labels = line.split(' ').collect();
                if labels.len() == 1 {
                    panic!("label {line} is pointing to a location not used")
                } else {
                    panic!("{line} is not a valid command")
                }
            }
            labels = line[..d+1].split(' ').collect(); // pega as labels que podem estar na linha
            *line = &line[d+2..]; // tira a label da lista original
        }
        for label in labels {
            let id = label[..label.len()-1].to_string(); // deixa só o identificador
            if !is_label(&id) {
                panic!("{id} is not a valid label")
            }
            if text_hash.contains_key(&id) {
                panic!("label {id} has already been defined");
            }
            // verificar se o endereço não é grande demais
            text_hash.insert(id, TEXT_ADDR + index*4);
        }
    }
    
    let mut current_offset : u32 = 0; 
    for mut line in data {
        let delimiter: Option<usize> = line.rfind(':');
        let mut labels : Vec<&str> = Vec::new();
        if let Some(d) = delimiter {
            if d == line.len() - 1 {
                labels = line.split(' ').collect();
                if labels.len() == 1 {
                    panic!("label {line} is pointing to a location not used")
                } else {
                    panic!("{line} is not a valid command")
                }
            }
            labels = line[..d+1].split(' ').collect();
            *line = &line[d+2..];
        }

        for label in labels {
            let id = label[..label.len()-1].to_string(); // deixa só o identificador
            if !is_label(&id) {
                panic!("{id} is not a valid label")
            }
            if data_hash.contains_key(&id) {
                panic!("label {id} has already been defined");
            }
            // verificar se o endereço não é grande demais
            data_hash.insert(id, DATA_ADDR + current_offset);
        }
        current_offset += get_data_size(&mut line);
        // println!("{current_offset}");
    }
    println!("{:?}", data_hash);
    // println!("{:?}", text_hash);
    (data_hash, text_hash)
}


fn pre_translator(path: &str) {
    let content: String = std::fs::read_to_string(path).expect("Couldn't read file");
    let content: String = format_code(&content);
    // println!("Content: \n{content}");
    let (mut data, mut text) = split_segments(&content);
    // println!("Data:\n{:?}", data);
    // println!("Text:\n{:?}", text);
    let (data_hash, text_hash) = sub_labels(&mut data, &mut text);
    println!("Data:\n{:?}", data);
    println!("Text:\n{:?}", text);
}

fn translator(data: &Vec<&str>, text : &Vec<&str>) {

}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    pre_translator(&args[1]);
}