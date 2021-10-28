use crate::errors::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Language {
    monograms: HashMap<char, f64>,
    digrams: HashMap<String, f64>,
    trigrams: HashMap<String, f64>,
    quadrigrams: HashMap<String, f64>,
    doubles: HashMap<String, f64>,
}

impl Language {
    pub fn monograms(&self) -> Result<HashMap<u32, f64>> {
        Ok(self.monograms.iter().map(|(k, v)| (*k as u32, *v)).collect())
    }
    pub fn digrams(&self) -> Result<HashMap<Vec<u32>, f64>> {
        Ok(self.digrams.iter().map(|(k, v)| (k.chars().map(|x| x as u32).collect(), *v)).collect())
    }
    pub fn trigrams(&self) -> Result<HashMap<Vec<u32>, f64>> {
        Ok(self.trigrams.iter().map(|(k, v)| (k.chars().map(|x| x as u32).collect(), *v)).collect())
    }
    pub fn quadrigrams(&self) -> Result<HashMap<Vec<u32>, f64>> {
        Ok(self.quadrigrams.iter().map(|(k, v)| (k.chars().map(|x| x as u32).collect(), *v)).collect())
    }
    pub fn doubles(&self) -> Result<HashMap<Vec<u32>, f64>> {
        Ok(self.doubles.iter().map(|(k, v)| (k.chars().map(|x| x as u32).collect(), *v)).collect())
    }
}