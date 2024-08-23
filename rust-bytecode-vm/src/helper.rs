pub fn constant_offset_to_long_constant_offset(offset: usize) -> [u8; 3] {
    assert!(offset <= 16777215);
    let mut bytes: [u8; 3] = Default::default();
    bytes.copy_from_slice(&offset.to_le_bytes()[0..3]);
    bytes
}
