//! Input
//!
//! Forward external input to output.
//!
//! Sources to connect: none required.

use crate::module::Module;
use crate::sample::{Frame, Sample};

pub struct Input {
    output: Vec<Sample>,
}

impl Input {
    pub fn new(channels: usize) -> Self {
        Input {
            output: vec![0.0; channels],
        }
    }
}

impl Module for Input {
    fn inputs(&self) -> u8 {
        0
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        let channels = self.output.len();
        self.output.clone_from_slice(&input[..channels]);
    }
}
