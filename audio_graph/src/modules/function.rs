//! # Functions
//!
//! Fn*N* modules allow to use regular numeric functions to transform input of *N* sources.
//!
//! Sources to connect: *N*, one for each argument of pure function.
use crate::module::Module;
use crate::sample::{Frame, Sample};

pub struct Fn1 {
    ys: Vec<Sample>,
    f: fn(Sample) -> Sample,
}

impl Fn1 {
    pub fn new(channels: usize, f: fn(Sample) -> Sample) -> Self {
        Fn1 {
            ys: vec![0.0; channels],
            f,
        }
    }
}

impl Module for Fn1 {
    fn inputs(&self) -> u8 {
        1
    }

    fn output(&self) -> &Frame {
        &self.ys
    }

    fn sample(&mut self, input: &Frame) {
        for i in 0..self.ys.len() {
            self.ys[i] = (self.f)(input[i]);
        }
    }
}

pub struct Fn2 {
    ys: Vec<Sample>,
    f: fn(Sample, Sample) -> Sample,
}

impl Fn2 {
    pub fn new(channels: usize, f: fn(Sample, Sample) -> Sample) -> Self {
        Fn2 {
            ys: vec![0.0; channels],
            f,
        }
    }
}

impl Module for Fn2 {
    fn inputs(&self) -> u8 {
        2
    }

    fn output(&self) -> &Frame {
        &self.ys
    }

    fn sample(&mut self, input: &Frame) {
        let channels = self.ys.len();
        for i in 0..channels {
            self.ys[i] = (self.f)(input[i], input[i + channels]);
        }
    }
}

pub struct Fn3 {
    ys: Vec<Sample>,
    f: fn(Sample, Sample, Sample) -> Sample,
}

impl Fn3 {
    pub fn new(channels: usize, f: fn(Sample, Sample, Sample) -> Sample) -> Self {
        Fn3 {
            ys: vec![0.0; channels],
            f,
        }
    }
}

impl Module for Fn3 {
    fn inputs(&self) -> u8 {
        3
    }

    fn output(&self) -> &Frame {
        &self.ys
    }

    fn sample(&mut self, input: &Frame) {
        let channels = self.ys.len();
        for i in 0..channels {
            self.ys[i] = (self.f)(input[i], input[i + channels], input[i + 2 * channels]);
        }
    }
}
