#![deny(warnings)]
#![deny(unsafe_code)]
#![no_std]
#![feature(const_fn)]

mod algorithm;
mod envelope;
mod operator;
mod oscillator;
mod preset;
mod reverb;
mod synthesizer;
mod voice;

pub use algorithm::{
    Algorithm, ALGORITHM_1, ALGORITHM_10, ALGORITHM_11, ALGORITHM_12, ALGORITHM_13, ALGORITHM_14,
    ALGORITHM_15, ALGORITHM_16, ALGORITHM_17, ALGORITHM_18, ALGORITHM_19, ALGORITHM_2,
    ALGORITHM_20, ALGORITHM_21, ALGORITHM_22, ALGORITHM_23, ALGORITHM_24, ALGORITHM_25,
    ALGORITHM_26, ALGORITHM_27, ALGORITHM_28, ALGORITHM_29, ALGORITHM_3, ALGORITHM_30,
    ALGORITHM_31, ALGORITHM_32, ALGORITHM_4, ALGORITHM_5, ALGORITHM_6, ALGORITHM_7, ALGORITHM_8,
    ALGORITHM_9,
};
pub use envelope::{Envelope, EnvelopeSettings, State};
pub use operator::{Operator, OperatorSettings};
pub use oscillator::{Oscillator, OscillatorSettings, OscillatorType, Ratio, SINE_OSCILLATOR};
pub use preset::{Preset, PRESET_1, PRESET_2, PRESET_3, PRESET_4};
pub use reverb::Moorer;
pub use synthesizer::Synthesizer;
pub use voice::Voice;

pub const MINIMUM_LEVEL: f32 = 0.01;
pub const NUM_VOICES: usize = 12;
pub const NUM_OPERATORS: usize = 6;
pub const PERIOD: f32 = core::f32::consts::PI * 2.0;
//pub const SAMPLE_RATE: f32 = 44100.0;
pub const SAMPLE_RATE: f32 = 44100.0;
pub const VOICE_LEVEL: f32 = 0.1;
