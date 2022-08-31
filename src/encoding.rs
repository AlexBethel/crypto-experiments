//! Input encoding and decoding.

use std::iter::{from_fn, repeat};

/// Pads the input to a multiple of `multiple`.
pub fn pad(mut input: Vec<u8>, multiple: usize) -> Vec<u8> {
    let needed_bytes = multiple - (input.len() % multiple);
    input.extend(repeat(needed_bytes as u8).take(needed_bytes));
    input
}

/// Creates an iterator over blocks of `n` bytes of the input.
pub fn iter_bits(mut input: &[u8], n: usize) -> impl Iterator<Item = Vec<bool>> + '_ {
    from_fn(move || {
        if input.is_empty() {
            None
        } else {
            let bits = block_to_bits(&input[0..n]);
            input = &input[n..];
            Some(bits)
        }
    })
}

fn block_to_bits(block: &[u8]) -> Vec<bool> {
    let mut data: Vec<bool> = Vec::with_capacity(block.len() * 8);
    for byte in block {
        for bit in (0..8).rev() {
            data.push(*byte & (1 << bit) != 0)
        }
    }
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pad() {
        let a = vec![1, 2, 3, 4, 5];
        assert_eq!(pad(a, 4), vec![1, 2, 3, 4, 5, 3, 3, 3]);

        let b = vec![1, 2, 3, 4];
        assert_eq!(pad(b, 4), vec![1, 2, 3, 4, 4, 4, 4, 4]);
    }

    #[test]
    fn test_iter_bits() {
        let data = vec![1, 2, 3];
        let mut bit_iter = iter_bits(&data, 1);
        assert_eq!(
            bit_iter.next(),
            Some(vec![false, false, false, false, false, false, false, true])
        );
        assert_eq!(
            bit_iter.next(),
            Some(vec![false, false, false, false, false, false, true, false])
        );
        assert_eq!(
            bit_iter.next(),
            Some(vec![false, false, false, false, false, false, true, true])
        );
    }
}
