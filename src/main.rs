use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = match File::open("testfiles/geom.txt") {
        Ok(f) => f,
        Err(_) => todo!(),
    }; // Or just use a '?' instead of match
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut charge = Vec::new();
    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut z = Vec::new();
    for line in contents.lines() {
        let line: Vec<_> = line.split_whitespace().collect();
        if line.len() > 1 {
            charge.push(line[0].parse::<usize>().unwrap());
            x.push(line[1].parse::<f64>().unwrap());
            y.push(line[2].parse::<f64>().unwrap());
            z.push(line[3].parse::<f64>().unwrap());
        }
    }
    dbg!(charge, x, y, z);
    println!("{}", contents);
    Ok(())
}
