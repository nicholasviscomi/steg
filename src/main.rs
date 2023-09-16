use std::{fs::File, io::Read};

macro_rules! print_and_die {
    ($msg:literal) => {
        println!("{}", $msg);
        std::process::exit(1);
    };
}

const N_LSB: usize = 1;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let options = parse_args(args);

    let data_file = open_file(&options.path_to_input);
    let data = read_contents(data_file);

    let medium_file = open_file(&options.path_to_medium);
    let medium = read_contents(medium_file);

    if medium.len() < data.len() * 8 / N_LSB {
        print_and_die!("The medium is not large enough to encode the input file");
    }
}

fn open_file(path: &String) -> File {
    match File::open(&path) {
        Ok(file) => file,
        Err(reason) => panic!("Could not open file ({}): {}", reason, path)
    }
}

fn read_contents(mut file: File) -> Vec<u8> {
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Reading of file was interupted");
    return buf;
}

struct Options {
    path_to_input: String,
    path_to_medium: String,
} 

// will return a valid options struct or panic
// if it returns successfully, you can assert that the values are not none
fn parse_args(args: Vec<String>) -> Options {
    if args.len() == 1 {
        print_and_die!("No arguments provided: run steg --help");
    }

    let mut i = 1; // REMEMBER: first argument is the path to the executable
    let mut options = Options { path_to_input: String::from(""), path_to_medium: String::from("") };
    while i < args.len() {
        let arg = &args[i];
        if arg.chars().next().unwrap() == '-' {
            match arg.as_str() {
                "-h" | "--help" => {
                    print_and_die!("Don't be an idiot");
                },
                "-i" | "--input" => {
                    i += 1; // the next argument should be the path to the input
                    assert!(i < args.len(), "No path to input file was specified");
                    options.path_to_input.push_str(&args[i]);
                }, 
                "-m" | "--medium" => {
                    i += 1;
                    assert!(i < args.len(), "No path to file to be used as a medium was specified");
                    options.path_to_medium.push_str(&args[i]);
                }, 
                
                _ => { print_and_die!("Unknown flag: run steg --help for a list of commands"); },
            }
        } else {
            print_and_die!("Unknown argument \"{arg}\": run steg --help for a list of commands");
        }
        i += 1; // jump to next arg to prepare for next iteration
    }

    return options;
}
