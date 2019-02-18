use crate::module::Module;
use crate::sample::{Frame, Sample};

pub struct Metro {
    output: Vec<Sample>,
    last_trigger: Vec<u64>,
    frame_number: u64,
    sample_rate: Sample,
}

impl Metro {
    pub fn new(channels: usize, sample_rate: usize) -> Self {
        Metro {
            output: vec![0.0; channels],
            last_trigger: vec![0; channels],
            frame_number: 0,
            sample_rate: sample_rate as Sample,
        }
    }
}

impl Module for Metro {
    fn inputs(&self) -> u8 {
        1
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        for (channel, (output, last_trigger)) in self
            .output
            .iter_mut()
            .zip(self.last_trigger.iter_mut())
            .enumerate()
        {
            let frequency = input[channel];
            let delta = self.sample_rate / frequency;
            *output = if delta as u64 <= self.frame_number - *last_trigger {
                *last_trigger = self.frame_number;
                1.0
            } else {
                0.0
            };
        }
        self.frame_number += 1;
    }
}

pub struct DMetro {
    output: Vec<Sample>,
    last_trigger: Vec<u64>,
    frame_number: u64,
    sample_rate: Sample,
}

impl DMetro {
    pub fn new(channels: usize, sample_rate: usize) -> Self {
        DMetro {
            output: vec![0.0; channels],
            last_trigger: vec![0; channels],
            frame_number: 0,
            sample_rate: sample_rate as Sample,
        }
    }
}

impl Module for DMetro {
    fn inputs(&self) -> u8 {
        1
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        for (channel, (output, last_trigger)) in self
            .output
            .iter_mut()
            .zip(self.last_trigger.iter_mut())
            .enumerate()
        {
            let dt = input[channel];
            let delta = self.sample_rate * dt;
            *output = if delta as u64 <= self.frame_number - *last_trigger {
                *last_trigger = self.frame_number;
                1.0
            } else {
                0.0
            };
        }
        self.frame_number += 1;
    }
}

pub struct MetroHold {
    output: Vec<Sample>,
    frequencies: Vec<Sample>,
    last_trigger: Vec<u64>,
    frame_number: u64,
    sample_rate: Sample,
}

impl MetroHold {
    pub fn new(channels: usize, sample_rate: usize) -> Self {
        MetroHold {
            output: vec![0.0; channels],
            frequencies: vec![0.0; channels],
            last_trigger: vec![0; channels],
            frame_number: 0,
            sample_rate: sample_rate as Sample,
        }
    }
}

impl Module for MetroHold {
    fn inputs(&self) -> u8 {
        1
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        for (channel, ((output, last_trigger), last_frequency)) in self
            .output
            .iter_mut()
            .zip(self.last_trigger.iter_mut())
            .zip(self.frequencies.iter_mut())
            .enumerate()
        {
            let frequency = input[channel];
            if *last_frequency == 0.0 {
                *last_frequency = frequency
            }
            let delta = self.sample_rate / *last_frequency;
            *output = if delta as u64 <= self.frame_number - *last_trigger {
                *last_trigger = self.frame_number;
                *last_frequency = frequency;
                1.0
            } else {
                0.0
            };
        }
        self.frame_number += 1;
    }
}

pub struct DMetroHold {
    output: Vec<Sample>,
    dts: Vec<Sample>,
    last_trigger: Vec<u64>,
    frame_number: u64,
    sample_rate: Sample,
}

impl DMetroHold {
    pub fn new(channels: usize, sample_rate: usize) -> Self {
        DMetroHold {
            output: vec![0.0; channels],
            dts: vec![0.0; channels],
            last_trigger: vec![0; channels],
            frame_number: 0,
            sample_rate: sample_rate as Sample,
        }
    }
}

impl Module for DMetroHold {
    fn inputs(&self) -> u8 {
        1
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        for (channel, ((output, last_trigger), last_dt)) in self
            .output
            .iter_mut()
            .zip(self.last_trigger.iter_mut())
            .zip(self.dts.iter_mut())
            .enumerate()
        {
            let dt = input[channel];
            if *last_dt == 0.0 {
                *last_dt = dt
            }
            let delta = self.sample_rate * *last_dt;
            *output = if delta as u64 <= self.frame_number - *last_trigger {
                *last_trigger = self.frame_number;
                *last_dt = dt;
                1.0
            } else {
                0.0
            };
        }
        self.frame_number += 1;
    }
}
