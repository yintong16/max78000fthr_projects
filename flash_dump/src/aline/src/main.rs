use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};



fn main() -> io::Result<()> {
    // Open the input file for reading
    let input_file = File::open("C:/Users/Yintong Luo/Desktop/eCTF/max78000_fall23/flash_dump/data/formatted_flash_2nd_half.txt")?;
    let input_file = BufReader::new(input_file);

    // Open the output file for writing
    let output_file = File::create("C:/Users/Yintong Luo/Desktop/eCTF/max78000_fall23/flash_dump/data/second_half_alined.txt")?;
    let mut output_file = BufWriter::new(output_file);
    let mut line_cnt = 0;
    for line_result in input_file.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                continue; // Skip to the next line in case of an error
            }
        };

        let trimmed_data = line.trim();

        if line_cnt == 0{
            write!(output_file, "{}", trimmed_data)?;
        } 
        else{
            let x1 = &trimmed_data[0..16];
            let x2 = &trimmed_data[16..32];
            write!(output_file, "{}", x1)?;
            writeln!(output_file)?;
            write!(output_file, "{}", x2)?;
        }
        line_cnt += 1;
    }

    println!("Conversion completed successfully.");

    Ok(())
}