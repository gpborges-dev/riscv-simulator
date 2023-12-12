use core::panic;
use std::{collections::HashMap, io::Write};

const DATA_ADDR : u32 = 0x00002000; // max 4096 bytes
const TEXT_ADDR : u32 = 0x00000000; // max 4092 bytes
// @ implementar linhas de código

fn u32str_to_bin(dec : &str) -> String {
    if let Ok(d) = dec.parse::<u32>() {
        format!("{:b}", d)
    } else {
        String::from("")
    }
}

// @ verificar o erro exato
fn get_i32(num : &str) -> i32 {
    if num.to_lowercase().starts_with("0x") {
        if let Ok(n) = i32::from_str_radix(&num[2..], 16) {
            n
        } else {
            panic!("Invalid hex: {num}")
        }
    } else {
        if let Ok(n) = num.parse::<i32>() {
            n
        } else {
            panic!("Invalid decimal: {num}")
        }
    }
}

fn get_u32(num : &str) -> u32 {
    if num.to_lowercase().starts_with("0x") {
        if let Ok(n) = u32::from_str_radix(&num[2..], 16) {
            n
        } else {
            panic!("Invalid hex: {num}")
        }
    } else {
        if let Ok(n) = num.parse::<u32>() {
            n
        } else {
            panic!("Invalid decimal: {num}")
        }
    }
}

fn get_u64(num : &str) -> u64 {
    if num.to_lowercase().starts_with("0x") {
        if let Ok(n) = u64::from_str_radix(&num[2..], 16) {
            n
        } else {
            panic!("Invalid hex: {num}")
        }
    } else {
        if let Ok(n) = num.parse::<u64>() {
            n
        } else {
            panic!("Invalid decimal: {num}")
        }
    }
}

