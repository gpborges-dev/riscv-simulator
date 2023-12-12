pub struct Memory {
    pub text_segment: Vec<u32>,
    pub data_segment: Vec<u32>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            text_segment: vec![0; 0x1000],
            data_segment: vec![0; 0x1000],
        }
    }

    pub fn write_data_word(&mut self, address: u32, value: u32) {
        let address = address as usize;
        if address < self.data_segment.len() {
            self.data_segment[address] = value;
        } else {
            println!("Endereço fora dos limites da memória de dados.");
        }
    }
    pub fn write_text_word(&mut self, address: u32, value: u32) {
        let address = address as usize;
        if address < self.text_segment.len() {
            self.text_segment[address] = value;
        } else {
            println!("Endereço fora dos limites da memória de dados.");
        }
    }

    pub fn read_data_word(&mut self, address: usize) -> u32 {
        if address < self.data_segment.len() {
            self.data_segment[address]
        } else {
            println!("Endereço fora dos limites da memória de dados.");
            0
        }
    }
    pub fn read_text_word(&mut self, address: usize) -> u32 {
        if address < self.text_segment.len() {
            self.text_segment[address]
        } else {
            println!("Endereço fora dos limites da memória de dados.");
            0
        }
    }
    pub fn print_memory(&mut self) {
        println!("Memória de texto: ");
        self.print_text_segment();
        println!("Memória de dados: ");
        self.print_data_segment();
    }
    pub fn print_text_segment(&mut self) {
        for (k, &word) in self.text_segment.iter().enumerate() {
            if word != 0 {
                println!("{:x}: {:x}", k, word);
            }
        }
    }
    pub fn print_data_segment(&mut self) {
        for (j, &word) in self.data_segment.iter().enumerate() {
            if word != 0 {
                println!("{:x}: {:x}", j, word);
            }
        }
    }
}