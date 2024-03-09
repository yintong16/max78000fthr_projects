use std::fs::File;
use std::io::{BufRead, BufReader, Write};


fn main() {
    // Open the input text file
    let input_file = match File::open("C:/Users/Yintong Luo/Desktop/eCTF/max78000_fall23/flash_dump/data/_formated_flash_new.txt") {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening input file: {}", err);
            return;
        }
    };

    // Create a reader to read lines from the input file
    let reader = BufReader::new(input_file);

    // Open the output binary file
    let mut output_file = match File::create("C:/Users/Yintong Luo/Desktop/eCTF/max78000_fall23/flash_dump/data/elf_found.txt") {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error creating output file: {}", err);
            return;
        }
    };

    let mut elf = false;
    let mut end = false;
    // Iterate over lines in the input file
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(_) => {
                eprintln!("Error reading line from input file");
                continue;
            }
        };

        // Trim the line and convert hex string to ASCII bytes
        let hex_string = line.trim();
        let ascii_bytes = hex_string.as_bytes();
        if elf == false && end == false{
            for i in 0..ascii_bytes.len() {
                if ascii_bytes[i] == 0x7F && ascii_bytes[i+1] == 0x45 && ascii_bytes[i+2] == 0x4C && ascii_bytes[i+3] == 0x46 {
                    elf = true;
                    match output_file.write_all(hex_string.as_bytes()) {
                        Ok(_) => (),
                        Err(err) => {
                            eprintln!("Error writing to output file: {}", err);
                            return;
                        }
                    }
                    break;
                }
            }
        }
        else if elf == true && end == false {
            let mut counter = 0;
            for i in 0..ascii_bytes.len() {
                if ascii_bytes[i] == 0xFF {
                    counter += 1;
                }
                if counter >= 8 {
                    end = true;
                    match output_file.write_all(hex_string.as_bytes()) {
                        Ok(_) => (),
                        Err(err) => {
                            eprintln!("Error writing to output file: {}", err);
                            return;
                        }
                    }
                    break;
                }
            }
            match output_file.write_all(hex_string.as_bytes()) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("Error writing to output file: {}", err);
                    return;
                }
            }
        }
    }

}
