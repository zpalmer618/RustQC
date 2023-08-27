use std::fs::File;
use std::io::prelude::*;

fn main() {
    let (contents, charge, x, y, z) = load_geom();
    dbg!(charge, x, y, z);
    println!("{}", contents);
}

/// Load a geometry from geom.txt and split it into appropriate vectors
fn load_geom() -> (String, Vec<usize>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let file = File::open("testfiles/geom.txt");
    let mut contents = String::new();
    file.expect("failed to open file").read_to_string(&mut contents).unwrap();
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
    (contents, charge, x, y, z)
}
