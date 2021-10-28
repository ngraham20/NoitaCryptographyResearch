#![recursion_limit = "1024"]
#![allow(warnings)]

mod errors;
use errors::*;
mod cipher;
mod language;
use language::Language;
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
    let engdata: serde_json::Value = util::loadJson("data/english.json")?;
    let english: Language = serde_json::from_value(engdata)?;

    let mut g = gene::Gene::from("allthatisgolddoesnotglitternotallthosewhowanderarelosttheoldthatisstrongdoesnotwitherdeeprootsarenotreachedbythefrostfromtheashesafireshallbewokenalightfromtheshadowsshallspringrenewedshallbebladethatwasbrokenthecrownlessagainshallbeking");
    let mut pop = gene::createPopulation(10, &g)?;
    println!("Original Gene: {:?}", g.genome_string()?);
    testGene(&mut g, &english);
    println!("Orginal Fitness: {}", g.fitness);
    // println!("Resulting Population: {:?}", pop.iter().map(|x| x.genome_string().unwrap()).collect::<Vec<String>>());
    testPopulation(&mut pop, &english);
    println!("Resulting Population Fitness: {:?}", pop.iter().map(|x| x.fitness).collect::<Vec<f64>>());
    
    // let mut ptwheel = Wheel::from("eodlrwh".chars().map(|x| x as u32).collect::<Vec<u32>>());
    // let mut ctwheel = Wheel::from("ᚠᚡᚢᚣᚤᚥᚦ".chars().map(|x| x as u32).collect::<Vec<u32>>());
    // let mut acipher = Alberti::new(ptwheel.clone(), ctwheel.clone());
    // let a = acipher.encode("hello world".chars().map(|x| x as u32).collect::<Vec<u32>>())?;
    // println!("{:?}", a.iter().map(|x| std::char::from_u32(*x).unwrap()).collect::<String>());
    // let mut bcipher = Alberti::new(ptwheel.clone(), ctwheel.clone());
    // let b = bcipher.decode("ᚦᚡᚥᚦᚥ ᚤᚡᚥᚥᚥ".chars().map(|x| x as u32).collect::<Vec<u32>>())?;
    // println!("{:?}", b.iter().map(|x| std::char::from_u32(*x).unwrap()).collect::<String>());

    // let mut e: Vec<u32> = "allthatisgolddoesnotglitternotallthosewhowanderarelosttheoldthatisstrongdoesnotwitherdeeprootsarenotreachedbythefrostfromtheashesafireshallbewokenalightfromtheshadowsshallspringrenewedshallbebladethatwasbrokenthecrownlessagainshallbeking".chars().into_iter().map(|x| x as u32).collect();
    // use rand::thread_rng;
    // use rand::seq::SliceRandom;
    // e.shuffle(&mut thread_rng());
    // println!("{:?}", b.iter().map(|x| std::char::from_u32(*x).unwrap()).collect::<String>());
    // let engdata: serde_json::Value = util::loadJson("data/english.json")?;
    // let english: Language = serde_json::from_value(engdata)?;
    // println!();
    // let monochi = testMonograms(&b, english.monograms()?)?;
    // println!();
    // let dichi = testMultigram(&b, english.digrams()?, 2)?;
    // println!();
    // let trichi = testMultigram(&b, english.trigrams()?, 3)?;
    // println!();
    // let quadrichi = testMultigram(&b, english.quadrigrams()?, 4)?;
    // println!();

    // println!("monochi: {}", monochi);
    // println!("dichi: {}", dichi);
    // println!("trichi: {}", trichi);
    // println!("quadrichi: {}", quadrichi);
    // println!("total: {}", 4f64 * monochi + 3f64 * dichi + 2f64 * trichi + quadrichi);
    Ok(())
}