use std::path::Path;
use std::fs;
use std::io::{Error, ErrorKind};

pub fn load_rom(rom_name: &str) -> Result<Vec<u8>, Error> {
    let filepath = format!("./roms/{}.ch8", rom_name);
    let path = Path::new(filepath.as_str());

    if !path.is_file() {
        return Err(Error::new(ErrorKind::NotFound, format!("Unable to found rom: {rom_name}")));    
    }

    fs::read(path)
}