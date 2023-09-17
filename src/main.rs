use core::panic;
use std::{fs::File, io::{Read, Write}};

macro_rules! print_and_die {
    ($msg:literal) => {
        println!("{}", $msg);
        std::process::exit(1);
    };
}

//https://wiki.multimedia.cx/index.php?title=YUV4MPEG2 for video support

const N_LSB: usize = 1;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let options = parse_args(args);

    let data_file = open_file(&options.path_to_input);
    let data = read_contents(data_file);

    let medium_file = open_file(&options.path_to_medium);
    let medium = read_contents(medium_file);

    let output = encode_message(&data, &medium, get_file_type(&options.path_to_medium));

    let mut out_file = match File::create("data/output.bmp") {
        Err(reason) => panic!("couldn't create: {}", reason),
        Ok(file) => file,
    };
    let _ = out_file.write_all(&output);
    
    if medium.len() < data.len() * 8 / N_LSB {
        print_and_die!("The medium is not large enough to encode the input file");
    }
}

fn get_file_type(s: &String) -> String {
    let contents: Vec<&str> = s.split('.').collect();
    if contents.len() == 2 {
        return contents[1].to_string();
    } else {
        panic!("Could ascertain file type of {}", s);
    }
}

// returns the 0-indexed start of the pixel data for a given file type
// e.x. the important information in the header of a bmp image is in the first 54 bytes.
//      thus, start encoding at the index 54
fn file_pixel_offset(file_type: &String) -> u8 {
    match file_type.as_str() {
        "bmp" => 54, // definitely right
        "png" => 8, // ! untested rn
        _ => panic!("Unknown file type: {}", file_type)
    }
}

fn encode_message(msg: &Vec<u8>, medium: &Vec<u8>, file_type: String) -> Vec<u8> {
    // choosing to clone because the message will probably be finished before the image runs out of bytes
    // this way only the bytes that need to be changed are changed 
    let mut output: Vec<u8> = medium.clone(); 
    let start = file_pixel_offset(&file_type);

    // denotes which number bit we are at within the current byte of msg (0 indexed)
    let mut bit_index = 0; 
    // denotes which byte within msg we are at
    let mut byte_index = 0;
    for i in (start as usize)..medium.len() {
        let curr_bit = msg[byte_index] & (1 << bit_index); // equals 1 if current bit is 1, otherwise 0

        let mut host_byte = medium[i];
        if host_byte & 1 == 1 && curr_bit == 0 {
            // last bit is 1; change it to 0
            host_byte &= !(1 << 7);
            /*
            E.x. 11001001
               & 11111110 (!(1 << 7))
               ----------
                 11001000
            */
        } else if host_byte & 1 == 0 && curr_bit == 1 {
            // last bit is 0; change it to 1
            host_byte |= 1 << 7;
            /*
            E.x. 11001000
               | 00000001 (1 << 7)
               ----------
                 11001001
            */
        }
        // otherwise the last bit in the byte of the image is already set as needed
        
        output[i] = host_byte;
        // increment bit index and overflow if needed
        bit_index += 1;
        if bit_index > 7 {
            bit_index = 0;
            if byte_index + 1 < msg.len() {
                byte_index += 1;
            } else {
                break;
            }
        }
    }
    
    return output;
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
