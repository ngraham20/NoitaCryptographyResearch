use crate::errors::*;

mod substitution;
mod alberti;
pub use alberti::Alberti;
pub use substitution::Substitution;
pub use std::collections::VecDeque as Wheel;

pub trait Cipher {
    fn encode(&mut self, message: &[u16]) -> Result<Vec<u16>>;
    fn decode(&mut self, ciphertext: &[u16]) -> Result<Vec<u16>>;
}