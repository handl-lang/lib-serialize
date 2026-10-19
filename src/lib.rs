//! # handl-lib-serialize
//!
//! `serialize` — generic serialization contract. `.hnl` `serialize`/
//! `deserialize` dispatch on the format name (`"json"`, `"yaml"`, `"toml"`)
//! or `{ type: "serializer", name }` records. The `Serializer`/`Encoder`/
//! `Decoder`/`SerializeError` *contract types* remain in Rust for the
//! native serializer impls in `plugin-lib-stdlib` (toml/yaml ops).
#![allow(dead_code)]

pub mod error;
pub mod serializer;
pub mod traits;

pub use error::SerializeError;
pub use serializer::{Serializer, SerializerRegistry};
pub use traits::{Decoder, Encoder};

/// The handl source declaring the serialize interface.
pub const SERIALIZE_HNL: &str = include_str!("index.hnl");
pub const SERIALIZE_OPS: &[&str] = &["serialize.serialize", "serialize.deserialize"];

/// `.hnl` package source set — index + siblings.
pub const SERIALIZE_FUNCS: &[&str] = &["serialize","deserialize","toml","yaml"];
pub const SERIALIZE_SOURCES: &[(&str, &str, &[&str])] = &[
    ("serialize", include_str!("serialize.hnl"), &["fmt_name","serialize","deserialize","toml","yaml"]),
];
pub const SERIALIZE_FLAT_HNL: &str = include_str!("index_flat.hnl");
