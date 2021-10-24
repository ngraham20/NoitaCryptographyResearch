use crate::errors::*;
use crate::cipher::{Cipher, Wheel};

pub struct Alberti {
    plainwheel: Wheel<u16>,
    cipherwheel: Wheel<u16>
}

impl Alberti {
    pub fn new(plainwheel: Wheel<u16>, cipherwheel: Wheel<u16>) -> Self {
        Alberti {
            plainwheel,
            cipherwheel
        }
    }
}

impl Cipher for Alberti {
    fn encode(&mut self, message: &[u16]) -> Result<Vec<u16>> {
        let mut ciphertext: Vec<u16> = vec![];
        for letter in message {
            ciphertext.push( match self.plainwheel.iter()
                .position(|x| x == letter) {
                    Some(i) => self.cipherwheel[i],
                    _ => *letter
            });
            self.cipherwheel.rotate_left(1);
        }

        Ok(ciphertext)
    }
    fn decode(&mut self, ciphertext: &[u16]) -> Result<Vec<u16>> {
        let mut plaintext: Vec<u16> = vec![];
        for letter in ciphertext {
            plaintext.push( match self.cipherwheel.iter()
                .position(|x| x == letter) {
                    Some(i) => self.plainwheel[i],
                    _ => *letter
            });
            self.plainwheel.rotate_right(1);
        }

        Ok(plaintext)
    }
}