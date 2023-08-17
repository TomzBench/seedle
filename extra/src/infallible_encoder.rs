pub struct InfallibleEncoder(Vec<u8>);
impl InfallibleEncoder {
    pub fn new(len: usize) -> InfallibleEncoder {
        InfallibleEncoder(Vec::with_capacity(len))
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }
}

impl minicbor::encode::Write for InfallibleEncoder {
    type Error = core::convert::Infallible;
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        self.0.extend_from_slice(buf);
        Ok(())
    }
}
