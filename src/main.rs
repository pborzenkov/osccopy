use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();

    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => print!("\x1B]52;c;{}\x07", base64::encode(&buffer)),
        Err(err) => eprintln!("Error: {}", err),
    }
}
