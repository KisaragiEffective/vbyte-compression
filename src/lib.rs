#![cfg_attr(not(feature = "std"), no_std)]
#![deny(clippy::all, clippy::std_instead_of_alloc, clippy::std_instead_of_core)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::cast_lossless)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(not(feature = "alloc"))]
compile_error!("This crate requires allocation");

use core::fmt::{Display, Formatter};
use alloc::vec::Vec;
// for vec! macro
use alloc::vec;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum Error {
    UnexpectedEndOfInput(()),
}

impl Display for Error {
    #[allow(clippy::ignored_unit_patterns)]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnexpectedEndOfInput(_) => f.write_str("End of input reached")
        }
    }
}

#[must_use]
pub fn compress(mut val: u64) -> Vec<u8> {
    if val == 0 {
        return vec![0];
    }

    // In worst case, we need 10 bytes to encode 0xFFFF_FFFF_FFFF_FFFF
    let mut v = Vec::with_capacity(10);

    while val > 0 {
        // take the least significant 7 bits of the value
        let mut byte = (val & 0b111_1111) as u8;

        // shift to see next LSB groups
        val >>= 7;

        // Set the `follow` byte,
        // if there remains information to be encoded
        if val > 0 {
            byte |= 0b1000_0000;
        }

        v.push(byte);
    }

    v.shrink_to_fit();
    v
}

#[must_use]
pub fn compress_list(vs: &[u64]) -> Vec<u8> {
    // we need vs.len() bytes at least
    let mut buffer = Vec::with_capacity(vs.len());
    for v in vs {
        let c = compress(*v);
        buffer.extend(c);
    }

    buffer
}

/// decompresses a string, returning the rest of the input as second argument.
/// # Errors
/// Error occurs when more data was expected
pub fn decompress(data: &[u8]) -> Result<(u64, &[u8]), Error> {
    let mut val = 0u64;

    for i in 0..data.len() {
        let byte = data[i];
        let byte_index = i as u64 * 7;

        // update value
        {
            // cut of leading byte, if present
            let byte = (byte & 0b0111_1111) as u64;
            // decode proper position in value
            let byte = byte << byte_index;
            // assign to value
            val |= byte;
        }

        // continue?
        if byte & 0b1000_0000 != 0 {
            continue;
        }

        // end of value is reached, return

        let i = i + 1;
        let rest = &data[i..];
        return Ok((val, rest));
    }

    Err(Error::UnexpectedEndOfInput(()))
}

/// decompresses a string, returning the rest of the input as second argument.
/// # Errors
/// Error occurs when more data was expected
pub fn decompress_n<const N: usize>(mut data: &[u8]) -> Result<([u64; N], &[u8]), Error> {
    let mut out = [0; N];
    for entry in &mut out {
        let (val, rest) = decompress(data)?;
        *entry = val;
        data = rest;
    }

    Ok((out, data))
}

/// decompresses a string, returning the rest of the input as second argument.
/// # Errors
/// Error occurs when more data was expected
pub fn decompress_list(mut data: &[u8]) -> Result<Vec<u64>, Error> {
    let mut out = Vec::with_capacity(data.len());
    while !data.is_empty() {
        let (val, rest) = decompress(data)?;
        out.push(val);
        data = rest;
    }

    Ok(out)
}
