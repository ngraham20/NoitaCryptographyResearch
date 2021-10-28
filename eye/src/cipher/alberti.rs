use crate::errors::*;
use crate::cipher::{Cipher, Wheel};

pub struct Alberti {
    plainwheel: Wheel<u32>,
    cipherwheel: Wheel<u32>
}

impl Alberti {
    pub fn new(plainwheel: Wheel<u32>, cipherwheel: Wheel<u32>) -> Self {
        Alberti {
            plainwheel,
            cipherwheel
        }
    }
}

impl Cipher for Alberti {
    fn encode(&mut self, message: Vec<u32>) -> Result<Vec<u32>> {
        let mut ciphertext: Vec<u32> = vec![];
        for letter in message {
            ciphertext.push( match self.plainwheel.iter()
                .position(|x| *x == letter) {
                    Some(i) => self.cipherwheel[i],
                    _ => letter
            });
            self.cipherwheel.rotate_left(1);
        }

        Ok(ciphertext)
    }
    fn decode(&mut self, ciphertext: Vec<u32>) -> Result<Vec<u32>> {
        let mut plaintext: Vec<u32> = vec![];
        for letter in ciphertext {
            plaintext.push( match self.cipherwheel.iter()
                .position(|x| *x == letter) {
                    Some(i) => self.plainwheel[i],
                    _ => letter
            });
            self.plainwheel.rotate_right(1);
        }

        Ok(plaintext)
    }
}