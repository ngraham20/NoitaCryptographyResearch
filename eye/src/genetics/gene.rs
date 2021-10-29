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

    /// Produce a single child with one other gene
    pub fn produceWith(&self, other: &Gene) -> Result<Gene> {
        use rand::distributions::{Distribution, Uniform};
        let mut rng = rand::thread_rng();
        let distro = Uniform::from(0..self.genomes.len());
        let mut crosspoints = [distro.sample(&mut rng), distro.sample(&mut rng)].to_vec();
        crosspoints.sort();

        // OPTIMIZABLE: count letter occurances in genome. Could be optimized probably by doing this once much earlier on.
        let mut copymap: HashMap<u32, usize> = HashMap::new();
        for letter in &self.genomes {
            let count = copymap.entry(*letter).or_insert(0);
            *count += 1;
        }
        
        // copy the middle chunk to the child
        let mut child = Gene::new();
        child.genomes = vec![0u32; self.genomes.len()];
        for i in (crosspoints[0]..crosspoints[1]) {
            child.genomes[i] = self.genomes[i];
        }
        

        let mut i = crosspoints[1];
        for genome in &other.genomes {
            if let Some(count) = copymap.get_mut(genome) {
                if *count > 0 {
                    child.genomes[i % self.genomes.len()] = *genome;
                    i += 1;
                    *count -= 1;
                }
            }
        }
        
        let mutdistro = Uniform::from(0..100);
        if mutdistro.sample(&mut rng) < 20 {
            child.mutate();
        }

        Ok(child)
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