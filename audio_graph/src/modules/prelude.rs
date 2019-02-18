//! # Modules prelude
//!
//! Essentially is a re-export of all modules.
pub use crate::modules::biquad::{make_hpf_coefficients, make_lpf_coefficients, BiQuad};
pub use crate::modules::constant::Constant;
pub use crate::modules::delay::Delay;
pub use crate::modules::feedback::Feedback;
pub use crate::modules::filter::{HPF, LPF};
pub use crate::modules::function::{Fn1, Fn2, Fn3};
pub use crate::modules::input::Input;
pub use crate::modules::metro::{DMetro, DMetroHold, Metro, MetroHold};
pub use crate::modules::noise::Noise;
pub use crate::modules::osc::{Osc, OscPhase};
pub use crate::modules::pan::{Pan1, Pan2, Pan3};
pub use crate::modules::parameter::Parameter;
pub use crate::modules::phasor::{Phasor, Phasor0};
pub use crate::modules::pulse::Pulse;
pub use crate::modules::sample_and_hold::SampleAndHold;
pub use crate::modules::yin::Yin;
pub use crate::modules::zip::Zip;
