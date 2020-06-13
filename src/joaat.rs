use std::num::Wrapping;

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
        assert!(is_preimage("adder", hash));
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
pub fn is_preimage(input: &str, target: u32) -> bool {
    let mut hash = Wrapping(target);

    for (_, byte) in input.bytes().rev().enumerate() {
        // for every byte backwards, we are undoing some of the hash
        hash ^= (hash >> 6) ^ (hash >> 12) ^ (hash >> 18) ^ (hash >> 24) ^ (hash >> 30);
        hash *= Wrapping(0xC00F_FC01);
        hash -= Wrapping(byte as u32);
    }

    // if after the last byte, we're left at 0, then it's a match
    hash.0 == 0
}

/// Find all possible pre-images of a length of a hash
pub fn find_preimages(target: u32, input_length: usize, characters: Vec<char>) -> Vec<String> {
    let hash = undo_finalization(target);

    let (min_char, max_char) = get_character_bounds(&characters);

    let characters_ref = &characters;

    let mut output = Vec::new();
    let mut buffer = vec!['\0'; input_length];

    reverse(hash, &mut buffer, input_length - 1, characters_ref, (min_char, max_char), None, &mut output);

    output
}

fn reverse(hash: u32, buffer: &mut [char], depth: usize, characters: &[char], (min_char, max_char): (u32, u32), force_char: Option<char>, output: &mut Vec<String>) {
    let mut hash = Wrapping(hash);

    // invert the hash mixing step
    hash ^= (hash >> 6) ^ (hash >> 12) ^ (hash >> 18) ^ (hash >> 24) ^ (hash >> 30);
    hash *= Wrapping(0xC00F_FC01); // inverse of hash += hash << 10;

    // for the lowest three levels, abort early if no solution is possible
    let hash_val = hash.0;
    match depth {
        0 => {
            if hash_val < min_char || hash_val > max_char {
                return;
            }

            // we've found a valid preimage
            buffer[0] = hash_val as u8 as char;

            output.push(buffer.iter().collect());

            return;
        }
        1 => {
            if hash_val > max_char * 1_043 {
                return;
            }
        }
        2 => {
            if hash_val > max_char * 1_084_746 {
                return;
            }
        }
        _ => {}
    }

    let mut recur = |ch| {
        buffer[depth] = ch;
        reverse((hash - Wrapping(ch as u32)).0, buffer, depth - 1, &characters, (min_char, max_char), None, output);
    };

    if let Some(force_char) = force_char {
        // we should use a specific char
        recur(force_char);
    } else {
        // try all possible values for this byte
        for &ch in characters {
            recur(ch);
        }
    }
}

fn get_character_bounds(characters: &[char]) -> (u32, u32) {
    (
        (*characters.iter().min().unwrap()) as u32,
        (*characters.iter().max().unwrap()) as u32
    )
}

/// Calculates some things
pub fn calculate_brute_boundaries(characters: &[char], length: usize) {

}
