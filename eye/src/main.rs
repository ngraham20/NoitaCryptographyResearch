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

    let e: Vec<Vec<u8>> = "ofalltheplacestotravelmexicoisatthetopofmylist".chars().into_iter().map(|x| String::from(x).as_bytes().to_vec()).collect();
    // println!("{}", e.len());
    let echi = testMonogramsEnglish(&e)?;
    println!("echi: {}", echi);
    let lanfreqs: serde_json::Value = util::loadJson("data/english.json")?;
    let english: Language = serde_json::from_value(lanfreqs)?;
    // english.print_csv();

    // let lanfreqs: serde_json::Value = util::loadJson("data/test.json")?;
    // let english: Language = serde_json::from_value(lanfreqs)?;
    // use std::collections::HashMap;
    // let mut test: HashMap<Vec<u8>, &str> = HashMap::new();
    // // use byteorder::{ByteOrder, BigEndian, LittleEndian};
    // let r0 = "a".as_bytes();
    // let r1 = "ᚴ".as_bytes();
    // let r2 = "ᚵ".as_bytes();
    // let r3 = "ᛪ".as_bytes();
    // let r4 = "ᚠ".as_bytes();
    // test.insert(r0.to_vec(), "a");
    // test.insert(r1.to_vec(), "ᚴ");
    // test.insert(r4.to_vec(), "ᚠ");
    // println!("a: {:X?}", r0);
    // println!("ᚴ: {:X?}", r1);
    // println!("ᚵ: {:X?}", r2);
    // println!("ᛪ: {:X?}", r3);
    // println!("ᚠ: {:X?}", r4);
    // println!("hm: {:X?}", test);
    // println!("trial: {}", test[&vec![0xE1, 0x9A, 0xB4]]);
    // println!("ᚠ: {:X?}", english.datagrams()?.get(&vec![0xE1, 0x9A, 0xA0]).unwrap());

    // let b = b"Hello World!";

    // let mut hm: HashMap<[u8;2], u8> = HashMap::new();
    // hm.insert([1, 2], 3);

    // let b: [u8; 2] = [1, 2];
    // let mut c: Vec<u8> = vec![];

    // println!("{:?}", hm.get(&b));

    // // INFO: this actually works! You can use an array as a map value... huh


    // let mut ht: HashMap<Vec<u8>, u8> = HashMap::new();
    // ht.insert(vec![1, 2], 3);
    // c.push(1);
    // c.push(2);


    // println!("{:?}", ht.get(&c));

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
// use serde::{Serialize, Deserialize};
// use std::collections::HashMap;
// use serde_with::*;
// use serde_with::serde_as;

// #[serde_as]
// #[derive(Serialize, Deserialize)]
// struct Language {
//     #[serde_as(as = "HashMap<DisplayFromStr, DisplayFromStr>")]
//     monograms: HashMap<String, f64>,
// }

// #[serde_as]
// #[derive(Deserialize, Serialize)]
// struct ByteArray(
//     // Equivalent to serde_with::hex::Hex<serde_with::formats::Lowercase>
//     #[serde_as(as = "serde_with::hex::Hex")]
//     [u8; 12]
// );