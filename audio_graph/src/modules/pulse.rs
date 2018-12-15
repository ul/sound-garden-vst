//! # Pulse wave
//!
//! Sources to connect: frequency, duty cycle.
use crate::module::Module;
use crate::modules::function::Fn2;
use crate::modules::phasor::Phasor;
use crate::pure::rectangle;
use crate::sample::{Frame, Sample};

pub struct Pulse {
    channels: usize,
    input: Vec<Sample>,
    phasor: Phasor,
    osc: Fn2,
}

impl Pulse {
    pub fn new(channels: usize, sample_rate: usize) -> Self {
        let phasor = Phasor::new(channels, sample_rate);
        let osc = Fn2::new(channels, rectangle);
        let input = vec![0.0; 2 * channels];
        Pulse {
            channels,
            input,
            phasor,
            osc,
        }
    }
}

impl Module for Pulse {
    fn inputs(&self) -> u8 {
        2
    }

    fn output(&self) -> &Frame {
        self.osc.output()
    }

    fn sample(&mut self, input: &Frame) {
        self.phasor.sample(input);
        self.input[..self.channels].clone_from_slice(self.phasor.output());
        self.input[self.channels..].clone_from_slice(&input[self.channels..(2 * self.channels)]);
        self.osc.sample(&self.input);
    }
}
