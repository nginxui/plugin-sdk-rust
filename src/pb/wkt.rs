//! Shims for `google.protobuf.Struct`, `Value` and `ListValue`.
//!
//! The generated messages use these types instead of the ones of
//! `prost-types`, which have no JSON mapping. They map to and from
//! [`serde_json::Value`] and serialize as plain JSON, as the protobuf JSON
//! mapping requires. Numbers are IEEE 754 doubles in protobuf, so a whole
//! number that a double holds exactly is written as an integer, which keeps
//! `as_i64` working on values that arrived over gRPC.

use std::collections::BTreeMap;
use std::fmt;

use serde::de::{self, MapAccess, SeqAccess, Visitor};
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// `google.protobuf.Struct`, a JSON object.
#[derive(Clone, PartialEq, prost::Message)]
pub struct Struct {
    /// Members of the object.
    #[prost(btree_map = "string, message", tag = "1")]
    pub fields: BTreeMap<String, Value>,
}

/// `google.protobuf.Value`, any JSON value.
#[derive(Clone, PartialEq, prost::Message)]
pub struct Value {
    /// The kind of value, `None` reads as null.
    #[prost(oneof = "value::Kind", tags = "1, 2, 3, 4, 5, 6")]
    pub kind: Option<value::Kind>,
}

/// `google.protobuf.ListValue`, a JSON array.
#[derive(Clone, PartialEq, prost::Message)]
pub struct ListValue {
    /// Elements of the array.
    #[prost(message, repeated, tag = "1")]
    pub values: Vec<Value>,
}

/// `google.protobuf.NullValue`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
#[repr(i32)]
pub enum NullValue {
    /// The only value.
    NullValue = 0,
}

/// Kinds of [`Value`].
pub mod value {
    /// The oneof of `google.protobuf.Value`.
    #[derive(Clone, PartialEq, prost::Oneof)]
    pub enum Kind {
        /// JSON null.
        #[prost(enumeration = "super::NullValue", tag = "1")]
        NullValue(i32),
        /// JSON number.
        #[prost(double, tag = "2")]
        NumberValue(f64),
        /// JSON string.
        #[prost(string, tag = "3")]
        StringValue(String),
        /// JSON boolean.
        #[prost(bool, tag = "4")]
        BoolValue(bool),
        /// JSON object.
        #[prost(message, tag = "5")]
        StructValue(super::Struct),
        /// JSON array.
        #[prost(message, tag = "6")]
        ListValue(super::ListValue),
    }
}

/// Largest integer a double holds exactly.
const MAX_EXACT: f64 = 9_007_199_254_740_992.0;

impl From<serde_json::Value> for Value {
    fn from(v: serde_json::Value) -> Self {
        let kind = match v {
            serde_json::Value::Null => value::Kind::NullValue(0),
            serde_json::Value::Bool(b) => value::Kind::BoolValue(b),
            serde_json::Value::Number(n) => value::Kind::NumberValue(n.as_f64().unwrap_or(0.0)),
            serde_json::Value::String(s) => value::Kind::StringValue(s),
            serde_json::Value::Array(a) => value::Kind::ListValue(ListValue {
                values: a.into_iter().map(Value::from).collect(),
            }),
            serde_json::Value::Object(o) => value::Kind::StructValue(Struct::from(o)),
        };
        Value { kind: Some(kind) }
    }
}

impl From<serde_json::Map<String, serde_json::Value>> for Struct {
    fn from(o: serde_json::Map<String, serde_json::Value>) -> Self {
        Struct {
            fields: o.into_iter().map(|(k, v)| (k, Value::from(v))).collect(),
        }
    }
}

impl From<Value> for serde_json::Value {
    fn from(v: Value) -> Self {
        match v.kind {
            None | Some(value::Kind::NullValue(_)) => serde_json::Value::Null,
            Some(value::Kind::BoolValue(b)) => serde_json::Value::Bool(b),
            Some(value::Kind::NumberValue(n)) => number(n),
            Some(value::Kind::StringValue(s)) => serde_json::Value::String(s),
            Some(value::Kind::StructValue(s)) => serde_json::Value::Object(s.into()),
            Some(value::Kind::ListValue(l)) => {
                serde_json::Value::Array(l.values.into_iter().map(Into::into).collect())
            }
        }
    }
}

