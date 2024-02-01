use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() -> io::Result<()> {
    // Open the input file for reading
    let input_file = File::open("C:/Users/Yintong Luo/Desktop/eCTF/max78000_fall23/flash.txt")?;
    let input_file = BufReader::new(input_file);

    // Open the output file for writing
    let output_file = File::create("C:/Users/Yintong Luo/Desktop/eCTF/max78000_fall23/hex_flash2.txt")?;
    let mut output_file = BufWriter::new(output_file);

    // Read lines from the input file and process each line
    for line_result in input_file.lines() {
        let line = line_result?;
        let trimmed_line = line.trim();

        // Process each space-separated integer in the line
        for integer_str in trimmed_line.split_whitespace() {
            if let Ok(decimal_number) = integer_str.parse::<i32>() {
                // Write the hexadecimal representation to the output file
                

                write!(output_file, "{:X} ", decimal_number)?;
            } else {
                // Handle the case where parsing fails
                eprintln!("Error: Invalid integer in input file - {}", integer_str);
            }
        }

        // Move to the next line in the output file
        //eprintln!("counter");
        writeln!(output_file)?;
    }

    println!("Conversion completed successfully.");

    Ok(())
}