use crate::errors::*;
use crate::util;
use std::fmt;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_with::serde_as;

#[derive(Clone, Debug)]
pub struct Gene {
    pub genomes: Vec<u16>,
    pub age: u64,
    pub fitness: f64
}

impl Gene {
    pub fn new() -> Self {
        Gene {
            genomes: Vec::new(),
            age: 0,
            fitness: 0.0
        }
    }

    pub fn genome_string(&self) -> Result<String> {
        Ok(std::char::decode_utf16(self.genomes.clone())
        .map(|r| r.unwrap_or(std::char::REPLACEMENT_CHARACTER)).collect())
    }
    /// Takes a random two genomes and swaps them
    pub fn mutate(&mut self) -> Result<()> {
        use rand::distributions::{Distribution, Uniform};
        let mut rng = rand::thread_rng();
        let distro = Uniform::from(0..self.genomes.len());
        let genomea = distro.sample(&mut rng);
        let genomeb = distro.sample(&mut rng);
        self.genomes.swap(genomea, genomeb);
        Ok(())
    }

    /// Shuffles the genomes around in place
    pub fn shuffle(&mut self) {
        use rand::thread_rng;
        use rand::seq::SliceRandom;
        self.genomes.shuffle(& mut thread_rng());
    }
}

impl From<&str> for Gene {
    fn from(item: &str) -> Self {
        Gene {
            genomes: item.chars().map(|x| x as u16).collect(),
            age: 0,
            fitness: 0.0
        }
    }
}

impl From<&Vec<u16>> for Gene {
    fn from(item: &Vec<u16>) -> Self {
        Gene {
            genomes: item.clone(),
            age: 0,
            fitness: 0.0
        }
    }
}

// 1. Create a population with random genes
// 2. Figure out which n are the best
// 3. Remove some or all of the rest
// 4. Using the parents, create more children
//    The parents and children are now the population
// 5. Repeat until some condition is reached

/// Takes two genes and generates a population of size
pub fn createPopulation(size: usize, gene: &Gene) -> Result<Vec<Gene>> {
    let mut population: Vec<Gene> = vec![Gene::from(&gene.genomes); size];
    for mut gene in population.iter_mut() {
        gene.shuffle();
    }
    Ok(population)
}

/// Test multiple frequencies in English
/// http://pi.math.cornell.edu/~mec/2003-2004/cryptography/subs/digraphs.html
/// http://pi.math.cornell.edu/~mec/2003-2004/cryptography/subs/hints.html
/// http://pi.math.cornell.edu/~mec/2003-2004/cryptography/subs/frequencies.html
/// https://www3.nd.edu/~busiforc/handouts/cryptography/Letter%20Frequencies.html#trigrams
/// - Monographs: E and T are most common
/// - Digraphs: TH and HE are most common
/// - Trigraphs: THE, AND, ING, HER are the most common
/// - Doubles: Commonly SS, LL, OO, EE, NN, PP. Seldom AA, YY, or UU
/// 
/// Steps:
/// - Decode the message using the cipher and the gene
/// - Decode the result 25 times through substitution
/// - The best of these is the fitness for that gene
/// 
/// The idea here is that if knowing one wheel turns this problem into
/// a substitution cipher, then even with some of the letters wrong, we
/// may be able to see actual english come out of it
pub fn testGene(gene: &mut Gene) -> Result<()> {

    gene.fitness = 10.0;
    Ok(())
}

fn testChiSquared(o: &HashMap<u16, f64>, e: &HashMap<u16, f64>) -> Result<f64> {

    let mut chisquare: Vec<f64> = Vec::new();

    for (eke, eva) in e {
        if let  Some(ova) = o.get(eke) {
            chisquare.push((ova - eva).powi(2) / eva);
        }
        else {
            error_chain::bail!("Chisquared: Observed hashmap missing value from Expected");
        }
    }
    Ok(chisquare.iter().sum())
}

pub fn testDigramsEnglish(message: &Vec<u16>) -> Result<f64> {

    Ok(1f64)
}

#[serde_as]
#[derive(Serialize, Deserialize)]
struct Language {
    #[serde_as(as = "HashMap<serde_with::hex::Hex, f64")]
    monograms: HashMap<u16, f64>,
}

// impl Language {
//     fn monoDatagrams(&self) -> Result<HashMap<u16, f64>> {
//         Ok(self.monograms.clone().into_iter().map(|(x, y)| (x as u16, y)).collect())
//     }
// }

// pub fn testMonogramsEnglish(message: &Vec<u16>) -> Result<f64> {
//     use serde_json::Value;
//     let lanfreqs: Value = util::loadJson("data/test.json")?;
//     let english: Language = serde_json::from_value(lanfreqs)?;
//     // let eletters = ['e',  't',  'a',  'o',  'i',  'n',  's',  'r',  'h',  'd',  'l',  'u', 'c', 'm', 'f', 'y', 'w', 'g', 'p', 'b', 'v', 'k', 'x', 'q', 'j', 'z'];
//     // let efreq =    [12.02, 9.10, 8.12, 7.68, 7.31, 6.95, 6.28, 6.02, 5.92, 4.32, 3.98, 2.88, 2.71, 2.61, 2.30, 2.11, 2.09, 2.03, 1.82, 1.49, 1.11, 0.69, 0.17, 0.11, 0.10, 0.07];
//     // let english: HashMap<u16, f64> = (0..eletters.len()).map(|i| (eletters[i] as u16, efreq[i])).collect();
//     let mut mletters: HashMap<u16, f64> = HashMap::new();
//     for letter in message {
//         let count = mletters.entry(*letter).or_insert(0.0);
//         *count += 1.0;
//     }
//     for (_, v) in mletters.iter_mut() {
//         *v = *v * 100f64 / message.len() as f64
//     }
//     // make sure the rest of the alphabet is here
//     for (letter, _) in &english.monograms {
//         mletters.entry(*letter as u16).or_insert(0.0);
//     }
//     Ok(testChiSquared(&mletters, &english.monograms)?)
// }

pub fn testPopulation(population: &mut Vec<Gene>) -> Result<()> {

    for mut gene in population {
        testGene(&mut gene);
    }
    Ok(())
}
