use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let options = parse_args(args);
    println!("Path to data: {}\nPath to where it will be encoded: {}", options.path_to_input, options.path_to_medium);
}

struct Options {
    path_to_input: String,
    path_to_medium: String,
} 

// will return a valid options struct or panic
// if it returns successfully, you can assert that the values are not none
fn parse_args(args: Vec<String>) -> Options {
    if args.len() == 1 {
        panic!("No arguments provided: run steg --help");
    }

    let mut i = 1; // REMEMBER: first argument is the path to the executable
    let mut options = Options { path_to_input: String::from(""), path_to_medium: String::from("") };
    while i < args.len() {
        let arg = &args[i];
        if arg.chars().next().unwrap() == '-' {
            match arg.as_str() {
                "-h" | "--help" => {
                    println!("Don't be an idiot");
                    std::process::exit(1);
                },
                "-i" | "--input" => {
                    i += 1; // the next argument should be the path to the input
                    assert!(i < args.len(), "No path to input file was specified");
                    options.path_to_input = args[i].to_string();
                }, 
                "-m" | "--medium" => {
                    i += 1;
                    assert!(i < args.len(), "No path to file to be used as a medium was specified");
                    options.path_to_medium = args[i].to_string();
                }, 
                
                _ => panic!("Unknown flag: run steg --help for a list of commands"),
            }
        } else {
            panic!("Unknown argument \"{}\": run steg --help for a list of commands", arg);
        }
        i += 1; // jump to next arg to prepare for next iteration
    }

    return options;
}
