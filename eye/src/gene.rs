use crate::errors::*;
use std::fmt;

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
        .map(|r| r.unwrap_or(std::char::REPLACEMENT_CHARACTER)).collect::<String>())
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
            genomes: item.chars().map(|x| x as u16).collect::<Vec<u16>>(),
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
    let population: Vec<Gene> = vec![Gene {genomes: gene.genomes.clone(), ..*gene}; size]
        .into_iter()
        .map(|mut x| {x.shuffle(); x})
        .collect();

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
pub fn testPopulation(population: &Vec<Gene>) -> Result<()> {

    Ok(())
}