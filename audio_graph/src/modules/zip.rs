//! # Zip
//!
//! Zip the first channel of each input into multi-channel output.
//!
//! Sources to connect: number of inputs equal to channels count.
use crate::module::Module;
use crate::sample::{Frame, Sample};

pub struct Zip {
    channels: usize,
    output: Vec<Sample>,
}

impl Zip {
    pub fn new(channels: usize) -> Self {
        Zip {
            channels,
            output: vec![0.0; channels],
        }
    }
}

impl Module for Zip {
    fn inputs(&self) -> u8 {
        self.channels as u8
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        for (channel, output) in self.output.iter_mut().enumerate() {
            *output = input[channel * self.channels];
        }
    }
}
