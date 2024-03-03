use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn main() -> io::Result<()> {
    // Open the input file for reading
    let hex_file = File::open("C:/Users/Yintong Luo/Desktop/eCTF/max78000_fall23/flash_dump/data/original_program_2nd_half.hex")?;//original_program

    let hex_reader = BufReader::new(hex_file);

    let txt_file = File::open("C:/Users/Yintong Luo/Desktop/eCTF/max78000_fall23/flash_dump/data/second_half_alined.txt")?;//formatted_flash
    let txt_reader = BufReader::new(txt_file);

    // Open the output file for writing
    let mut output_file = File::create("C:/Users/Yintong Luo/Desktop/eCTF/max78000_fall23/flash_dump/data/cmp_result.txt")?;
    //let mut counter = 0;

    for (hex_line, txt_line) in hex_reader.lines().zip(txt_reader.lines()) {
        let hex_line = hex_line?;
        let txt_line = txt_line?;
        // if counter == 0 {
        //     println!("{}, {}", hex_line, txt_line);
        // }
            
        // Strip the first 9 characters and the last 2 characters
        let stripped_hex_line = &hex_line[9..hex_line.len() - 2];

        // Compare the stripped lines
        if stripped_hex_line.trim() == txt_line.trim() {
            // Stripped lines are the same, write to the output file
            writeln!(output_file, "{}", stripped_hex_line)?;// should write hex_line maybe
        } else {
            // Stripped lines are different, write both values to the output file separated by a comma
            println!("Found different at address: {}", &hex_line[0..8]);
            writeln!(output_file, "{},{}", stripped_hex_line, txt_line)?;
        }
        //counter += 1;
    }

    println!("Conversion completed successfully.");

    Ok(())
}