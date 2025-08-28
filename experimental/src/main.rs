use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn maybe_number_ex()
{
    let maybe_number = Some(42); // or None in some cases
    if let Some(num) = maybe_number {
        println!("The number is {}", num);
    } else {
        println!("No number provided.");
    }
}

fn file_reader_application()
{
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    // if the file name does not have an extension, do not read it
    // println!("result: {}", file_name.find("."));
    if file_name.find(".") == None
    {
        println!("Please provide a file name with an extension\nInput was: {}", file_name);
    }
    else 
    {
        let file = File::open(file_name);

        let file = match file {
            Ok(file) => file,
            Err(error) => {
                match error.kind() {
                    std::io::ErrorKind::NotFound => {
                        panic!("File not found: {}", error)
                    }
                    _ => {
                        panic!("Error opening file: {}", error)
                    }
                }
            }
        };
        
        let reader = BufReader::new(file);
        for line in reader.lines() {
            match line {
                Ok(line) => println!("{}", line),
                Err(error) => {
                    panic!("Error reading line: {}", error)
                }
            }
        }
    }
}

fn main() {
    maybe_number_ex();

    file_reader_application();
}
