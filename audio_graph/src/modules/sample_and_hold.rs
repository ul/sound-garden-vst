//! Sample & Hold
//!
//! Sources to connect: trigger, input.
use crate::module::Module;
use crate::sample::{Frame, Sample};

pub struct SampleAndHold {
    output: Vec<Sample>,
}

impl SampleAndHold {
    pub fn new(channels: usize) -> Self {
        SampleAndHold {
            output: vec![0.0; channels],
        }
    }
}

impl Module for SampleAndHold {
    fn inputs(&self) -> u8 {
        2
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        let channels = self.output.len();
        for (channel, output) in self.output.iter_mut().enumerate() {
            let t = input[channel];
            let x = input[channel + channels];
            *output = *output * (1.0 - t) + x * t
        }
    }
}
