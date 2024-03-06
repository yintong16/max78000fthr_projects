use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn hex_to_ascii(hex: &str) -> Option<Vec<u8>> {
    if hex.len() != 32 {
        return None;
    }

    let mut ascii: Vec<u8> = Vec::with_capacity(16);

    for i in 0..16 {
        let slice = &hex[i * 2..(i + 1) * 2];
        let byte = u8::from_str_radix(slice, 16).ok()?;
        ascii.push(byte);
    }

    Some(ascii)
}

fn main() {
    // Open the input text file
    let input_file = match File::open("C:/Users/Yintong Luo/Desktop/eCTF/max78000_fall23/flash_dump/data/formatted_flash.txt") {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening input file: {}", err);
            return;
        }
    };

    // Create a reader to read lines from the input file
    let reader = BufReader::new(input_file);

    // Open the output binary file
    let mut output_file = match File::create("C:/Users/Yintong Luo/Desktop/eCTF/max78000_fall23/flash_dump/data/flash_512kb_formatted.bin") {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error creating output file: {}", err);
            return;
        }
    };

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
        let ascii_bytes = match hex_to_ascii(hex_string) {
            Some(bytes) => bytes,
            None => {
                eprintln!("Invalid hex string: {}", hex_string);
                continue;
            }
        };

        // Write ASCII bytes to the output file
        if let Err(err) = output_file.write_all(&ascii_bytes) {
            eprintln!("Error writing to output file: {}", err);
            return;
        }
    }

    println!("Conversion completed successfully.");
}