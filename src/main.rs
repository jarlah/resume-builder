pub mod parser;
use std::fs;

use crate::parser::CVData;

pub struct CVParser;
fn read_file(file: &str) -> Result<String, ()> {
    fs::read_to_string(file).map_err(|err| eprintln!("ERROR: Failed to read file: {err}"))
}

fn main() -> Result<(), ()> {
    let mut cv_data: CVData = Default::default();
    let unparsed_file = read_file("cv.ini")?;
    let (about, _) = parser::parse_about(&unparsed_file);
    cv_data.about = about;
    println!("{:#?}", cv_data);
    Ok(())
}
