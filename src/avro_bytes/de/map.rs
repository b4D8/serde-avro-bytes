use crate::avro_bytes::de::bytes::BytesVisitor;
use serde::de::{Error, IgnoredAny, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::collections::{BTreeMap, HashMap};
use std::fmt::Formatter;

#[derive(Debug)]
struct Pair {
    key: Bytes,
    value: Bytes,
}

#[derive(Debug)]
pub struct Bytes(pub(crate) Vec<u8>);

impl<'de> Deserialize<'de> for Bytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Bytes(deserializer.deserialize_bytes(BytesVisitor)?))
    }
}

impl<'de> Deserialize<'de> for Pair {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct("Pair", &[], VisitorPair)
    }
}

struct VisitorPair;

impl<'de> Visitor<'de> for VisitorPair {
    type Value = Pair;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "Unable to decode Pair")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut key = None::<Bytes>;
        let mut value = None::<Bytes>;

        while let Some(ref key_str) = map.next_key::<String>()? {
            let key_str = key_str.as_str();
            match key_str {
                "key" => key = Some(map.next_value::<Bytes>()?),
                "value" => value = Some(map.next_value::<Bytes>()?),
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }

        let key = if let Some(x) = key {
            x
        } else {
            return Err(Error::missing_field("key"));
        };
        let value = if let Some(x) = value {
            x
        } else {
            return Err(Error::missing_field("value"));
        };

        Ok(Pair { key, value })
    }
}

#[allow(unused)]
pub fn deserialize_hash_map<'de, D>(deserializer: D) -> Result<HashMap<Vec<u8>, Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    let visitor = HashMapVisitor;
    deserializer.deserialize_seq(visitor)
}

struct HashMapVisitor;

impl<'de> Visitor<'de> for HashMapVisitor {
    type Value = HashMap<Vec<u8>, Vec<u8>>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "Unable to decode option of hashmap")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut map = HashMap::new();

        while let Some(Pair { key, value }) = seq.next_element::<Pair>()? {
            map.insert(key.0, value.0);
        }

        Ok(map)
    }
}

#[allow(unused)]
pub fn deserialize_btree_map<'de, D>(
    deserializer: D,
) -> Result<BTreeMap<Vec<u8>, Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    let visitor = BtreeMapVisitor;
    deserializer.deserialize_seq(visitor)
}

struct BtreeMapVisitor;

impl<'de> Visitor<'de> for BtreeMapVisitor {
    type Value = BTreeMap<Vec<u8>, Vec<u8>>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "Unable to decode btreemap")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut map = BTreeMap::new();

        while let Some(Pair { key, value }) = seq.next_element::<Pair>()? {
            map.insert(key.0, value.0);
        }

        Ok(map)
    }
}

#[allow(unused)]
pub fn deserialize_option_hash_map<'de, D>(
    deserializer: D,
) -> Result<Option<HashMap<Vec<u8>, Vec<u8>>>, D::Error>
where
    D: Deserializer<'de>,
{
    let visitor = OptionHashMapVisitor;
    deserializer.deserialize_option(visitor)
}

#[allow(unused)]
pub fn deserialize_option_btree_map<'de, D>(
    deserializer: D,
) -> Result<Option<BTreeMap<Vec<u8>, Vec<u8>>>, D::Error>
where
    D: Deserializer<'de>,
{
    let visitor = OptionBtreeMapVisitor;
    deserializer.deserialize_option(visitor)
}

struct OptionHashMapVisitor;

impl<'de> Visitor<'de> for OptionHashMapVisitor {
    type Value = Option<HashMap<Vec<u8>, Vec<u8>>>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "Unable to decode option of hashmap")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Some(deserialize_hash_map(deserializer)?))
    }
}

struct OptionBtreeMapVisitor;

impl<'de> Visitor<'de> for OptionBtreeMapVisitor {
    type Value = Option<BTreeMap<Vec<u8>, Vec<u8>>>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "Unable to decode option of btreemap")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Some(deserialize_btree_map(deserializer)?))
    }
}