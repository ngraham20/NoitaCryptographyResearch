use crate::errors::*;
use std::collections::HashMap;

pub fn testChiSquared<T: std::cmp::Eq + std::hash::Hash + std::fmt::Debug>(o: &HashMap<T, f64>, e: &HashMap<T, f64>) -> Result<f64> {

    let mut chisquare: Vec<f64> = Vec::new();

    for (eke, eva) in e {
        if let Some(ova) = o.get(eke) {
            // println!("{:?},{:?} | {:?},{:?}", eke, eva, eke, ova);
            chisquare.push((ova - eva).powi(2) / eva);
        }
        else {
            error_chain::bail!("Chisquared: Observed hashmap missing value from Expected");
        }
    }
    Ok(chisquare.iter().sum())
}