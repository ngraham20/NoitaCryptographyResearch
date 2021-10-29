use crate::errors::*;
use crate::genetics::gene::Gene;
use crate::genetics::chisquared::*;

use std::collections::HashMap;

// 1. Create a population with random genes
// 2. Figure out which n are the best
// 3. Remove some or all of the rest
// 4. Using the parents, create more children
//    The parents and children are now the population
// 5. Repeat until some condition is reached

/// Takes two genes and generates a population of size
pub fn createPopulation(size: usize, gene: &Gene) -> Result<Vec<Gene>> {
    let mut population: Vec<Gene> = vec![Gene::from(&gene.genomes); size];
    // println!("beforeshuffle: {:?}", population);
    for mut gene in population.iter_mut() {
        gene.shuffle();
    }

    // println!("aftershuffle: {:?}", population);

    Ok(population)
}

pub fn testPopulation(population: &mut Vec<Gene>, language: &crate::language::Language) -> Result<()> {
    for mut gene in population {
        testGene(&mut gene, &language);
    }
    Ok(())
}

/// Select parents from a population
/// 
/// for now, just pick the best two
// pub fn selectParents(population: &mut Vec<Gene>) -> Result<[usize; 2]> {
//     population.sort_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());
//     Ok(&population[0..2])
// }

pub fn incrementGeneration(population: &mut Vec<Gene>, count: usize) -> Result<()> {
    use rand::distributions::{Distribution, Uniform};

    // age the genes, and if one gets too old, kill it
    for (i, gene) in population.iter_mut().enumerate() {
        // println!("i: {}", i);
        gene.age += 1;
    }

    // remove the oldest two
    population.sort_by_key(|a| a.age);
    println!("oldest is: {}", population[population.len()-1].age);
    population.pop();
    population.pop();
    population.pop();
    population.pop();
    population.pop();
    population.pop();

    population.sort_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());
    
    // the parents are the two best remaining
    let parents: Vec<Gene> = population[0..2].to_vec();

    // produce enough children to fill out the population
    for i in (0..count-population.len()) {
        println!("producing. . .");
        if i % 2 == 0 {
            population.push(parents[0].produceWith(&parents[1])?);
        }
        else {
            population.push(parents[1].produceWith(&parents[0])?);
        }
    }
    
    Ok(())
}

pub fn mutate(gene: &mut Gene) -> Result<()> {
    // pick a random number
    // if it's within certain bounds
    //   swap two genomes
    Ok(())
}

/// Test multiple frequencies in English
/// http://pi.math.cornell.edu/~mec/2003-2004/cryptography/subs/digraphs.html
/// http://pi.math.cornell.edu/~mec/2003-2004/cryptography/subs/hints.html
/// http://pi.math.cornell.edu/~mec/2003-2004/cryptography/subs/frequencies.html
/// https://www3.nd.edu/~busiforc/handouts/cryptography/Letter%20Frequencies.html#trigrams
/// - Monographs: E and T are most common
/// - Digraphs: TH and HE are most common
/// - Trigraphs: THE, AND, ING, HER are the most common
///
/// Steps:
/// - Decode the message using the cipher and the gene
/// - Decode the result 25 times through substitution
/// - The best of these is the fitness for that gene
/// 
/// The idea here is that if knowing one wheel turns this problem into
/// a substitution cipher, then even with some of the letters wrong, we
/// may be able to see actual english come out of it
pub fn testGene(gene: &mut Gene, language: &crate::language::Language) -> Result<()> {
    let monochi = testMonograms(&gene.genomes, language.monograms()?)?;
    let dichi = testMultigram(&gene.genomes, language.digrams()?, 2)?;
    let trichi = testMultigram(&gene.genomes, language.trigrams()?, 3)?;
    let quadrichi = testMultigram(&gene.genomes, language.quadrigrams()?, 4)?;
    let doublechi = testMultigram(&gene.genomes, language.doubles()?, 2)?;
    // println!("Gene: {:?}", &gene.genomes);
    // println!("m: {}, d: {}, t: {}, q: {}, dd: {}", monochi, dichi, trichi, quadrichi, doublechi);
    // println!();
    gene.fitness = monochi + dichi + trichi + quadrichi + doublechi;
    // gene.fitness = 8f64 * monochi + 4f64*dichi + 2f64*trichi + quadrichi;
    // gene.fitness = monochi + 2f64 * dichi + 4f64 * trichi + 8f64 * quadrichi;
    Ok(())
}

pub fn testMultigram(message: &Vec<u32>, mut compdata: HashMap<Vec<u32>, f64>, size: usize) -> Result<f64> {
    let mut mesdata: HashMap<Vec<u32>, f64> = HashMap::new();
    let mlen = message.len()-size-1;
    for i in (0..mlen) {
        // mdi is any given digram (including overlaps)
        let mdi = message[i..i+size].to_vec();
        let count = mesdata.entry(mdi).or_insert(0.0);
        *count += 1.0;
    }

    for (_, v) in compdata.iter_mut() {
        *v = *v / 100f64 * mlen as f64
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