fn is_reg(reg : &str) -> bool {
    if let Ok(num) = reg[1..].parse::<u8>() {
        if reg.starts_with('x') && (0..=31).contains(&num) {
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn read_escapes(input: &Vec<u8>) -> Vec<u8> {
    let mut output = Vec::new();

    let mut iter = input.iter().copied();

    while let Some(byte) = iter.next() {
        if byte == b'\\' {
            if let Some(escaped) = iter.next() {
                match escaped {
                    b'n' => output.push(b'\n'),
                    b't' => output.push(b'\t'),
                    b'r' => output.push(b'\r'),
                    b'0' => output.push(b'\0'),
                    _ => {
                        output.push(b'\\');
                        output.push(escaped);
                    }
                }
            } else {
                output.push(b'\\');
            }
        } else {
            output.push(byte);
        }
    }

    output
}

fn r_type(mnemonic : &str, args : &Vec<&str>) -> String {
    if args.len() != 3 || !is_reg(&args[0]) || !is_reg(&args[1]) || !is_reg(&args[2]) {
        panic!{"Arguments {:?} invalid for {mnemonic}", args}
    }

    let opcode : String = String::from("0110011");
    let funct7 : String = String::from(if ["sub", "sra"].iter().any(|m| m == &mnemonic) {"0100000"} else {"0000000"});
    let funct3_hash : HashMap<&str, &str> = HashMap::from([
        ("add", "000"),
        ("sub", "000"),
        ("sll", "001"),
        ("slt", "010"),
        ("sltu", "011"),
        ("xor", "100"),
        ("srl", "101"),
        ("sra", "101"),
        ("or", "110"),
        ("and", "111"),
    ]);
    let funct3 : String = funct3_hash[mnemonic].to_string();

    let rd : String = format!("{:0>5}", u32str_to_bin(&args[0][1..]));
    let rs1 : String = format!("{:0>5}", u32str_to_bin(&args[1][1..]));
    let rs2 : String = format!("{:0>5}", u32str_to_bin(&args[2][1..]));
    
    format!("{funct7}{rs2}{rs1}{funct3}{rd}{opcode}\n")
}

fn i_type(mnemonic : &str, args : &Vec<&str>) -> String {
    let mut imm : String = String::new();
    let mut rs1 : String = String::new();
    let mut rd : String = String::new();
    if mnemonic.starts_with('l') && args.len() == 2 {
        rd = args[0].to_string();
        let open: Option<usize> = args[1].find('(');
        let close : Option<usize> = args[1].rfind(')');
        match (open, close) {
            (Some(o), Some(c)) => {
                imm = args[1][..o].to_string();
                let num : i32 = get_i32(&imm);
                rs1 = args[1][o+1..c].to_string();
                if !is_reg(&rd) || !is_reg(&rs1) {
                    panic!("Arguments {:?} invalid for {mnemonic}", args)   
                }
                if -2048 <= num && num < 0 {
                    imm = format!("{:b}", num)[20..=31].to_string(); // se o número é negativo vai retornar mais de 12 bits
                 } else if 0 <= num && num <= 2047 {
                     imm = format!("{:0>12b}", num); // se o número é positivo vai retornar certo
                 } else {
                    panic!("Immediate {imm} out of range for {mnemonic}")
                }
            } 
            (_, _) => panic!("Arguments {:?} invalid for {mnemonic}", args) 
        }
        rs1 = format!("{:0>5}", u32str_to_bin(&rs1[1..]));
        
    } else {
        if args.len() == 3 && is_reg(&args[0]) && is_reg(args[1]) {
            let num = get_i32(&args[2]);
            if ["slli", "srli", "srai"].iter().any(|m| m == &mnemonic) {
                if 0 <= num && num <= 31 {
                    imm = format!("{}{:0>5b}", if mnemonic == "srai" {"0100000"} else {"0000000"}, num)
                } else {
                    panic!("Immediate {imm} out of range for {mnemonic}")
                }
            } else {
                if -2048 <= num && num < 0 {
                    imm = format!("{:b}", num)[20..=31].to_string(); // se o número é negativo vai retornar mais de 12 bits
                } else if 0 <= num && num <= 2047 {
                    imm = format!("{:0>12b}", num); // se o número é positivo vai retornar certo
                } else {
                    panic!("Immediate {imm} out of range for {mnemonic}")
                }
            }
            rs1 = format!("{:0>5}", u32str_to_bin(&args[1][1..]));

        } else {
            panic!("Arguments {:?} invalid for {mnemonic}", args) 
        }
    }

    let opcode : String = String::from(if mnemonic.starts_with('l') {"0000011"} else if mnemonic == "jalr" {"1100111"} else {"0010011"});
    let funct3_hash : HashMap<&str, &str> = HashMap::from([
        ("addi", "000"),
        ("slti", "010"),
        ("sltiu", "011"),
        ("xori", "100"),
        ("ori", "110"),
        ("andi", "111"),
        ("slli", "001"),
        ("srli", "101"),
        ("srai", "101"),
        ("lb", "000"),
        ("lh", "001"),
        ("lw", "010"),
        ("lbu", "100"),
        ("lhu", "101"),
        ("jalr", "000")
    ]);
    let funct3 : String = funct3_hash[mnemonic].to_string();
    rd = format!("{:0>5}", u32str_to_bin(&args[0][1..]));

    format!("{}{}{}{}{}\n", imm, rs1, funct3, rd, opcode)
}

fn s_type(mnemonic : &str, args : &Vec<&str>) -> String {
    if args.len() != 2 {
        panic!("Arguments {:?} invalid for {mnemonic}", args) 
    }

    let mut imm : String = String::new();
    let mut rs1 : String = String::new();
    let mut rs2 : String = args[0].to_string();
    let open: Option<usize> = args[1].find('(');
    let close : Option<usize> = args[1].rfind(')');
    match (open, close) {
        (Some(o), Some(c)) => {
            imm = args[1][..o].to_string();
            let num : i32 = get_i32(&imm);
            rs1 = args[1][o+1..c].to_string();
            if !is_reg(&rs2) || !is_reg(&rs1) {
                panic!("Arguments {:?} invalid for {mnemonic}", args)   
            }
            if -2048 <= num && num < 0 {
                imm = format!("{:b}", num)[20..=31].to_string(); // se o número é negativo vai retornar mais de 12 bits
            } else if 0 <= num && num <= 2047 {
                imm = format!("{:0>12b}", num); // se o número é positivo vai retornar certo
            } else {
                panic!("Immediate {imm} out of range for {mnemonic}")
            }
        } 
        (_, _) => panic!("Arguments {:?} invalid for {mnemonic}", args) 
    }

    let opcode : String = String::from("0100011");
    let funct3_hash : HashMap<&str, &str> = HashMap::from([
        ("sb", "000"),
        ("sh", "001"),
        ("sw", "010"),
    ]);
    let funct3 : String = funct3_hash[mnemonic].to_string();

    rs1 = format!("{:0>5}", u32str_to_bin(&rs1[1..]));
    rs2 = format!("{:0>5}", u32str_to_bin(&rs2[1..]));

    format!("{}{}{}{}{}{}\n", &imm[0..=6], rs2, rs1, funct3, &imm[7..], opcode)
}

fn b_type(mnemonic : &str, args : &Vec<&str>, text_hash : &HashMap<String, u32>, inst_addr : &u32) -> String {
    if args.len() != 3 || !is_reg(&args[0]) || !is_reg(&args[1]) || !text_hash.contains_key(args[2]) {
        panic!("Arguments {:?} invalid for {mnemonic}", args)
    }

    let opcode : String = String::from("1100011");
    let funct3_hash : HashMap<&str, &str> = HashMap::from([
        ("beq", "000"),
        ("bne", "001"),
        ("blt", "100"),
        ("bge", "101"),
        ("bltu", "110"),
        ("bgeu", "111"),
    ]);
    let funct3: String = funct3_hash[mnemonic].to_string();
    let rs1: String = format!("{:0>5}", u32str_to_bin(&args[0][1..]));        
    let rs2: String = format!("{:0>5}", u32str_to_bin(&args[1][1..]));

    let branch_addr: u32 = text_hash[args[2]];
    let offset : i32 = branch_addr as i32 - *inst_addr as i32;
    if !(-2048..=2047).contains(&offset) {
        panic!("Offset {offset} must be max 12 bits long")
    }
    let imm : String = format!("{:0>32b}", offset);
    format!("{}{}{}{}{}{}{}{}\n", &imm.chars().nth(19).unwrap(), &imm[21..=26], rs2, rs1, funct3, &imm[27..=30], &imm.chars().nth(20).unwrap(), opcode)
}

fn u_type(mnemonic : &str, args : &Vec<&str>) -> String {
    if args.len() != 2 || !is_reg(&args[0]) {
        panic!("Arguments {:?} invalid for {mnemonic}", args)
    }

    let num = get_i32(&args[1]);
    let mut imm : String = String::new();
    if -524288 <= num && num < 0 {
        imm = format!("{:b}", num)[12..=31].to_string();
    } else if 0 <= num && num <= 524287 {
        imm = format!("{:0>20b}", num).to_string();
    } else {
        panic!("Immediate {} must be max 20 bits long", args[1])
    }

    let opcode : String = String::from(if mnemonic == "lui" {"0110111"} else {"0010111"});
    let rd : String = format!("{:0>5}", u32str_to_bin(&args[0][1..]));

    format!("{imm}{rd}{opcode}\n")
}

fn j_type(mnemonic : &str, args : &Vec<&str>, text_hash : &HashMap<String, u32>, inst_addr : &u32) -> String {
    if args.len() != 2 && is_reg(&args[0]) && !text_hash.contains_key(args[2]) {
        panic!("Arguments {:?} invalid for {mnemonic}", args)
    }

    let branch_addr: u32 = text_hash[args[1]];
    let offset : i32 = branch_addr as i32 - *inst_addr as i32;
    if !(-524288..=524287).contains(&offset) {
        panic!("Offset {offset} must be max 20 bits long")
    }
    let opcode: String = String::from("1101111");
    let rd : String = format!("{:0>5}", u32str_to_bin(&args[0][1..]));
    let imm : String = format!("{:0>32b}", offset);
    format!("{}{}{}{}{}{}\n", &imm.chars().nth(11).unwrap(), &imm[21..=30], &imm.chars().nth(20).unwrap(), &imm[12..=19], rd, opcode)
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
    let line : Vec<&str> = data.splitn(2," ").collect();
    if line.len() == 2 {
        if line[0] == ".string" || line[0] == ".asciiz" {
            let mut string: &str = line[1];
            if string.chars().nth(0).unwrap() == '"' && string.chars().nth(string.len()-1).unwrap() == '"'{
                string = &string[1..string.len()-1];
                // println!("{string}");
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
            if line[1].to_lowercase().starts_with("0x") {
                if let Ok(_) = <u8>::from_str_radix(&line[1][2..], 16) {
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
            if line[1].to_lowercase().starts_with("0x") {
                if let Ok(_) = <u16>::from_str_radix(&line[1][2..], 16) {
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
            if line[1].to_lowercase().starts_with("0x") {
                if let Ok(_) = <u32>::from_str_radix(&line[1][2..], 16) {
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
            if line[1].to_lowercase().starts_with("0x") {
                if let Ok(_) = <u64>::from_str_radix(&line[1][2..], 16) {
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
        if data.to_lowercase().starts_with("0x") {
            if let Ok(_) = <u8>::from_str_radix(&data[2..], 16) {
                let new_data = format!(".byte {}", *data);
                *data = Box::leak(new_data.into_boxed_str());
                1
            } else if let Ok(_) = <u16>::from_str_radix(&data[2..], 16) {
                let new_data = format!(".half {}", *data);
                *data = Box::leak(new_data.into_boxed_str());
                2
            } else if let Ok(_) = <u32>::from_str_radix(&data[2..], 16) {
                let new_data = format!(".word {}", *data);
                *data = Box::leak(new_data.into_boxed_str());
                4
            } else if let Ok(_) = <u64>::from_str_radix(&data[2..], 16) {
                let new_data = format!("dword. {}", *data);
                *data = Box::leak(new_data.into_boxed_str());
                8
            } else {
                panic!("{data} is a invalid type or is out of bounds")
            }
        } else {
            if let Ok(_) = data.parse::<u8>() {
                let new_data = format!(".byte {}", *data);
                *data = Box::leak(new_data.into_boxed_str());
                1
            } else if let Ok(_) = data.parse::<u16>() {
                let new_data = format!(".half {}", *data);
                *data = Box::leak(new_data.into_boxed_str());
                2
            } else if let Ok(_) = data.parse::<u32>() {
                let new_data = format!(".word {}", *data);
                *data = Box::leak(new_data.into_boxed_str());
                4
            } else if let Ok(_) = data.parse::<u64>() {
                let new_data = format!("dword. {}", *data);
                *data = Box::leak(new_data.into_boxed_str());
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
            if index * 4 > 4092 {
                panic!("Text memory overflow")
            }
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
        if current_offset > 4096 {
            panic!("Data memory overflow")
        }
        // println!("{current_offset}");
    }
    // println!("{:?}", data_hash);
    // println!("{:?}", text_hash);
    (data_hash, text_hash)
}

fn translator(data: &Vec<&str>, text : &Vec<&str>, data_hash : &HashMap<String, u32>, text_hash : &HashMap<String, u32>) -> String {
    let r_mnemonics = ["add", "sub", "sll", "slt", "sltu", "xor", "srl", "sra", "or", "and"];
    let i_mnemonics = ["addi", "addi", "slti", "sltiu", "xori", "ori", "andi", "slli", "srli", "srai", "jalr", "lb", "lh", "lw", "lbu", "lhu"];
    let s_mnemonics = ["sb", "sw", "sh"];
    let b_mnemonics = ["beq", "bne", "blt", "bge", "bltu", "bgeu"];
    let u_mnemonics = ["lui", "auipc"];
    let j_mnemonics = ["jal"];

    let mut textbinaries : String = String::new();
    for (index, instruction) in text.iter().enumerate() {
        let inst_addr: u32 = TEXT_ADDR + 4 * index as u32;
        let delimiter = instruction.find(' ');
        let mut mnemonic : &str = "";
        let mut args : Vec<&str> = Vec::new();
        if let Some(d) = delimiter {
            mnemonic = &instruction[..d].trim();
            args = instruction[d+1..].split(',').map(|arg| arg.trim()).collect();
        } else if instruction.trim() == "ecall" {
            mnemonic = "ecall";
        } else {
            panic!("Instruction {instruction} not valid")
        }

        if r_mnemonics.iter().any(|m| m == &mnemonic) {
            textbinaries.push_str(&r_type(&mnemonic, &args));
        } else if i_mnemonics.iter().any(|m| m == &mnemonic) {
            textbinaries.push_str(&i_type(&mnemonic, &args));
        } else if s_mnemonics.iter().any(|m| m == &mnemonic) {
            textbinaries.push_str(&s_type(&mnemonic, &args));
        } else if b_mnemonics.iter().any(|m| m == &mnemonic) {
            textbinaries.push_str(&b_type(&mnemonic, &args, &text_hash, &inst_addr));
        } else if u_mnemonics.iter().any(|m| m == &mnemonic) {
            textbinaries.push_str(&u_type(&mnemonic, &args));
        } else if j_mnemonics.iter().any(|m| m == &mnemonic) {
            textbinaries.push_str(&j_type(&mnemonic, &args, &text_hash, &inst_addr));
        } else if mnemonic == "ecall" {
            textbinaries.push_str("00000000000000000000000001110011\n");
        } else {
            panic!("Invalid mnemonic {mnemonic}")
        }
    }

    // @ fazer o \n ser apenas um caractere
    let mut databinaries: String = String::new();
    for d in data {
        let element : Vec<&str> = d.splitn(2, " ").collect();
        let etype = element[0];
        let content = element[1].trim_matches('"');

        if etype == ".string" || etype == ".asciiz" || etype == ".ascii" {
            let mut bytes : Vec<u8> = content.bytes().collect();
            bytes = read_escapes(&bytes);
            for b in bytes {
                databinaries += &format!("{:0>8b}", b);
            }
            if etype != ".ascii" {
                databinaries += "00000000";
            }
        } else if etype == ".byte" {
            databinaries += &format!("{:0>8b}", get_u32(content));
        } else if etype == ".half" {
            databinaries += &format!("{:0>16b}", get_u32(content));
        } else if etype == ".word" {
            databinaries += &format!("{:0>32b}", get_u32(content));
        } else if etype == ".dword" {
            databinaries += &format!("{:0>64b}", get_u64(content));
        }
    }

    let mut formatedatabin: String = String::new();
    for (index, bit) in databinaries.chars().enumerate() {
        if index % 32 == 0 && index > 0 {
            formatedatabin += "\n";
        }
        formatedatabin += &bit.to_string();
    }
    formatedatabin += &"0".repeat(if databinaries.len() % 32 == 0 {0} else {32 - databinaries.len() % 32});
    formatedatabin += if data.len() > 0 {"\n\n"} else {""};

    format!("{}{}", formatedatabin, textbinaries).trim_end_matches("\n").to_string()
}

pub fn assemble(path: &str) {
    let content: String = std::fs::read_to_string(path).expect("Couldn't read file");
    let formated_content: String = format_code(&content);
    // println!("Content: \n{content}");
    let (mut data, mut text) = split_segments(&formated_content);
    // println!("Data:\n{:?}", data);
    // println!("Text:\n{:?}", text);
    let (data_hash, text_hash) = sub_labels(&mut data, &mut text);
    // println!("Data:\n{:?}", data);
    // println!("Text:\n{:?}", text);
    let mut file = std::fs::File::create("bin.txt").expect("Couldn't create file");
    file.write(translator(&data, &text, &data_hash, &text_hash).as_bytes()).expect("Couldn't write to file");
}