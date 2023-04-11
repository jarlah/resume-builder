use shellfish::{app, Command, Shell};
use std::error::Error;
use async_std::prelude::*;
 
extern crate async_std;

#[macro_use]
extern crate shellfish;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define a shell
    let mut shell = Shell::new_async(0_u64, "<[Resume-CLI]>-$ ");

    shell.commands.insert(
        "cat",
        Command::new_async(
            "Displays a file.".to_string(),
            async_fn!(u64, cat)
        ),
    );

    // Check if we have > 2 args, if so no need for interactive shell
    let mut args = std::env::args();
    if args.nth(1).is_some() {
        // Create the app from the shell.
        let mut app: app::App<u64, _> =
            app::App::try_from_async(shell)?;

        // Set the binary name
        app.handler.proj_name = Some("shellfish-example".to_string());
        app.load_cache()?;

        // Run it
        app.run_args_async().await?;
    } else {
        // Run the shell
        shell.run_async().await?;
    }
    Ok(())
}

/// Asynchronously reads a file
async fn cat(_state: &mut u64, args: Vec<String>) -> Result<(), Box<dyn Error>> {
    use async_std::fs;

    if let Some(file) = args.get(1) {
        let mut contents = String::new();
        let mut file = fs::File::open(file).await?;
        file.read_to_string(&mut contents).await?;
        println!("{}", contents);
    }
    
    Ok(())
}

// fn main1() -> Result<(), ()> {
//     let conf = Ini::load_from_file("cv.ini")
//         .map_err(|err| eprintln!("ERROR: Failed to load ini file: {err}"))?;
//     for (sec, prop) in &conf {
//         println!("Section: {:?}", sec);
//         for (key, value) in prop.iter() {
//             println!("  {:?}:{:?}", key, value);
//         }
//     }
//     Ok(())
// }
