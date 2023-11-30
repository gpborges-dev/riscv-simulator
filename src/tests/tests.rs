#[cfg(test)]
mod tests {
    use test::ula; // Importa o módulo ula
    struct TestALU {
        instance: alu::Alu,
    }

    impl TestALU {
        fn new() -> Self {
            TestALU {
                instance: alu::Alu::new(),
            }
        }
    }

    impl Drop for TestALU {
        fn drop(&mut self) {
            // Limpeza
        }
    }

    #[test]
    fn test_alu_add() {
        let mut test = TestALU::new();
        // add: 1 + 2 = 3
        let in1 = 0b00000000000000000000000000000001;
        let in2 = 0b00000000000000000000000000000010;
        let op = 0b00000000;
        let out = test.instance.calculate(in1, in2, op);
        assert_eq!(out, 3);
    }

    #[test]
    fn test_alu_sub() {
        let mut test = TestALU::new();
        // add: 5 - 2 = 3
        let in1 = 0b00000000000000000000000000000101;
        let in2 = 0b00000000000000000000000000000010;
        let op = 0b00001000;
        let out = test.instance.calculate(in1, in2, op);
        assert_eq!(out, 3);
    }

    #[test]
    fn test_alu_sra() {
        let mut test = TestALU::new();
        // sra
        let in1 = 0b10000000000000000000000000000101;
        let in2 = 0b00000000000000000000000000000110;
        let op = 13;
        let out = test.instance.calculate(in1, in2, op);
        let reference = 0b11111110000000000000000000000000;
        assert_eq!(out, reference);
    }

    #[test]
    fn test_alu_slt() {
        let mut test = TestALU::new();
        let in1 = 0b10000000000000000000000000000101;
        let in2 = 0b00000000000000000000000000000110;
        let op = 2;
        let out = test.instance.calculate(in1, in2, op);
        let reference = 1;
        assert_eq!(out, reference);
    }

    #[test]
    fn test_alu_slt2() {
        let mut test = TestALU::new();
        let in1 = 0b00000000000000000000000000000111;
        let in2 = 0b00000000000000000000000000000110;
        let op = 2;
        let out = test.instance.calculate(in1, in2, op);
        let reference = 0;
        assert_eq!(out, reference);
    }

    #[test]
    fn test_alu_sltu() {
        let mut test = TestALU::new();
        let in1 = 0b10000000000000000000000000000101;
        let in2 = 0b00000000000000000000000000000110;
        let op = 3;
        let out = test.instance.calculate(in1, in2, op);
        let reference = 0;
        assert_eq!(out, reference);
    }
}
