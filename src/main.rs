use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Atom {
    charge: usize,
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug)]
struct Molecule {
    atoms: Vec<Atom>,
}

/// Load a geometry from geom.txt and split it into appropriate vectors
fn load_geom() -> (String, Molecule) {
    let file = File::open("testfiles/geom.txt");
    let mut contents = String::new();
    file.expect("failed to open file")
        .read_to_string(&mut contents)
        .unwrap();
    let mut atoms = Vec::new();
    for line in contents.lines() {
        let line: Vec<_> = line.split_whitespace().collect();
        if line.len() > 1 {
            let charge = line[0].parse::<usize>().unwrap();
            let x = line[1].parse::<f64>().unwrap();
            let y = line[2].parse::<f64>().unwrap();
            let z = line[3].parse::<f64>().unwrap();
            let atom = Atom { charge, x, y, z };
            atoms.push(atom)
        }
    }
    (contents, Molecule { atoms })
}

impl Molecule {
    fn bond_length(&self) -> Vec<(usize, usize, f64)> {
        let mut ret = Vec::new();
        for (indexi, atomi) in self.atoms.iter().enumerate() {
            for (indexj, atomj) in self.atoms.iter().enumerate() {
                if indexi < indexj {
                    let xs = (atomi.x - atomj.x).powi(2);
                    let ys = (atomi.y - atomj.y).powi(2);
                    let zs = (atomi.z - atomj.z).powi(2);
                    let bl = (xs + ys + zs).sqrt();
                    ret.push((indexi, indexj, bl));
                    println!("{:5} {} {:5.2}", indexi, indexj, bl);
                }
            }
        }
        ret
    }
}

fn main() {
    let (contents, mol) = load_geom();
    mol.bond_length();
    println!("{}", contents);
}
