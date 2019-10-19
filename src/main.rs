extern crate clap;
extern crate hex;

use clap::{App, Arg, ArgMatches};

const MIN_CHAR: char = 'a';
const MAX_CHAR: char = 'z';

fn main() {
    let matches = App::new("joaat-reverser")
        .version("0.1.0")
        .author("Micah Allen")
        .arg(Arg::with_name("TARGET")
            .help("The target hash to reverse")
            .required(true)
            .index(1))
        .arg(Arg::with_name("INPUT_LENGTH")
            .help("The length of the input to find")
            .required(true)
            .index(2))
        .get_matches();

    if let Ok((target, input_length)) = parse_args(matches) {
        let mut buffer = vec!['\0'; input_length];

        let hash = undo_finalization(target);

        // then search recursively until we find all matching inputs
        reverse(hash, &mut buffer, input_length - 1);
    }
}

fn reverse(mut hash: u32, buffer: &mut[char], depth: usize) {
    // invert the hash mixing step
    hash ^= (hash >> 6) ^ (hash >> 12) ^ (hash >> 18) ^ (hash >> 24) ^ (hash >> 30);
    hash = hash.wrapping_mul(0xC00FFC01); // inverse of hash += hash << 10;

    // for the lowest three levels, abort early if no solution is possible
    let max_char = MAX_CHAR as u32;
    let min_char = MIN_CHAR as u32;
    match depth {
        0 => {
            if hash < min_char || hash > max_char {
                return;
            }

            buffer[0] = hash as u8 as char;
            found_possibility(buffer);
            return;
        }
        1 => {
            if hash > max_char * 1043 {
                return;
            }
        }
        2 => {
            if hash > max_char * 1084746 {
                return;
            }
        }
        _ => {}
    }

    // try all possible values for this byte
    for ch in min_char..max_char {
        buffer[depth] = ch as u8 as char;
        reverse(hash.wrapping_sub(ch), buffer, depth - 1);
    }
}

fn parse_args(matches: ArgMatches) -> Result<(u32, usize), ()> {
    let target_string = matches.value_of("TARGET").unwrap();
    let target = match target_string.parse::<u32>() {
        Ok(a) => a,
        Err(_) => {
             hex::decode(target_string);
            return Err(());
        }
    };
    let input_length = match matches.value_of("INPUT_LENGTH").unwrap().parse::<usize>() {
        Ok(a) => a,
        Err(_) => {
            println!("Enter a valid expected input length.");
            return Err(());
        }
    };

    if input_length < 1 {
        println!("Input length must be greater than 0.");
        return Err(());
    }

    Ok((target, input_length))
}

fn undo_finalization(mut hash: u32) -> u32 {
    hash = hash.wrapping_mul(0x3FFF8001); // inverse of hash += hash << 15;
    hash ^= (hash >> 11) ^ (hash >> 22);
    hash = hash.wrapping_mul(0x38E38E39); // inverse of hash += hash << 3;
    hash
}

fn found_possibility(input: &[char]){
    let str: String = input.iter().collect();
    println!("{}", str);
}