use serde_json;
use std::fs::File;
use std::io::{self, Write};

pub fn saving() -> io::Result<()> {
    let mut input_string = String::new();
    println!("Enter a number: ");
    io::stdin()
        .read_line(&mut input_string)
        .expect("couldnt read line");

    let value: i64 = input_string.trim().parse().expect("Enter a number:");

    let serialized = serde_json::to_string(&value).expect("Failed to save");
    let mut file = File::create("number.json")?;
    file.write_all(serialized.as_bytes())?;

    println!("mumber saved");

    Ok(())
}
