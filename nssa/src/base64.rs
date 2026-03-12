use base64::prelude::{BASE64_STANDARD, Engine as _};
use serde::{Deserialize as _, Deserializer, Serialize as _, Serializer};

pub fn serialize<S: Serializer>(v: &[u8], s: S) -> Result<S::Ok, S::Error> {
    let base64 = BASE64_STANDARD.encode(v);
    String::serialize(&base64, s)
}

pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
    let base64 = String::deserialize(d)?;
    BASE64_STANDARD
        .decode(base64.as_bytes())
        .map_err(serde::de::Error::custom)
}
