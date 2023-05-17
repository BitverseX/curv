use std::fmt;

use serde::de::{SeqAccess, Visitor};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use crate::BigInt;

use super::traits::Converter;

impl Serialize for BigInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let data = self.to_hex();
        serializer.serialize_str(data.as_str())
    }
}

impl<'de> Deserialize<'de> for BigInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BigintVisitor;

        impl<'de> Visitor<'de> for BigintVisitor {
            type Value = BigInt;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "bigint")
            }

            fn visit_str<E: de::Error>(self, s: &str) -> Result<BigInt, E> {
                Ok(BigInt::from_hex(s).expect("Failed in serde"))
            }
        }

        deserializer.deserialize_str(BigintVisitor)
    }
}
