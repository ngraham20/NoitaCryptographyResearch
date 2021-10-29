use crate::errors::*;

pub mod algorithm;
mod chisquared;
mod gene;
pub use gene::*;

pub trait Cipher {
    fn encode(&mut self, message: Vec<u32>) -> Result<Vec<u32>>;
    fn decode(&mut self, ciphertext: Vec<u32>) -> Result<Vec<u32>>;
}