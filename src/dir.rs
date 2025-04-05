use std::fs;
use std::io;
use std::env;
use std::path::PathBuf;

pub struct GetPath {
    pub config_dir: PathBuf,
    pub starship_path: PathBuf,
}

impl GetPath {
    pub fn dir() -> Self {
            let home_dir = env::var("HOME").expect("Error: HOME directory not set.");
            let config_dir = PathBuf::from(&home_dir).join(".config");
            let starship_path = config_dir.join("starship.toml");

            Self {
                config_dir,
                starship_path,
            }
    }
    
}

pub fn create_file() -> io::Result<()> {
    let path = GetPath::dir();
    fs::create_dir_all(&path.config_dir)?;
    Ok(())    
}

pub fn reset_file() -> io::Result<()> {
    let rest = GetPath::dir();
    if rest.starship_path.exists() {
        fs::remove_file(&rest.starship_path)?;
        println!("Theme file removed.");
    } else {
        println!("Theme file does not exist.");
    };
    Ok(())   
}

