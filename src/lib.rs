#[cfg(test)]
mod tests;

pub fn compress(mut val: u64) -> Vec<u8> {
    if val == 0 {
        return vec![0];
    }

    let mut v = Vec::new();

    while val > 0 {
        // take the first 7 bytes of the value
        let mut byte = (val & 0b111_1111) as u8;

        // decrement value
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

pub fn compress_list(vs: &[u64]) -> Vec<u8> {
    let mut buffer = Vec::new();
    for v in vs {
        let c = compress(*v);
        buffer.extend(c);
    }

    buffer
}

/// decompresses a string, returning the rest of the input as second argument.
/// If an error occured, it means that more data was expected
pub fn decompress(data: &[u8]) -> Result<(u64, &[u8]), &str> {
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

    Err("end of input reached")
}

pub fn decompress_n<const N: usize>(mut data: &[u8]) -> Result<([u64; N], &[u8]), &str> {
    let mut out = [0; N];
    for entry in out.iter_mut() {
        let (val, rest) = decompress(data)?;
        *entry = val;
        data = rest;
    }

    Ok((out, data))
}

pub fn decompress_list(mut data: &[u8]) -> Result<Vec<u64>, &str> {
    let mut out = Vec::with_capacity(data.len());
    while !data.is_empty() {
        let (val, rest) = decompress(data)?;
        out.push(val);
        data = rest;
    }

    Ok(out)
}
