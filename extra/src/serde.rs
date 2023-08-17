struct StrToBytes<const N: usize> {}
impl<'de, const N: usize> serde::de::Visitor<'de> for StrToBytes<N> {
    type Value = [u8; N];
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E> {
        let mut ret: [u8; N] = [0; N];
        let min = if s.len() < N { s.len() } else { N };
        ret[0..min].copy_from_slice(&s.as_bytes()[0..min]);
        Ok(ret)
    }
}

#[allow(unused)]
fn ser_bytes_as_str<B: crate::from_bytes::FromBytes, S: serde::Serializer>(
    ty: &B,
    s: S,
) -> std::result::Result<S::Ok, S::Error> {
    ty.from_bytes()
        .map_err(|e| serde::ser::Error::custom(format!("{}", e)))
        .and_then(|val| s.serialize_str(val))
}

#[allow(unused)]
fn ser_option_big_array<const N: usize, S: serde::Serializer>(
    ty: Option<[u8; N]>,
    s: S,
) -> std::result::Result<S::Ok, S::Error> {
    match ty {
        Some(arr) => serde_big_array::BigArray::serialize(&arr, s),
        None => s.serialize_none(),
    }
}

#[allow(unused)]
fn de_str_as_bytes<'de, D, const N: usize>(de: D) -> std::result::Result<[u8; N], D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    de.deserialize_str(StrToBytes::<N> {})
}

#[allow(unused)]
fn de_option_str_as_bytes<'de, D, const N: usize>(
    de: D,
) -> std::result::Result<Option<[u8; N]>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    Ok(de.deserialize_str(StrToBytes::<N> {}).map_or(None, Some))
}

#[allow(unused)]
fn de_option_big_str_as_bytes<'de, D, const N: usize>(
    de: D,
) -> std::result::Result<Option<[u8; N]>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    Ok(de.deserialize_str(StrToBytes::<N> {}).map_or(None, Some))
}

#[allow(unused)]
fn make_default_bytes<const N: usize>() -> [u8; N] {
    [0; N]
}

#[allow(unused)]
fn make_option_default_bytes<const N: usize>() -> Option<[u8; N]> {
    Some([0; N])
}
