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
        let mut genomeb = distro.sample(&mut rng);
        if genomea == genomeb {
            genomeb = (genomeb + 1) % self.genomes.len();
        }
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
        // in this version, lack of duplicates, and the order matters most
        // let mut copymap: HashMap<u32, usize> = HashMap::new();
        // for letter in &self.genomes {
        //     let count = copymap.entry(*letter).or_insert(0);
        //     *count += 1;
        // }
        
        // // copy the middle chunk to the child
        // let mut child = Gene::new();
        // child.genomes = vec![0u32; self.genomes.len()];
        // for i in (crosspoints[0]..crosspoints[1]) {
        //     child.genomes[i] = self.genomes[i];
        // }
        

        // let mut i = crosspoints[1];
        // for genome in &other.genomes {
        //     if let Some(count) = copymap.get_mut(genome) {
        //         if *count > 0 {
        //             child.genomes[i % self.genomes.len()] = *genome;
        //             i += 1;
        //             *count -= 1;
        //         }
        //     }
        // }

        // in this version, position matters most, duplicates are fine
        let mut child = Gene::new();
        child.genomes = vec![0u32; self.genomes.len()];
        for i in (0..crosspoints[0]) {
            child.genomes[i] = self.genomes[i];
        }

        for i in (crosspoints[0]..crosspoints[1]) {
            child.genomes[i] = other.genomes[i];
        }

        for i in (crosspoints[1]..self.genomes.len()) {
            child.genomes[i] = self.genomes[i];
        }
        
        let mutdistro = Uniform::from(0..100);
        if mutdistro.sample(&mut rng) <= 10 {
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

#[cfg(test)]
mod test {
    use crate::genetics::gene::Gene;
    
    #[test]
    fn from_string() {
        let gene = Gene::from("gandalf");
        assert_eq!("gandalf".chars().map(|x| x as u32).collect::<Vec<u32>>(), gene.genomes)
    }
    #[test]
    fn genome_string() {
        let gene = Gene::from("islidur");
        assert_eq!("islidur", gene.genome_string().unwrap())
    }
    #[test]
    fn shuffle() {
        use std::collections::HashMap;
        let genea = Gene::from("frodo");
        let mut gac: HashMap<u32, usize> = HashMap::new();
        
        // count genomes in genea
        for genome in &genea.genomes {
            let count = gac.entry(*genome).or_insert(0);
            *count += 1;
        }
        
        let mut geneb = genea.clone();
        geneb.shuffle();
        let mut gbc: HashMap<u32, usize> = HashMap::new();

        // count genomes in geneb
        for genome in &geneb.genomes {
            let count = gbc.entry(*genome).or_insert(0);
            *count += 1;
        }

        // pass if the counts are the same, but the genomes are different
        assert_eq!(gac, gbc);
        assert_ne!(genea.genomes, geneb.genomes);
    }
    #[test]
    fn mutate() {
        use std::collections::HashMap;
        let genea = Gene::from("abcdefg");
        let mut gac: HashMap<u32, usize> = HashMap::new();
        
        // count genomes in genea
        for genome in &genea.genomes {
            let count = gac.entry(*genome).or_insert(0);
            *count += 1;
        }
        
        let mut geneb = genea.clone();
        geneb.mutate();
        let mut gbc: HashMap<u32, usize> = HashMap::new();

        // count genomes in geneb
        for genome in &geneb.genomes {
            let count = gbc.entry(*genome).or_insert(0);
            *count += 1;
        }

        let mut errors = 0;
        for i in (0..genea.genomes.len()) {
            if genea.genomes[i] != geneb.genomes[i] {
                errors += 1;
            }
        }
        // pass if the counts are the same and exactly two genomes are misplaced
        assert_eq!(gac, gbc);
        assert_eq!(2, errors);
    }
}