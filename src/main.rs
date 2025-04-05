mod dir;
mod starship;

use dir::{create_file, reset_file, GetPath};
use std::{fs::File, io::{self, Write}};

fn main() -> io::Result<()> {
    print!("Select Theme (1or2/Default-1/Rest-r): ");
    io::stdout().flush().expect("Display output error");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Get input error");

    let input = input.trim();

    let theme = if input.is_empty() {
        println!("Default theme is set");
        starship::THEME1
    } 
    
    else if input.eq_ignore_ascii_case("r") {
        reset_file()?;
        return Ok(());
    } 
    
    else  {
        match input.parse::<i32>() {
            Ok(1) => starship::THEME1,
            Ok(2) => starship::THEME2,
            _ => {
                println!("Invalid Input");
                return Ok(());
            }
        }
    };
    create_file()?;
    let path = GetPath::dir();
    let mut star_toml = File::create(&path.starship_path)?;
    star_toml.write_all(theme.as_bytes())?;
    println!("Successfully create starship theme {:?}", path.starship_path);
    
    Ok(())
}