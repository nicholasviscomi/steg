use core::panic;
use std::{fs::File, io::{Read, Write}};

macro_rules! print_and_die {
    ($msg:literal) => {
        println!("{}", $msg);
        std::process::exit(1);
    };
}

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

    decode_message(&read_contents(open_file(&String::from("data/output.bmp"))), &String::from("bmp"));
}

fn get_file_type(s: &String) -> String {
    let contents: Vec<&str> = s.split('.').collect();
    if contents.len() == 2 {
        return contents[1].to_string();
    } else {
        panic!("Could not ascertain file type of {}", s);
    }
}

// returns the 0-indexed start of the pixel data for a given file type
// e.x. the important information in the header of a bmp image is in the first 54 bytes.
//      thus, start encoding at the index 54
fn file_pixel_offset(file_type: &String) -> usize {
    match file_type.as_str() {
        "bmp" => 54, // definitely right
        "png" => 8, // ! untested rn
        _ => panic!("Unknown file type: {}", file_type)
    }
}

fn update_host_byte(mut host_byte: u8, new_bit: u8) -> u8 {
    if new_bit != 0 && new_bit != 1 { panic!("New Bit was not 0 or 1"); }

    host_byte = (host_byte >> 1) << 1; // knock off the least significant bit and replace with a 0
    host_byte |= new_bit; // change ending 0 to a 1 if needed

    return host_byte;
}

fn decode_message(medium: &Vec<u8>, file_type: &String) -> Vec<u8> {
    let mut start = file_pixel_offset(file_type);  
    let mut msg_len = 0;
    for i in start..(start + 32) {
        let bit = medium[i] & 1; // returns whether the LSB is 0 or 1
        if bit == 1 {
            msg_len |= bit << (i - start);
        } // otherwise the bit is already 0 from initialization
    }
    println!("\nMSG LEN: {}", msg_len);

    let mut output: Vec<u8> = vec![];
    start += 32;
    let mut curr_byte: u8 = !(0 << 7);
    let mut bit_index: u8 = 0;
    for i in start..(start + ((msg_len*8) as usize)) {
        let bit = medium[i] & 1;
        if bit == 0 {
            print!("{}, {}: \t", bit, bit_index);
            print!("{:b} ", curr_byte);
            curr_byte &= !(1 << bit_index);
            print!("{:b}\n", curr_byte);
        }

        bit_index += 1;
        if bit_index > 7 {
            bit_index = 0;
            output.push(curr_byte);
            curr_byte = !(0 << 7); // ! critical line
        }
    }
    println!();
    for b in &output {
        print!("{:b} ", b);
    }
    let message = match String::from_utf8(output.clone()) {
        Ok(v) => v,
        Err(e) => panic!("Invalid utf8 sequence: {}", e)
    };
    println!("\nMESSAGE: {}", message);
    
    return output;
}

fn encode_message(msg: &Vec<u8>, medium: &Vec<u8>, file_type: String) -> Vec<u8> {
    // choosing to clone because the message will probably be finished before the image runs out of bytes
    // this way only the bytes that need to be changed are changed 
    let mut output: Vec<u8> = medium.clone(); 
    let start = file_pixel_offset(&file_type);

    // Encode a 32 bit 
    let len = msg.len() as u32;
    for i in start..(start + 32) {
        // println!("{:b}, {}th = {}", len, i - start, (len >> (i - start)) & 1);
        output[i] = update_host_byte(medium[i], ((len >> (i - start)) & 1) as u8);

        //? seems to be encoding the number correctly
        // println!("{} ({:b}) vs {} ({:b})", medium[i], medium[i], output[i], output[i]);
    }

    // denotes which number bit we are at within the current byte of msg (0 indexed)
    let mut bit_index: usize = 0;
    // denotes which byte within msg we are at
    let mut byte_index = 0;
    for i in (start + 32)..medium.len() {
        let curr_bit = (msg[byte_index] >> bit_index) & 1; // equals 1 if current bit is 1, otherwise 0

        output[i] = update_host_byte(medium[i], curr_bit);

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
                "-i" | "--input" | "--decode" => {
                    i += 1; // the next argument should be the path to the input
                    assert!(i < args.len(), "No path to file was specified after \"{}\"", &arg);
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
