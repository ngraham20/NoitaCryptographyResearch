#![recursion_limit = "1024"]
#![allow(warnings)]

mod errors;
use errors::*;
mod cipher;
mod prelude {
    pub use crate::cipher::{Alberti, Substitution, Wheel, Cipher};
}
mod gene;
use gene::*;
use prelude::*;

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }
        
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    // let mut g = gene::Gene::from("abcdefg");
    // let pop = gene::createPopulation(10, &g)?;
    // println!("Original Gene: {:?}", std::str::from_utf8(&g.genomes)?);
    // println!("Resulting Population: {:?}", pop.into_iter().map(|x| String::from_utf8(x.genomes)));
    let mut ptwheel = Wheel::from("eodlrwh".chars().map(|x| x as u32).collect::<Vec<u32>>());
    let mut ctwheel = Wheel::from("ᚠᚡᚢᚣᚤᚥᚦ".chars().map(|x| x as u32).collect::<Vec<u32>>());
    let mut acipher = Alberti::new(ptwheel.clone(), ctwheel.clone());
    let a = acipher.encode(&"hello world".chars().map(|x|x as u32).collect::<Vec<u32>>())?;
    println!("{:X?}", a.iter().map(|x| std::char::from_u32(*x).unwrap()).collect::<String>());
    let mut bcipher = Alberti::new(ptwheel.clone(), ctwheel.clone());
    let b = bcipher.decode(&"ᚦᚡᚥᚦᚥ ᚤᚡᚥᚥᚥ".chars().map(|x|x as u32).collect::<Vec<u32>>())?;
    println!("{:X?}", b.iter().map(|x| std::char::from_u32(*x).unwrap()).collect::<String>());
    
    Ok(())
}