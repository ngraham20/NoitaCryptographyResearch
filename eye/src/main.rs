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
    // let mut g = gene::Gene::from("ᚠᚡᚢᚣᚤᚥᚦ");
    // let mut pop = gene::createPopulation(10, &g)?;
    // println!("Original Gene: {:?}", g.genome_string()?);

    // println!("Resulting Population: {:?}", pop.iter().map(|x| x.genome_string().unwrap()).collect::<Vec<String>>());
    // testPopulation(&mut pop);
    // println!("Resulting Population Fitness: {:?}", pop.iter().map(|x| x.fitness).collect::<Vec<f64>>());
    
    
    // let mut ptwheel = Wheel::from("eodlrwh".chars().map(|x| x as u16).collect::<Vec<u16>>());
    // let mut ctwheel = Wheel::from("ᚠᚡᚢᚣᚤᚥᚦ".chars().map(|x| x as u16).collect::<Vec<u16>>());
    // let mut acipher = Alberti::new(ptwheel.clone(), ctwheel.clone());
    // let a = acipher.encode(&"hello world".chars().map(|x|x as u16).collect::<Vec<u16>>())?;
    // println!("{:X?}", std::char::decode_utf16(a.clone())
    //     .map(|r| r.unwrap_or(std::char::REPLACEMENT_CHARACTER)).collect::<String>());
    // let mut bcipher = Alberti::new(ptwheel.clone(), ctwheel.clone());
    // let b = bcipher.decode(&"ᚦᚡᚥᚦᚥ ᚤᚡᚥᚥᚥ".chars().map(|x|x as u16).collect::<Vec<u16>>())?;
    // println!("{:X?}", std::char::decode_utf16(b.clone())
    //     .map(|r| r.unwrap_or(std::char::REPLACEMENT_CHARACTER)).collect::<String>());

    // let e = "ENALHEASHESAFGHTFROMTHEIRESHALLBEWOKADISHFROMTOWSSHALLSPRING".chars().map(|x| x as u16).collect::<Vec<u16>>();
    // println!("{}", e.len());
    // println!("{:?}", std::char::decode_utf16(e.clone())
    //     .map(|r| r.unwrap_or(std::char::REPLACEMENT_CHARACTER)).collect::<String>());
    // let echi = testMonogramsEnglish(&e)?;
    // println!("echi: {}", echi);
    // let b = b"Hello World!";

    let mut hm: HashMap<[u8;2], u8> = HashMap::new();
    hm.insert([1, 2], 3);

    let b: [u8; 2] = [1, 2];
    let c: Vec<u8> = vec![1, 2];

    println!("{:?}", hm.get(&b));

    // INFO: this actually works! You can use an array as a map value... huh


    let mut ht: HashMap<Vec<u8>, u8> = HashMap::new();
    ht.insert(vec![1, 2], 3);

    println!("{:?}", ht.get(&c));

    // INFO: holy crap, this works too...

    
    // use serde_json::json;
    // assert_eq!(
    //     json!("48656c6c6f20576f726c6421"),
    //     serde_json::to_value(ByteArray(b.clone())).unwrap()
    // );
    // let l: Language = serde_json::from_value(util::loadJson("data/test.json")?)?;
    // println!("{:?}", l.monograms);

    Ok(())
}
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_with::*;
use serde_with::serde_as;
use std::collections::BTreeMap;

#[serde_as]
#[derive(Serialize, Deserialize)]
struct Language {
    #[serde_as(as = "HashMap<DisplayFromStr, serde_with::hex::Hex>")]
    monograms: HashMap<String, Vec<u8>>,
}

#[serde_as]
#[derive(Deserialize, Serialize)]
struct ByteArray(
    // Equivalent to serde_with::hex::Hex<serde_with::formats::Lowercase>
    #[serde_as(as = "serde_with::hex::Hex")]
    [u8; 12]
);