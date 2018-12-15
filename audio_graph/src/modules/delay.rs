//! # Delay
//!
//! Variable signal delay up to maximum period.
//!
//! Sources to connect: input to delay, delay time.
use crate::module::Module;
use crate::sample::{Frame, Sample};

pub struct Delay {
    buffer: Vec<Sample>,
    channels: usize,
    mask: usize,
    frame_number: usize,
    sample_rate: Sample,
    output: Vec<Sample>,
}

impl Delay {
    pub fn new(channels: usize, sample_rate: usize, max_delay: f64) -> Self {
        let sample_rate = sample_rate as Sample;
        // +1 because interpolation looks for the next sample
        // next_power_of_two to trade memory for speed by replacing `mod` with `&`
        let max_delay_frames =
            ((sample_rate as Sample * max_delay) as usize + 1).next_power_of_two();
        let mask = max_delay_frames - 1;
        let buffer = vec![0.0; channels * max_delay_frames];
        Delay {
            buffer,
            channels,
            frame_number: 0,
            mask,
            output: vec![0.0; channels],
            sample_rate,
        }
    }
}

impl Module for Delay {
    fn inputs(&self) -> u8 {
        2
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        for channel in 0..self.channels {
            let x = input[channel];
            let z = input[channel + self.channels] * self.sample_rate;
            let delay = z as usize;
            let k = z.fract();
            if self.frame_number > delay {
                let i = self.frame_number - delay;
                let a = self.buffer[((i - 1) & self.mask) * self.channels + channel];
                let b = self.buffer[(i & self.mask) * self.channels + channel];
                self.output[channel] = k * a + (1.0 - k) * b;
            }
            self.buffer[(self.frame_number & self.mask) * self.channels + channel] = x;
        }
        self.frame_number += 1;
    }
}
