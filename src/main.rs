mod joaat;

use clap::{App, Arg, ArgMatches};

const ALPHANUM_CHARS: [char; 62] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
];

enum Action {
    None,
    Hash(String),
    Reverse(u32, usize, Vec<char>)
}

fn main() {
    let mut app = App::new("joaat-reverser")
        .author("Micah Allen")
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .value_name("INPUT")
            .takes_value(true)
            .help("The string to hash"))
        .arg(Arg::with_name("target")
            .short("t")
            .long("target")
            .value_name("TARGET_HASH")
            .takes_value(true)
            .help("The hash to reverse"))
        .arg(Arg::with_name("length")
            .short("l")
            .long("length")
            .value_name("INPUT_LENGTH")
            .takes_value(true)
            .help("The length of the pre-images to find"))
        .arg(Arg::with_name("characters")
            .short("c")
            .long("characters")
            .value_name("CHARACTERS")
            .takes_value(true)
            .help("A list of characters to try. Default is alphanumeric."));

    let matches = app.clone().get_matches();

    let action = parse_args(&matches);

    match action {
        Action::Hash(input) => {
            let hash = joaat::hash(&input);

            println!("Jenkins' one-at-a-time hash for \"{}\":", input);
            println!("Hexadecimal: 0x{:X}", hash);
            println!("Decimal: {}", hash);
        },
        Action::Reverse(target, len, characters) => {
            let start = std::time::Instant::now();

            let preimages = joaat::find_preimages(target, len, characters);

            for preimage in preimages {
                println!("{}", preimage);
            }

            println!("Finished! Took {:?}", start.elapsed());
        },
        Action::None => {
            app.print_help().unwrap();
        }
    }
}

fn parse_args(matches: &ArgMatches) -> Action {
    if let Some(input) = matches.value_of("input") {
        Action::Hash(String::from(input))
    } else {
        let has_target = matches.is_present("target");
        let has_length = matches.is_present("length");

        if has_target && has_length {
            let target = matches.value_of("target").unwrap().parse::<u32>().expect("Please enter a valid hash to reverse.");
            let input_length = matches.value_of("length").unwrap().parse::<usize>().expect("Please enter a valid input length.");

            if input_length < 1 {
                panic!("Input length must be greater than 0.");
            }

            let characters: Vec<char> = if let Some(chars) = matches.value_of("characters") {
                chars.chars().collect()
            } else {
                ALPHANUM_CHARS.to_owned().to_vec()
            };

            Action::Reverse(target, input_length, characters)
        } else if has_target && !has_length {
            panic!("You must provide an input length to reverse a hash using the -l argument.");
        } else if has_length && !has_target {
            panic!("You must provide the hash to be reversed using the -t argument.");
        } else {
            Action::None
        }
    }
}