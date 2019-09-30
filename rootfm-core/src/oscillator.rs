use crate::{PERIOD, SAMPLE_RATE};
use libm::{fabsf, floorf, sinf};

pub const SINE_OSCILLATOR: OscillatorSettings = OscillatorSettings {
    oscillator_type: OscillatorType::Sine,
    ratio: Ratio::Ratio(1.0),
    detune: 0.0,
};

#[derive(Copy, Clone, Debug)]
pub enum OscillatorType {
    Sine,
}

#[derive(Copy, Clone, Debug)]
pub enum Ratio {
    Ratio(f32),
    Fixed(f32),
}

impl Ratio {
    pub fn apply(&self, frequency: f32) -> f32 {
        match self {
            Ratio::Ratio(r) => r * frequency,
            Ratio::Fixed(f) => *f,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct OscillatorSettings {
    oscillator_type: OscillatorType,
    ratio: Ratio,
    detune: f32,
}

impl OscillatorSettings {
    pub const fn new(
        oscillator_type: OscillatorType,
        ratio: Ratio,
        detune: f32,
    ) -> OscillatorSettings {
        OscillatorSettings {
            oscillator_type,
            ratio,
            detune,
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Oscillator {
    phase: f32,
    phase_step: f32,
}

const PI: f32 = 3.141593;
const HALF_PI: f32 = 1.570796;
const TWO_PI: f32 = 6.283185;
const THREE_HALF_PI: f32 = 4.7123889;
const INV_TWO_PI: f32 = 0.1591549;

fn cos_32s(x: f32) -> f32 {
    const c1: f32 = 0.99940307;
    const c2: f32 = -0.49558072;
    const c3: f32 = 0.03679168;
    let x2 = x * x;
    c1 + x2 * (c2 + c3 * x2)
}

fn cos(mut angle: f32) -> f32 {
    //clamp to the range 0..2PI
    angle = angle - floorf(angle * INV_TWO_PI) * TWO_PI;
    angle = fabsf(angle);

    if angle < HALF_PI {
        cos_32s(angle)
    } else if angle < PI {
        -cos_32s(PI - angle)
    } else if angle < THREE_HALF_PI {
        -cos_32s(angle - PI)
    } else {
        cos_32s(TWO_PI - angle)
    }
}
fn sin(angle: f32) -> f32 {
    cos(HALF_PI - angle)
}

impl Oscillator {
    pub fn with_frequency(settings: &OscillatorSettings, frequency: f32) -> Oscillator {
        Oscillator {
            phase: 0.0,
            phase_step: PERIOD * settings.ratio.apply(frequency) / SAMPLE_RATE,
        }
    }

    pub fn compute(&mut self, settings: &OscillatorSettings, modulation: f32) -> f32 {
        match settings.oscillator_type {
            OscillatorType::Sine => {
                let value = sin(self.phase + modulation);
                self.phase += self.phase_step;
                value
            }
        }
    }
}
