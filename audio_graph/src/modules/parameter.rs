//! Parameter
//!
//! Extract parameter from external input by index.
//!
//! Sources to connect: none required.

use crate::module::Module;
use crate::sample::{Frame, Sample};

pub struct Parameter {
    index: usize,
    output: Vec<Sample>,
}

impl Parameter {
    pub fn new(channels: usize, index: usize) -> Self {
        Parameter {
            index,
            output: vec![0.0; channels],
        }
    }
}

impl Module for Parameter {
    fn inputs(&self) -> u8 {
        0
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        let channels = self.output.len();
        let value = input[channels + self.index];
        for output in self.output.iter_mut() {
            *output = value;
        }
    }
}
