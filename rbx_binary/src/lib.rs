//! Implementation of Roblox's binary model (rbxm) and place (rbxl) file
//! formats.

#![deny(missing_docs)]

mod cframe;
mod chunk;
mod core;
mod deserializer;
mod serializer;
mod types;

#[cfg(any(test, feature = "unstable_text_format"))]
mod text_deserializer;

#[cfg(test)]
mod tests;

use std::io::{Read, Write};

use rbx_dom_weak::{types::Ref, WeakDom};

use crate::serializer::encode;

/// An unstable textual format that can be used to debug binary models.
#[cfg(feature = "unstable_text_format")]
pub mod text_format {
    pub use crate::text_deserializer::*;
}

pub use crate::{
    deserializer::{Deserializer, Error as DecodeError},
    serializer::Error as EncodeError,
};

/// Deserialize a Roblox binary model or place from a stream.
pub fn from_reader<R: Read>(reader: R) -> Result<WeakDom, DecodeError> {
    Deserializer::new().deserialize(reader)
}

/// Serializes a subset of the given DOM to a binary format model or place,
/// writing to something that implements the `std::io::Write` trait.
pub fn to_writer_default<W: Write>(
    writer: W,
    dom: &WeakDom,
    refs: &[Ref],
) -> Result<(), EncodeError> {
    encode(dom, refs, writer)
}
