//! BiQuad Filters
//!
//! Sources to connect: input, cut-off frequency, Q.
use crate::module::Module;
use crate::sample::{Frame, Sample};

type MakeCoefficients =
    fn(Sample, Sample, Sample) -> (Sample, Sample, Sample, Sample, Sample, Sample);

pub fn make_lpf_coefficients(
    _sin_o: Sample,
    cos_o: Sample,
    alpha: Sample,
) -> (Sample, Sample, Sample, Sample, Sample, Sample) {
    let b1 = 1.0 - cos_o;
    let b0 = 0.5 * b1;
    (b0, b1, b0, 1.0 + alpha, -2.0 * cos_o, 1.0 - alpha)
}

pub fn make_hpf_coefficients(
    _sin_o: Sample,
    cos_o: Sample,
    alpha: Sample,
) -> (Sample, Sample, Sample, Sample, Sample, Sample) {
    let k = 1.0 + cos_o;
    let b0 = 0.5 * k;
    let b1 = -k;
    (b0, b1, b0, 1.0 + alpha, -2.0 * cos_o, 1.0 - alpha)
}

pub struct BiQuad {
    make_coefficients: MakeCoefficients,
    output: Vec<Sample>,
    sample_angular_period: Sample,
    x1: Vec<Sample>,
    x2: Vec<Sample>,
    y2: Vec<Sample>,
}

impl BiQuad {
    pub fn new(channels: usize, sample_rate: usize, make_coefficients: MakeCoefficients) -> Self {
        let sample_angular_period = 2.0 * std::f64::consts::PI / sample_rate as f64;
        BiQuad {
            make_coefficients,
            output: vec![0.0; channels],
            sample_angular_period,
            x1: vec![0.0; channels],
            x2: vec![0.0; channels],
            y2: vec![0.0; channels],
        }
    }
}

impl Module for BiQuad {
    fn inputs(&self) -> u8 {
        3
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        let channels = self.output.len();
        for (channel, y) in self.output.iter_mut().enumerate() {
            let x = input[channel];
            let freq = input[channel + channels];
            let q = input[channel + 2 * channels];

            let x1 = self.x1[channel];
            let x2 = self.x2[channel];
            let y1 = *y;
            let y2 = self.y2[channel];

            let o = freq * self.sample_angular_period;
            let sin_o = o.sin();
            let cos_o = o.cos();
            let alpha = sin_o / (2.0 * q);
            let (b0, b1, b2, a0, a1, a2) = (self.make_coefficients)(sin_o, cos_o, alpha);
            *y = (x * b0 + x1 * b1 + x2 * b2 - y1 * a1 - y2 * a2) / a0;

            self.x2[channel] = x1;
            self.x1[channel] = x;
            self.y2[channel] = y1;
        }
    }
}
