use rust_bytecode_vm::{helper, Chunk, VM};

fn main() {
    let mut chunk = Chunk::default();
    let constant_offset = chunk.add_constant(1.2);
    chunk.add_code(1, 123);
    // When we add the constant offset into the stack, we need to encode it as a u8
    chunk.add_code(constant_offset as u8, 123);

    let constant_offset = chunk.add_constant(3.4);
    chunk.add_code(1, 123);
    chunk.add_code(constant_offset as u8, 123);

    chunk.add_code(4, 123);

    let constant_offset = helper::constant_offset_to_long_constant_offset(chunk.add_constant(5.6));
    chunk.add_code(2, 123);
    chunk.add_code(constant_offset[0], 123);
    chunk.add_code(constant_offset[1], 123);
    chunk.add_code(constant_offset[2], 123);

    chunk.add_code(7, 123);

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
