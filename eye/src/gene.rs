use crate::errors::*;
use crate::util;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Gene {
    pub genomes: Vec<u32>,
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
        Ok(self.genomes.clone().iter().map(|x| std::char::from_u32(*x).unwrap()).collect())
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
            genomes: item.chars().map(|x| x as u32).collect(),
            age: 0,
            fitness: 0.0
        }
    }
}

impl From<&Vec<u32>> for Gene {
    fn from(item: &Vec<u32>) -> Self {
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

fn testChiSquared<T: std::cmp::Eq + std::hash::Hash + std::fmt::Debug>(o: &HashMap<T, f64>, e: &HashMap<T, f64>) -> Result<f64> {

    let mut chisquare: Vec<f64> = Vec::new();

    for (eke, eva) in e {
        if let Some(ova) = o.get(eke) {
            println!("{:?},{:?} | {:?},{:?}", eke, eva, eke, ova);
            chisquare.push((ova - eva).powi(2) / eva);
        }
        else {
            error_chain::bail!("Chisquared: Observed hashmap missing value from Expected");
        }
    }
    Ok(chisquare.iter().sum())
}

pub fn testMultigram(message: &Vec<u32>, mut compdata: HashMap<Vec<u32>, f64>, size: usize) -> Result<f64> {
    let mut mesdata: HashMap<Vec<u32>, f64> = HashMap::new();
    for i in (0..message.len()-size-1) {
        // mdi is any given digram (including overlaps)
        let mdi = message[i..i+size].to_vec();
        let count = mesdata.entry(mdi).or_insert(0.0);
        *count += 1.0;
    }

    for (_, v) in compdata.iter_mut() {
        *v = *v / 100f64 * message.len() as f64
    }

    // fill the message data with any missing common datagrams
    for (letter, v) in compdata.iter() {
        mesdata.entry(letter.clone()).or_insert(0.0);
    }
    Ok(testChiSquared(&mesdata, &compdata)?)
}

pub fn testMonograms(message: &Vec<u32>, mut compdata: HashMap<u32, f64>) -> Result<f64> {
    let mut mesdata: HashMap<u32, f64> = HashMap::new();
    for letter in message {
        let count = mesdata.entry(letter.clone()).or_insert(0.0);
        *count += 1.0;
    }
    for (_, v) in compdata.iter_mut() {
        *v = *v / 100f64 * message.len() as f64
    }

    // make sure the rest of the alphabet is here
    for (letter, v) in compdata.iter() {
        mesdata.entry(letter.clone()).or_insert(0.0);
    }
    Ok(testChiSquared(&mesdata, &compdata)?)
}

pub fn testPopulation(population: &mut Vec<Gene>) -> Result<()> {

    for mut gene in population {
        testGene(&mut gene);
    }
    Ok(())
}
