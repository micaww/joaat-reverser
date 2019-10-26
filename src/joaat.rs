use std::num::Wrapping;

const MIN_CHAR: char = 'a';
const MAX_CHAR: char = 'z';

#[cfg(test)]
mod tests {
    use super::hash;

    #[test]
    fn hashing() {
        assert_eq!(hash("hello"), 0xc8fd181b);
        assert_eq!(hash("thisisasuperlongstringanditcontainssomereallycoolstuff"), 0x7adcfe15);
        assert_eq!(hash("Здравствуйте"), 0xe69f4c94);
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

/// Find all possible pre-images of a length of a hash
pub fn find_preimages(target: u32, input_length: usize) -> Vec<String> {
    let mut hash = Wrapping(target);

    // undo hash finalization
    hash *= Wrapping(0x3FFF_8001); // inverse of hash += hash << 15;
    hash ^= (hash >> 11) ^ (hash >> 22);
    hash *= Wrapping(0x38E3_8E39); // inverse of hash += hash << 3;

    let mut output = Vec::new();

    let mut buffer = vec!['\0'; input_length];

    reverse_recursive(hash.0, &mut buffer, input_length - 1, &mut output);

    output
}

fn reverse_recursive(hash: u32, buffer: &mut [char], depth: usize, output: &mut Vec<String>) {
    let mut hash = Wrapping(hash);

    // invert the hash mixing step
    hash ^= (hash >> 6) ^ (hash >> 12) ^ (hash >> 18) ^ (hash >> 24) ^ (hash >> 30);
    hash *= Wrapping(0xC00F_FC01); // inverse of hash += hash << 10;

    // for the lowest three levels, abort early if no solution is possible
    let max_char = MAX_CHAR as u32;
    let min_char = MIN_CHAR as u32;
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
            if hash_val > max_char * 1043 {
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

    // try all possible values for this byte
    for ch in min_char..max_char {
        buffer[depth] = ch as u8 as char;
        reverse_recursive((hash - Wrapping(ch)).0, buffer, depth - 1, output);
    }
}