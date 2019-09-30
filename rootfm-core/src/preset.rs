use crate::{
    Algorithm, EnvelopeSettings, OperatorSettings, OscillatorSettings, OscillatorType, Ratio,
    ALGORITHM_1, ALGORITHM_2, ALGORITHM_3, ALGORITHM_4, NUM_OPERATORS,
};

#[derive(Copy, Clone, Debug)]
pub struct Preset {
    operator_settings: [OperatorSettings; NUM_OPERATORS],
    algorithm: Algorithm,
    feedback: f32,
}

impl Preset {
    pub const fn operator_settings(&self) -> &[OperatorSettings; NUM_OPERATORS] {
        &self.operator_settings
    }

    pub const fn algorithm(&self) -> &Algorithm {
        &self.algorithm
    }

    pub const fn feedback(&self) -> f32 {
        self.feedback
    }

    pub fn set_algorithm(&mut self, algorithm: Algorithm) {
        self.algorithm = algorithm;
    }
}

pub const ENVELOPE_1: EnvelopeSettings = EnvelopeSettings::new(0.0, 0.01, 0.2, 0.1, 1.0, 1.0);
pub const ENVELOPE_2: EnvelopeSettings = EnvelopeSettings::new(0.0, 0.01, 0.0, 0.5, 0.0, 1.0);
pub const ENVELOPE_3: EnvelopeSettings = EnvelopeSettings::new(0.0, 0.01, 0.0, 0.3, 0.0, 1.0);

pub static PRESET_1: Preset = Preset {
    operator_settings: [
        OperatorSettings::new(
            ENVELOPE_1,
            OscillatorSettings::new(OscillatorType::Sine, Ratio::Ratio(1.0), -3.0),
            0.94,
            0.0,
        ),
        OperatorSettings::new(
            ENVELOPE_2,
            OscillatorSettings::new(OscillatorType::Sine, Ratio::Ratio(5.0), -3.0),
            0.84,
            0.0,
        ),
        OperatorSettings::new(
            ENVELOPE_3,
            OscillatorSettings::new(OscillatorType::Sine, Ratio::Ratio(9.0), -3.0),
            0.99,
            0.0,
        ),
        OperatorSettings::new(
            ENVELOPE_1,
            OscillatorSettings::new(OscillatorType::Sine, Ratio::Ratio(1.01), 7.0),
            0.94,
            0.0,
        ),
        OperatorSettings::new(
            ENVELOPE_2,
            OscillatorSettings::new(OscillatorType::Sine, Ratio::Ratio(5.05), 0.0),
            0.82,
            0.0,
        ),
        OperatorSettings::new(
            ENVELOPE_3,
            OscillatorSettings::new(OscillatorType::Sine, Ratio::Ratio(9.09), 0.0),
            0.99,
            0.0,
        ),
    ],
    algorithm: ALGORITHM_1,
    feedback: 0.0,
};

pub static PRESET_2: Preset = Preset {
    algorithm: ALGORITHM_2,
    ..PRESET_1
};

pub static PRESET_3: Preset = Preset {
    algorithm: ALGORITHM_3,
    ..PRESET_1
};

pub static PRESET_4: Preset = Preset {
    algorithm: ALGORITHM_4,
    ..PRESET_1
};
