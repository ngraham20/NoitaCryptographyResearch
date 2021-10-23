use crate::errors::*;

#[derive(Clone, Debug)]
pub struct Gene {
    pub genomes: Vec<u8>,
    pub age: u64
}

impl Gene {
    pub fn new() -> Gene {
        Gene {
            genomes: Vec::new(),
            age: 0
        }
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

// 1. Create a population with random genes
// 2. Figure out which n are the best
// 3. Remove some or all of the rest
// 4. Using the parents, create more children
//    The parents and children are now the population
// 5. Repeat until some condition is reached

pub fn createPopulation(size: usize, genome: Gene) -> Result<Vec<Gene>> {
    let population: Vec<Gene> = vec![genome.clone(); size]
        .into_iter()
        .map(|mut x| {x.shuffle(); x})
        .collect();

    Ok(population)
}