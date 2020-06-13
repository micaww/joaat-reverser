use std::num::Wrapping;
use crossbeam::thread;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashing() {
        assert_eq!(hash("hello"), 0xc8fd181b);
        assert_eq!(hash("thisisasuperlongstringanditcontainssomereallycoolstuff"), 0x7adcfe15);
        assert_eq!(hash("Здравствуйте"), 0xe69f4c94);
    }

    #[test]
    fn reverse_adder() {
        let hash = undo_finalization(0xb779a091);
        assert!(is_preimage("adder", hash, &'a', &'z'));
    }
}

/// Hashes the given string.
pub fn hash(input: &str) -> u32 {
    let mut hash = Wrapping(0u32);

    for &byte in input.as_bytes() {
        hash += Wrapping(u32::from(byte));
        hash += hash << 10;
        hash ^= hash >> 6;
    }

    hash += hash << 3;
    hash ^= hash >> 11;
    hash += hash << 15;

    hash.0
}

/// Undoes the finalization step of a target hash
pub fn undo_finalization(hash: u32) -> u32 {
    let mut hash = Wrapping(hash);

    hash *= Wrapping(0x3FFF_8001); // inverse of hash += hash << 15;
    hash ^= (hash >> 11) ^ (hash >> 22);
    hash *= Wrapping(0x38E3_8E39); // inverse of hash += hash << 3;

    hash.0
}

/// Checks whether an input is a valid pre-image of an unfinalized hash
pub fn is_preimage(input: &str, target: u32, min_char: &char, max_char: &char) -> bool {
    let mut hash = Wrapping(target);
    let len = input.len();

    for (i, byte) in input.bytes().rev().enumerate() {
        let i = (len - 1) - i;

        hash ^= (hash >> 6) ^ (hash >> 12) ^ (hash >> 18) ^ (hash >> 24) ^ (hash >> 30);
        hash *= Wrapping(0xC00F_FC01);
        hash -= Wrapping(byte as u32);
    }

    hash.0 == 0
}