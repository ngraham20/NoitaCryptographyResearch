#![recursion_limit = "1024"]
#![allow(warnings)]

mod errors;
use errors::*;
mod cipher;
mod prelude {
    pub use crate::cipher::{Alberti, Substitution, Wheel, Cipher};
}
mod util;
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
    let mut g = gene::Gene::from("ᚠᚡᚢᚣᚤᚥᚦ");
    let mut pop = gene::createPopulation(10, &g)?;
    println!("Original Gene: {:?}", g.genome_string()?);

    println!("Resulting Population: {:?}", pop.iter().map(|x| x.genome_string().unwrap()).collect::<Vec<String>>());
    testPopulation(&mut pop);
    println!("Resulting Population Fitness: {:?}", pop.iter().map(|x| x.fitness).collect::<Vec<f64>>());
    
    
    // TODO: make this stuff work with generics so I can toy with it more than I am. This is a nightmare to modify haha
    let mut ptwheel = Wheel::from("eodlrwh".chars().map(|x| x as u32).collect::<Vec<u32>>());
    let mut ctwheel = Wheel::from("ᚠᚡᚢᚣᚤᚥᚦ".chars().map(|x| x as u32).collect::<Vec<u32>>());
    let mut acipher = Alberti::new(ptwheel.clone(), ctwheel.clone());
    let a = acipher.encode("hello world".chars().map(|x| x as u32).collect::<Vec<u32>>())?;
    println!("{:X?}", a.iter().map(|x| std::char::from_u32(*x).unwrap()).collect::<String>());
    let mut bcipher = Alberti::new(ptwheel.clone(), ctwheel.clone());
    let b = bcipher.decode("ᚦᚡᚥᚦᚥ ᚤᚡᚥᚥᚥ".chars().map(|x| x as u32).collect::<Vec<u32>>())?;
    println!("{:X?}", b.iter().map(|x| std::char::from_u32(*x).unwrap()).collect::<String>());

    let e: Vec<u32> = "ofalltheplacestotravelmexicoisatthetopofmylist".chars().into_iter().map(|x| x as u32).collect();
    let echi = testMonogramsEnglish(&e)?;
    println!("echi: {}", echi);
    let lanfreqs: serde_json::Value = util::loadJson("data/english.json")?;
    let english: Language = serde_json::from_value(lanfreqs)?;
    Ok(())
}