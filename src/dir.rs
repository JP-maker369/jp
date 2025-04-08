use std::fs;
use std::io;
use std::env;
use std::path::PathBuf;

pub struct GetPath {
    pub wezterm_path: PathBuf,
    pub starship_path: PathBuf,
    pub wezterm_dir: PathBuf,
}

impl GetPath {
    pub fn new() -> Self {
            let home_dir = env::var("HOME").expect("Error: HOME directory not set.");
            let config_dir = PathBuf::from(&home_dir).join(".config");
            let wezterm_dir = config_dir.join("wezterm");
            let starship_path = config_dir.join("starship.toml");
            let wezterm_path = wezterm_dir.join("wezterm.lua");

            Self {
                wezterm_dir,
                wezterm_path,
                starship_path,
            }
    }
    
}

pub fn create_file() -> io::Result<()> {
    let path = GetPath::new();
    fs::create_dir_all(&path.wezterm_dir)?;
    Ok(())    
}

pub fn reset_file() -> io::Result<()> {
    let path = GetPath::new();
    if path.starship_path.exists() {
        fs::remove_file(&path.starship_path)?;
    } 
    if path.wezterm_path.exists() {
        fs::remove_dir_all(&path.wezterm_dir)?;
        println!("Theme file is removed.");
    } 
    else {
        println!("Theme file does not exist.");
    };
    Ok(())   
}

