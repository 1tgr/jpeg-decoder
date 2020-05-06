//! This crate contains a JPEG decoder.
//!
//! # Examples
//!
//! ```
//! use jpeg_decoder::Decoder;
//! use std::fs::File;
//! use std::io::BufReader;
//!
//! let file = File::open("tests/reftest/images/extraneous-data.jpg").expect("failed to open file");
//! let mut decoder = Decoder::new(BufReader::new(file));
//! let pixels = decoder.decode().expect("failed to decode image");
//! let metadata = decoder.info().unwrap();
//! ```
//!
//! Get metadata from a file without decoding it:
//!
//! ```
//! use jpeg_decoder::Decoder;
//! use std::fs::File;
//! use std::io::BufReader;
//!
//! let file = File::open("tests/reftest/images/extraneous-data.jpg").expect("failed to open file");
//! let mut decoder = Decoder::new(BufReader::new(file));
//! decoder.read_info().expect("failed to read metadata");
//! let metadata = decoder.info().unwrap();
//! ```

#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

extern crate byteorder;

#[cfg(feature = "std")]
extern crate core;

#[cfg(not(feature = "std"))]
extern crate libm;

#[cfg(feature = "rayon")]
extern crate rayon;

pub use decoder::{Decoder, ImageInfo, PixelFormat};
pub use error::{Error, UnsupportedFeature};

#[cfg(feature = "std")]
mod compat {
    use error::{Error, Result};
    use std::io;

    pub use std::io::Read;
    pub use std::f32::ceil as ceilf;
    pub use std::f32::trunc as truncf;

    pub fn skip_bytes<R: Read>(reader: &mut R, length: usize) -> Result<()> {
        let length = length as u64;
        let to_skip = &mut reader.by_ref().take(length);
        let copied = io::copy(to_skip, &mut io::sink())?;
        if copied < length {
            Err(Error::Io(io::ErrorKind::UnexpectedEof.into()))
        } else {
            Ok(())
        }
    }
}

#[cfg(not(feature = "std"))]
mod compat {
    use error::Result;

    pub use read::Read;
    pub use libm::{ceilf, truncf};

    pub fn skip_bytes<R: Read>(reader: &mut R, length: usize) -> Result<()> {
        reader.skip_bytes(length)
    }
}

#[cfg(not(feature = "std"))]
pub use read::Read;

mod decoder;
mod error;
mod huffman;
mod idct;
mod marker;
mod parser;
mod read;
mod upsampler;
mod worker;
