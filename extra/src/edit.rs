pub trait Edit {
    fn edit(&mut self, bytes: &str);
}

impl Edit for [u8] {
    fn edit(&mut self, bytes: &str) {
        self[0..bytes.len()].copy_from_slice(bytes.as_bytes());
        self[bytes.len()..].fill(0);
    }
}
