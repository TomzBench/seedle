pub trait FromBytes {
    fn from_bytes(&self) -> core::result::Result<&str, core::str::Utf8Error>;
}

impl FromBytes for [u8] {
    fn from_bytes(&self) -> core::result::Result<&str, core::str::Utf8Error> {
        let ascii = self
            .iter()
            .position(|&x| x == 0)
            .map(|pos| self.split_at(pos).0)
            .unwrap_or(self);
        core::str::from_utf8(ascii)
    }
}

impl FromBytes for Option<&[u8]> {
    fn from_bytes(&self) -> core::result::Result<&str, core::str::Utf8Error> {
        match self {
            Some(bytes) => bytes.from_bytes(),
            None => Ok(""),
        }
    }
}

impl<const N: usize> FromBytes for [u8; N] {
    fn from_bytes(&self) -> core::result::Result<&str, core::str::Utf8Error> {
        self.as_ref().from_bytes()
    }
}

impl<const N: usize> FromBytes for Option<[u8; N]> {
    fn from_bytes(&self) -> core::result::Result<&str, core::str::Utf8Error> {
        match self {
            Some(bytes) => bytes.from_bytes(),
            None => Ok(""),
        }
    }
}
