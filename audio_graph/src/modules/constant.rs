//! # Constant
//!
//! Constant module always outputs the same given sample in all channels.
//!
//! Sources to connect: none required.
use crate::module::Module;
use crate::sample::{Frame, Sample};

pub struct Constant {
    values: Vec<Sample>,
}

impl Constant {
    pub fn new(channels: usize, x: Sample) -> Self {
        Constant {
            values: vec![x; channels],
        }
    }
}

impl Module for Constant {
    fn inputs(&self) -> u8 {
        0
    }

    fn output(&self) -> &Frame {
        &self.values
    }

    fn sample(&mut self, _input: &Frame) {}
}
