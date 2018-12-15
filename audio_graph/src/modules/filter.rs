//! Filters
//!
//! Basic IIR low/high-pass filters.
//!
//! Sources to connect: input, cut-off frequency.
use crate::module::Module;
use crate::sample::{Frame, Sample};

pub struct LPF {
    output: Vec<Sample>,
    sample_angular_period: Sample,
}

impl LPF {
    pub fn new(channels: usize, sample_rate: usize) -> Self {
        let sample_angular_period = 2.0 * std::f64::consts::PI / sample_rate as f64;
        LPF {
            output: vec![0.0; channels],
            sample_angular_period,
        }
    }
}

impl Module for LPF {
    fn inputs(&self) -> u8 {
        2
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        let channels = self.output.len();
        for (channel, value) in self.output.iter_mut().enumerate() {
            let x = input[channel];
            let freq = input[channel + channels];
            let k = freq * self.sample_angular_period;
            let a = k / (k + 1.0);
            *value += a * (x - *value);
        }
    }
}

pub struct HPF {
    output: Vec<Sample>,
    sample_angular_period: Sample,
    x_prime: Vec<Sample>,
}

impl HPF {
    pub fn new(channels: usize, sample_rate: usize) -> Self {
        let sample_angular_period = 2.0 * std::f64::consts::PI / sample_rate as f64;
        HPF {
            output: vec![0.0; channels],
            sample_angular_period,
            x_prime: vec![0.0; channels],
        }
    }
}

impl Module for HPF {
    fn inputs(&self) -> u8 {
        2
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        let channels = self.output.len();
        for (channel, value) in self.output.iter_mut().enumerate() {
            let x = input[channel];
            let x_prime = self.x_prime[channel];
            let freq = input[channel + channels];
            let k = freq * self.sample_angular_period;
            let a = 1.0 / (k + 1.0);
            *value = a * (*value + x - x_prime);
        }
        self.x_prime.clone_from_slice(&input[0..channels]);
    }
}
