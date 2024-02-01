

fn calculate_checksum(hex_number: &str) -> u8 {
    // Convert the hex string to a vector of bytes
    let bytes: Vec<u8> = hex::decode(hex_number).unwrap_or_else(|e| {
        eprintln!("Error decoding hex string: {}", e);
        std::process::exit(1);
    });

    // Calculate the checksum
    let sum: u8 = bytes.iter().fold(0, |acc, &byte| acc.wrapping_add(byte));
    let checksum = (0x100 - u16::from(sum)) as u8;
    checksum
}

fn main() {
    // Example 16-byte hex number
    let hex_number = "10004000790600107B0600107D0600107F060010";

    // Calculate and print the checksum
    let checksum = calculate_checksum(hex_number);
    println!("Checksum: {:02X}", checksum);
}
