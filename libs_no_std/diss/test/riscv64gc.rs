use diss::riscv64gc::*;

const TEST: &[u32] = &[
    
];

#[test]
fn test_diss_and_ass() {
    let mut assembler = AssemblerRV64GC{};

    for word in TEST {
        let first_round = diss_riscv64gc(word, &mut assembler);
        let second_round = diss_riscv64gc(first_round, &mut assembler);
        // this is done because instructions like c_nop have multiple encodings
        // so we test that what we assemble is disassembled the same way
        assert_eq!(first_round, second_round);
    }
}