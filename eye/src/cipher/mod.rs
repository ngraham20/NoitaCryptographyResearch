use crate::errors::*;

mod substitution;
mod alberti;
pub use alberti::Alberti;
pub use substitution::Substitution;
pub use std::collections::VecDeque as Wheel;

pub trait Cipher {
    fn encode(&mut self, message: Vec<Vec<u8>>) -> Result<Vec<Vec<u8>>>;
    fn decode(&mut self, ciphertext: Vec<Vec<u8>>) -> Result<Vec<Vec<u8>>>;
}