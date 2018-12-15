//! # Noise
//!
//! White noise.
//!
//! Sources to connect: none required.
use crate::module::Module;
use crate::sample::{Frame, Sample};
use rand::{self, Rng};

pub struct Noise {
    values: Vec<Sample>,
}

impl Noise {
    pub fn new(channels: usize) -> Self {
        Noise {
            values: vec![0.0; channels],
        }
    }
}

impl Module for Noise {
    fn inputs(&self) -> u8 {
        0
    }

    fn output(&self) -> &Frame {
        &self.values
    }

    fn sample(&mut self, _input: &Frame) {
        let mut rng = rand::thread_rng();
        for value in self.values.iter_mut() {
            *value = rng.gen_range(-1.0, 1.0);
        }
    }
}
