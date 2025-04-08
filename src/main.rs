mod dir;
mod theme;

use dir::{create_file, reset_file, GetPath};
use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    print!("Select Theme (1or2/Default-1/Rest-r): ");
    io::stdout().flush().expect("Display output error for starship");

    let mut star_input = String::new();
    io::stdin().read_line(&mut star_input).expect("Get input error for starship");

    let star_input = star_input.trim();

    let star_theme = if star_input.is_empty() {
       // println!("Default theme is created");
        theme::STAR1
    } 
    
    else if star_input.eq_ignore_ascii_case("r") {
        reset_file()?;
        return Ok(());
    } 
    
    else {
        match star_input.parse::<i32>() {
            Ok(1) => {
                //println!("Successfully create starship theme 1");
                theme::STAR1
            }
            Ok(2) => {
                //println!("Successfully create starship theme 2");
                theme::STAR2
            }
            _ => {
                println!("Invalid Input");
                return Ok(());
            }
        }
    };

    print!("Select Theme: ");
    io::stdout().flush().expect("Display output error for wezterm");

    let mut wez_input = String::new();
    io::stdin().read_line(&mut wez_input).expect("Get input error for wezterm");

    let wez_input = wez_input.trim();

    let wez_theme = if wez_input.is_empty() {
        //println!("Default theme is created");
        theme::WEZ1
    } 
    
    else  {
        match wez_input.parse::<i32>() {
            Ok(1) => {
               // println!("Successfully create wezterm theme 1");
                theme::WEZ1
            }
            _ => {
                println!("Invalid Input");
                return Ok(());
            }
        }
    };
    create_file()?;
    let path = GetPath::new();
    let mut star_toml = File::create(&path.starship_path)?;
    star_toml.write_all(star_theme.as_bytes())?;

    let mut wez_lua = File::create(&path.wezterm_path)?;
    wez_lua.write_all(wez_theme.as_bytes())?;
    println!("Successfully create themes {:?} {:?}", path.starship_path, path.wezterm_path);
    
    Ok(())
}