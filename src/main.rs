use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = match File::open("testfiles/geom.txt"){
        Ok(f) => f,
        Err(_) => todo!(),
    }; // Or just use a '?' instead of match
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
} 
