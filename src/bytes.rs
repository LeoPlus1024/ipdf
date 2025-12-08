use std::cmp::{max, min};

pub fn bytes_to_u64(bytes: &[u8]) -> u64 {
    let len = min(bytes.len(), 8);
    let mut value: u64 = 0;
    for i in 0..len {
        let b = bytes[i] - 48;
        value = (value * 10) + b as u64;
    }
    value
}
