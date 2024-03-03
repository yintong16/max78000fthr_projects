use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn calculate_checksum(hex_number: &str) -> u8 {
    // Convert the hex string to a vector of bytes
    let bytes: Vec<u8> = hex::decode(hex_number).unwrap_or_else(|e| {
        println!("{}", hex_number);
        eprintln!("Error decoding hex string: {}", e);
        std::process::exit(1);
    });

    // Calculate the checksum
    let sum: u8 = bytes.iter().fold(0, |acc, &byte| acc.wrapping_add(byte));
    let checksum = (0x100 - u16::from(sum)) as u8;
    checksum
}


fn main() -> io::Result<()> {
    // Open the input file for reading
    let input_file = File::open("C:/Users/Yintong Luo/Desktop/eCTF/max78000_fall23/flash_dump/data/formatted_flash.txt")?;
    let input_file = BufReader::new(input_file);

    // Open the output file for writing
    let output_file = File::create("C:/Users/Yintong Luo/Desktop/eCTF/max78000_fall23/flash_dump/data/flash512kb.hex")?;
    let mut output_file = BufWriter::new(output_file);

    let mut addr: u32 = 0;
    let record_type = "00";
    for line_result in input_file.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                continue; // Skip to the next line in case of an error
            }
        };

        let trimmed_data = line.trim();
        let byte_cnt = "10";

        let addr_str = format!("{:X}", addr);
        let mut padded_addr_str = addr_str.to_owned();
        while padded_addr_str.len() < 4 {
            padded_addr_str.insert(0, '0');
        }

        let output_str = format!("{}{}{}{}", byte_cnt, padded_addr_str, record_type, trimmed_data);
        let check_sum = calculate_checksum(&output_str);
        write!(output_file, ":{}{:02X}\n",output_str, check_sum)?;
        addr = addr.wrapping_add(16);
    }

    println!("Conversion completed successfully.");

    Ok(())
}