use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn hex_string_to_char(hex_str: &str) -> char {
    // Convert hexadecimal string to u8
    let hex_value = u8::from_str_radix(hex_str, 16).unwrap_or_else(|e| {
        eprintln!("Error converting hex string {} to u8: {}", hex_str, e);
        std::process::exit(1);
    });

    // Convert u8 to char
    let ascii_char = hex_value as char;

    ascii_char
}

fn main() -> io::Result<()> {
    // Open the input file for reading
    let input_file = File::open("C:/Users/Yintong Luo/Desktop/eCTF/max78000_fall23/flash_dump/data/flash512kb.txt")?;
    let input_file = BufReader::new(input_file);

    // Open the output file for writing
    let output_file = File::create("C:/Users/Yintong Luo/Desktop/eCTF/max78000_fall23/flash_dump/data/formatted_flash.txt")?;
    let mut output_file = BufWriter::new(output_file);

    let mut value_count = 0; // Counter to count the number of values written

    for line_result in input_file.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                continue; // Skip to the next line in case of an error
            }
        };

        let trimmed_line = line.trim();

        for hex_str in trimmed_line.split_whitespace() {
            // Add zeros to the front of the string if the length is less than 8
            let mut padded_hex_str = hex_str.to_owned();
            while padded_hex_str.len() < 8 {
                padded_hex_str.insert(0, '0');
            }

            // Write to file in format "XX XX XX XX "
            let x1 = &padded_hex_str[0..2];
            let x2 = &padded_hex_str[2..4];
            let x3 = &padded_hex_str[4..6];
            let x4 = &padded_hex_str[6..8];

            // let char_x1 = hex_string_to_char(x1);
            // let char_x2 = hex_string_to_char(x2);
            // let char_x3 = hex_string_to_char(x3);
            // let char_x4 = hex_string_to_char(x4);

            // write!(output_file, "{}{}{}{}", char_x4, char_x3, char_x2, char_x1)?;
            write!(output_file, "{}{}{}{}", x4, x3, x2, x1)?;

            // Increment the value count
            value_count += 1;

            //Check if 8 values have been written and add a new line
            if value_count == 4 {
                writeln!(output_file)?;
                value_count = 0; // Reset the count after adding a new line
            }
        }
    }

    println!("Conversion completed successfully.");

    Ok(())
}