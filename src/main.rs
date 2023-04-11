use ini::Ini;

fn main() -> Result<(), ()> {
    let conf = Ini::load_from_file("cv.ini")
        .map_err(|err| eprintln!("ERROR: Failed to load ini file: {err}"))?;
    for (sec, prop) in &conf {
        println!("Section: {:?}", sec);
        for (key, value) in prop.iter() {
            println!("  {:?}:{:?}", key, value);
        }
    }
    Ok(())
}
