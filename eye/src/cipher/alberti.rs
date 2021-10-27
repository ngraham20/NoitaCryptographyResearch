use crate::errors::*;
use crate::cipher::{Cipher, Wheel};

pub struct Alberti {
    plainwheel: Wheel<Vec<u8>>,
    cipherwheel: Wheel<Vec<u8>>
}

impl Alberti {
    pub fn new(plainwheel: Wheel<Vec<u8>>, cipherwheel: Wheel<Vec<u8>>) -> Self {
        Alberti {
            plainwheel,
            cipherwheel
        }
    }
}

impl Cipher for Alberti {
    fn encode(&mut self, message: Vec<Vec<u8>>) -> Result<Vec<Vec<u8>>> {
        let mut ciphertext: Vec<Vec<u8>> = vec![];
        for letter in message {
            ciphertext.push( match self.plainwheel.iter()
                .position(|x| *x == letter) {
                    Some(i) => self.cipherwheel[i].clone(),
                    _ => letter
            });
            self.cipherwheel.rotate_left(1);
        }

        Ok(ciphertext)
    }
    fn decode(&mut self, ciphertext: Vec<Vec<u8>>) -> Result<Vec<Vec<u8>>> {
        let mut plaintext: Vec<Vec<u8>> = vec![];
        for letter in ciphertext {
            plaintext.push( match self.cipherwheel.iter()
                .position(|x| *x == letter) {
                    Some(i) => self.plainwheel[i].clone(),
                    _ => letter
            });
            self.plainwheel.rotate_right(1);
        }

        Ok(plaintext)
    }
}