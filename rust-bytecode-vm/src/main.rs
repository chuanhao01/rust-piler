use rust_bytecode_vm::{Chunk, VM};

fn main() {
    let mut chunk = Chunk::default();
    let constant_idx = chunk.add_constants(3.1);
    chunk.add_constants(123.2);
    chunk.add_code(1, 123);
    // When we add the constant offset into the stack, we need to encode it as a u8
    chunk.add_code(constant_idx as u8, 123);
    // chunk.add_code(2, 124);
    // chunk.add_code(0x01, 124);
    // chunk.add_code(0x00, 124);
    // chunk.add_code(0x00, 124);
    chunk.add_code(3, 123);
    chunk.add_code(0, 124);
    let mut vm = VM::new(chunk);
    vm.run();
    vm.free();
    // chunk.disassemble("test chunk");
    // chunk.free();
}
