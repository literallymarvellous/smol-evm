use smol_evm::{stack::Stack, instruction::Instruction, decode_opcode};


fn main() {
    let stack1 = Stack::new();

    let mut l1 = vec![1, 2, 4];
    let l2 = &l1[0..2];
    println!("{:?}", l2);

    let code = "20406080";
    println!("de {:?}", hex::decode(code));

    let inc = Instruction::new();

    decode_opcode(ctx);


}
