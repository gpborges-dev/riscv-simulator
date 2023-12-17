//! Temos aqui a implementação da memória do Riscv, que é dividida em duas partes: a memória de texto e a memória de dados.
//! A memória de texto é onde ficam armazenadas as instruções do programa, enquanto a memória de dados é onde ficam armazenados os dados do programa.
//! As duas memórias são implementadas como vetores de 32 bits, onde cada posição do vetor representa uma palavra de 32 bits.	
//! No entry point do programa, a memória de texto é instanciada e carregada com os valores do arquivo de texto que contém as instruções do programa.
//! A memória de dados é instanciada e carregada com os valores do arquivo de texto que contém os dados do programa.
pub struct Memory {
    pub text_segment: Vec<u32>,
    pub data_segment: Vec<u32>,
}

impl Memory {

    /// A função new() instancia a memória de texto e a memória de dados com vetores de 0xffc posições e 0x1000 posições, ou seja, 4096 posições.
    pub fn new() -> Self {
        Memory {
            text_segment: vec![0; 0xffc],
            data_segment: vec![0; 0x1000],
        }
    }

    /// As função write_data_word() escreve uma palavra de 32 bits na memória de dados.
    pub fn write_data_word(&mut self, address: u32, value: u32) {
        let address = address as usize;
        if address < self.data_segment.len() {
            self.data_segment[address] = value;
        } else {
            println!("Endereço fora dos limites da memória de dados.");
        }
    }
    /// A função write_text_word() escreve uma palavra de 32 bits na memória de texto.
    pub fn write_text_word(&mut self, address: u32, value: u32) {
        let address = address as usize;
        if address < self.text_segment.len() {
            self.text_segment[address] = value;
        } else {
            println!("Endereço fora dos limites da memória de dados.");
        }
    }
    /// A função read_data_word() lê uma palavra de 32 bits da memória de dados.

    pub fn read_data_word(&mut self, address: usize) -> u32 {
        if address < self.data_segment.len() {
            self.data_segment[address]
        } else {
            println!("Endereço fora dos limites da memória de dados.");
            0
        }
    }
    /// A função read_text_word() lê uma palavra de 32 bits da memória de texto.
    pub fn read_text_word(&mut self, address: usize) -> u32 {
        if address < self.text_segment.len() {
            self.text_segment[address]
        } else {
            println!("Endereço fora dos limites da memória de dados.");
            0
        }
    }
    /// A função print_memory() imprime a memória de texto e a memória de dados.
    pub fn print_memory(&mut self) {
        println!("Memória de texto: ");
        self.print_text_segment();
        println!("Memória de dados: ");
        self.print_data_segment();
    }
    /// A função print_text_segment() imprime a memória de texto.
    pub fn print_text_segment(&mut self) {
        for (k, &word) in self.text_segment.iter().enumerate() {
            if word != 0 {
                println!("{:x}: {:x}", k, word);
            }
        }
    }
    /// A função print_data_segment() imprime a memória de dados.
    pub fn print_data_segment(&mut self) {
        for (j, &word) in self.data_segment.iter().enumerate() {
            if word != 0 {
                println!("{:x}: {:x}", j, word);
            }
        }
    }
}