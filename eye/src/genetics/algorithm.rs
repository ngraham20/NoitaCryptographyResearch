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
    for mut gene in population.iter_mut() {
        gene.shuffle();
    }
    Ok(population)
}

pub fn testPopulation(population: &mut Vec<Gene>, language: &crate::language::Language) -> Result<()> {
    for mut gene in population {
        testGene(&mut gene, &language);
    }
    Ok(())
}

pub fn incrementGeneration(population: &mut Vec<Gene>, count: usize) -> Result<()> {
    use rand::distributions::{Distribution, Uniform};

    // age the genes, and if one gets too old, kill it
    for gene in population.iter_mut() {
        // println!("i: {}", i);
        gene.age += 1;
    }

    // remove those who are too old
    // let t = population.clone();
    population.retain(|gene| gene.age < 20 );

    // remove the oldest two
    // population.sort_by_key(|a| a.age);
    // println!("oldest is: {}", population[population.len()-1].age);
    population.sort_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());
    // keep the top 20%
    let mut newpop = population[0..(population.len() / 5)].to_vec();

    // population.sort_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());
    
    // the parents are the two best remaining
    let parents: Vec<Gene> = newpop[0..2].to_vec();
    println!("parents fitness: {}, {}", parents[0].fitness, parents[1].fitness);

    // produce enough children to fill out the population
    for i in (0..count-newpop.len()) {
        // println!("producing. . .");
        if i % 2 == 0 {
            newpop.push(parents[0].produceWith(&parents[1])?);
        }
        else {
            newpop.push(parents[1].produceWith(&parents[0])?);
        }
    }

    *population = newpop;
    
    Ok(())
}

// TODO: change the test from the whole population to just one gene, the looping can just happen in here
pub fn run<T>(origin: Gene,
    popsize: usize,
    test: fn(&mut Vec<Gene>, &T) -> Result<()>,
    against: &T,
    fitnesscondition: fn(f64) -> bool) -> Result<()> {

    let mut pop = createPopulation(popsize, &origin)?;
    let mut i = 0;
    while true {
        println!("generation: {}", i);
        incrementGeneration(&mut pop, popsize);
        test(&mut pop, &against);
        i += 1;
        if fitnesscondition(pop[0].fitness) {
            break;
        }
    }
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
/// 
/// 
/// The reason that glyphs end up in the plaintext is because doubles
/// some glyphs from actually translating
pub fn testGene(gene: &mut Gene, language: &crate::language::Language) -> Result<()> {
    pub use crate::cipher::{Alberti, Substitution, Wheel, Cipher};
    let mut ptwheel = Wheel::from("oyqpucbdsgmevgfbjauwtdpalbtmxkizezlsiffzxrrvakhwnoqvhaxnicnyobyqjmpewsdhctdrcgkejul".chars().map(|x| x as u32).collect::<Vec<u32>>());
    let mut ctwheel = Wheel::from(gene.genomes.clone());
    // println!("ctwheel: {:?}", ctwheel.iter().map(|x| std::char::from_u32(*x).unwrap()).collect::<String>());
    let mut cipher = Alberti::new(ptwheel.clone(), ctwheel.clone());
    let plaintext = cipher.decode("ᛖᛣᛈᛮᚴᛥᛈᛇᛎᚧᚶᛗᚪᚨᚬᛈᛟᛸᚧᚢᛐᚵᛵᛠᛯᛴᚹᛨᛑᚼᚺᛪᛂᚣᛘᛗᛯᛚᛜᚿᛋᛔᛢᛞᛁᛂᛳᚡᚻᛔᚾᛁᛔᛍᚾᚧᚹᛜᚳᛱᛘᛟᚥᛒᚸᛍᚾᛳᛈᛝᛗᛘᚥᛍᚿᛒᛙᚯᚱᚱᛏᚧᚵᚸᛛᛆᚬᚱᛞᚺᛄᛞᛐᚧᛑᚣᛟᛀᚱᛴᚦᛐᛴᛣᚹᛩᚭᚭᛈᚰᛍᚢᛠᛶᚫᛋᚣᛚᛲᛴᚺᚦᚿᛁᚦᚴᚻᛪᚣᛷᚮᛃᛄᚮᛢᚱᛅᛃᚥᛀᚩᚯᛢᚻᚽᛏᛘᛉᛦᛨᚭᛉᛒᛑᛶᚳᚶᛇᛡᛕᛋᛦᛡᛘᛖᛄᛞᚺᚪᛏᛑᚠᛆᛚᛰᛜᚪᛵᛮᚮᛣᛓᛏᛟᛈᛸᛇᚵᚰᛐᛗᚫᚴᛯᛗᛋᚴᚯᚼᚣᚣᛵᛯᛠᚥᚢᛝᛵᛄᛷᛄᛂᛁᛉᛚᚩᚱᛨᛷᚮᛀᚽᛸᚾᛉᛟᚩᛟᚥᛳᛉᛶᚯᚠᚱᛓᚥ".chars().map(|x| x as u32).collect::<Vec<u32>>())?;
    
    let monochi = testMonograms(&plaintext, language.monograms()?)?;
    let dichi = testMultigram(&plaintext, language.digrams()?, 2)?;
    let trichi = testMultigram(&plaintext, language.trigrams()?, 3)?;
    let quadrichi = testMultigram(&plaintext, language.quadrigrams()?, 4)?;
    let doublechi = testMultigram(&plaintext, language.doubles()?, 2)?;
    gene.fitness = monochi + dichi + trichi + quadrichi + doublechi;
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