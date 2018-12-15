//! # Feedback
//!
//! Feedback comb filter with variable delay time and gain.
//!
//! Sources to connect: input to delay, delay time, gain.
use crate::module::Module;
use crate::modules::delay::Delay;
use crate::sample::{Frame, Sample};

pub struct Feedback {
    channels: usize,
    delay: Delay,
    delay_input: Vec<Sample>,
    output: Vec<Sample>,
}

impl Feedback {
    pub fn new(channels: usize, sample_rate: usize, max_delay: f64) -> Self {
        let delay = Delay::new(channels, sample_rate, max_delay);
        Feedback {
            channels,
            delay,
            delay_input: vec![0.0; 2 * channels],
            output: vec![0.0; channels],
        }
    }
}

impl Module for Feedback {
    fn inputs(&self) -> u8 {
        3
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        let channels = self.channels;

        // feed output back to the delay module
        self.delay_input[..channels].clone_from_slice(&self.output);
        // just copy delay time
        self.delay_input[channels..2 * channels].clone_from_slice(&input[channels..2 * channels]);
        self.delay.sample(&self.delay_input);

        let delayed = self.delay.output();
        for (channel, output) in self.output.iter_mut().enumerate() {
            let x = input[channel];
            let gain = input[channel + 2 * channels];
            *output = x + gain * delayed[channel];
        }
    }
}
