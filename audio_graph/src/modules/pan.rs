//! # Stereo panner
//!
//! Sources to connect: left, right, position.
use crate::module::Module;
use crate::pure;
use crate::sample::{Frame, Sample};

pub struct Pan1 {
    channels: usize,
    output: Vec<Sample>,
}

impl Pan1 {
    pub fn new(channels: usize) -> Self {
        Pan1 {
            channels,
            output: vec![0.0; channels],
        }
    }
}

impl Module for Pan1 {
    fn inputs(&self) -> u8 {
        2
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        let (l, r) = pure::pan(input[0], input[1], input[self.channels]);
        self.output[0] = l;
        self.output[1] = r;
    }
}

pub struct Pan2 {
    channels: usize,
    output: Vec<Sample>,
}

impl Pan2 {
    pub fn new(channels: usize) -> Self {
        Pan2 {
            channels,
            output: vec![0.0; channels],
        }
    }
}

impl Module for Pan2 {
    fn inputs(&self) -> u8 {
        3
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        let channels = self.channels;
        let l = input[0]; // left of the first input
        let r = input[1 + channels]; // right of the second input
        let c = input[2 * channels]; // left of the position
        let (l, r) = pure::pan(l, r, c);
        self.output[0] = l;
        self.output[1] = r;
    }
}

pub struct Pan3 {
    channels: usize,
    output: Vec<Sample>,
}

impl Pan3 {
    pub fn new(channels: usize) -> Self {
        Pan3 {
            channels,
            output: vec![0.0; channels],
        }
    }
}

impl Module for Pan3 {
    fn inputs(&self) -> u8 {
        3
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        let channels = self.channels;
        for (channel, output) in self.output.iter_mut().enumerate() {
            let l = input[channel];
            let r = input[channel + channels];
            let c = input[channel + 2 * channels];
            *output = match channel {
                0 => 1.0_f64.min(1.0 - c).sqrt() * l + 0.0_f64.max(-c).sqrt() * r,
                1 => 0.0_f64.max(c).sqrt() * l + 1.0_f64.min(1.0 + c).sqrt() * r,
                _ => 0.0,
            }
        }
    }
}
