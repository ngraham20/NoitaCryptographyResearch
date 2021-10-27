use crate::errors::*;

mod substitution;
mod alberti;
pub use alberti::Alberti;
pub use substitution::Substitution;
pub use std::collections::VecDeque as Wheel;

pub trait Cipher {
    fn encode(&mut self, message: Vec<u32>) -> Result<Vec<u32>>;
    fn decode(&mut self, ciphertext: Vec<u32>) -> Result<Vec<u32>>;
}