impl From<Struct> for serde_json::Map<String, serde_json::Value> {
    fn from(s: Struct) -> Self {
        s.fields.into_iter().map(|(k, v)| (k, v.into())).collect()
    }
}

fn number(n: f64) -> serde_json::Value {
    if n.fract() == 0.0 && n.abs() < MAX_EXACT {
        return serde_json::Value::from(n as i64);
    }
    serde_json::Number::from_f64(n).map_or(serde_json::Value::Null, serde_json::Value::Number)
}

impl Serialize for Value {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match &self.kind {
            None | Some(value::Kind::NullValue(_)) => serializer.serialize_unit(),
            Some(value::Kind::BoolValue(b)) => serializer.serialize_bool(*b),
            Some(value::Kind::NumberValue(n)) => {
                if n.fract() == 0.0 && n.abs() < MAX_EXACT {
                    serializer.serialize_i64(*n as i64)
                } else {
                    serializer.serialize_f64(*n)
                }
            }
            Some(value::Kind::StringValue(s)) => serializer.serialize_str(s),
            Some(value::Kind::StructValue(s)) => s.serialize(serializer),
            Some(value::Kind::ListValue(l)) => l.serialize(serializer),
        }
    }
}

impl Serialize for Struct {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.fields.len()))?;
        for (k, v) in &self.fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

impl Serialize for ListValue {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.values.len()))?;
        for v in &self.values {
            seq.serialize_element(v)?;
        }
        seq.end()
    }
}

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("any JSON value")
    }

    fn visit_unit<E: de::Error>(self) -> Result<Value, E> {
        Ok(Value {
            kind: Some(value::Kind::NullValue(0)),
        })
    }

    fn visit_none<E: de::Error>(self) -> Result<Value, E> {
        self.visit_unit()
    }

    fn visit_bool<E: de::Error>(self, v: bool) -> Result<Value, E> {
        Ok(Value {
            kind: Some(value::Kind::BoolValue(v)),
        })
    }

    fn visit_i64<E: de::Error>(self, v: i64) -> Result<Value, E> {
        self.visit_f64(v as f64)
    }

    fn visit_u64<E: de::Error>(self, v: u64) -> Result<Value, E> {
        self.visit_f64(v as f64)
    }

    fn visit_f64<E: de::Error>(self, v: f64) -> Result<Value, E> {
        Ok(Value {
            kind: Some(value::Kind::NumberValue(v)),
        })
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Value, E> {
        Ok(Value {
            kind: Some(value::Kind::StringValue(v.to_owned())),
        })
    }

    fn visit_string<E: de::Error>(self, v: String) -> Result<Value, E> {
        Ok(Value {
            kind: Some(value::Kind::StringValue(v)),
        })
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Value, A::Error> {
        let mut values = Vec::new();
        while let Some(v) = seq.next_element::<Value>()? {
            values.push(v);
        }
        Ok(Value {
            kind: Some(value::Kind::ListValue(ListValue { values })),
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Value, A::Error> {
        let mut fields = BTreeMap::new();
        while let Some((k, v)) = map.next_entry::<String, Value>()? {
            fields.insert(k, v);
        }
        Ok(Value {
            kind: Some(value::Kind::StructValue(Struct { fields })),
        })
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(ValueVisitor)
    }
}

impl<'de> Deserialize<'de> for Struct {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match Value::deserialize(deserializer)?.kind {
            Some(value::Kind::StructValue(s)) => Ok(s),
            Some(value::Kind::NullValue(_)) | None => Ok(Struct::default()),
            _ => Err(de::Error::custom("expected a JSON object")),
        }
    }
}

impl<'de> Deserialize<'de> for ListValue {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match Value::deserialize(deserializer)?.kind {
            Some(value::Kind::ListValue(l)) => Ok(l),
            Some(value::Kind::NullValue(_)) | None => Ok(ListValue::default()),
            _ => Err(de::Error::custom("expected a JSON array")),
        }
    }
}
