pub fn print_bytes(data: &[u8]) {
    let mut index = 0;

    for byte in data {
        print!("{:02X} ", byte);

        index += 1;
        if index % 8 == 0 {
            println!();
        }
    }
}
