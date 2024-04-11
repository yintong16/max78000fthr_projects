use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};



fn main() -> std::io::Result<()>{
    // Open the input file for reading
    let input_file = File::open("C:/Users/Yintong Luo/Desktop/eCTF/max78000_fall23/flash_dump/data/formatted_512kb_formatted.txt")?;
    let input_file = BufReader::new(input_file);

    // Open the text section for writing
    let output_file = File::create("C:/Users/Yintong Luo/Desktop/eCTF/max78000_fall23/flash_dump/src/map_sections/src/text_section.txt")?;
    let mut output_file = BufWriter::new(output_file);

    // Open the data section for writing
    let output_file2 = File::create("C:/Users/Yintong Luo/Desktop/eCTF/max78000_fall23/flash_dump/src/map_sections/src/data_section.txt")?;
    let mut output_file2 = BufWriter::new(output_file2);

    let mut _stext = 0x10000000;
    let _etext = 0x10008dbc;

    for line_result in input_file.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                continue; // Skip to the next line in case of an error
            }
        };
        if _stext <= _etext {
            write!(output_file, "{}", line)?;
            _stext += 0x10;
        }
        else {
            write!(output_file2, "{}", line)?;
        }
    }

    Ok(())

}
