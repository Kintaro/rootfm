use crate::{Envelope, EnvelopeSettings, Oscillator, OscillatorSettings, State};

#[derive(Copy, Clone, Debug)]
pub struct OperatorSettings {
    envelope_settings: EnvelopeSettings,
    oscillator_settings: OscillatorSettings,
    enabled: bool,
    output_level: f32,
    sensitivity: f32,
}

impl OperatorSettings {
    pub const fn new(
        envelope_settings: EnvelopeSettings,
        oscillator_settings: OscillatorSettings,
        output_level: f32,
        sensitivity: f32,
    ) -> OperatorSettings {
        OperatorSettings {
            envelope_settings,
            oscillator_settings,
            enabled: true,
            output_level,
            sensitivity,
        }
    }

    pub const fn enabled(&self) -> bool {
        self.enabled
    }

    pub const fn envelope_settings(&self) -> &EnvelopeSettings {
        &self.envelope_settings
    }

    pub const fn oscillator_settings(&self) -> &OscillatorSettings {
        &self.oscillator_settings
    }

    pub const fn output_level(&self) -> f32 {
        self.output_level
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Operator {
    envelope: Envelope,
    oscillator: Oscillator,
    output_level: f32,
}

impl Operator {
    pub fn with_frequency(settings: &OperatorSettings, frequency: f32, velocity: f32) -> Operator {
        Operator {
            envelope: Envelope::new(),
            oscillator: Oscillator::with_frequency(settings.oscillator_settings(), frequency),
            output_level: (1.0 + (velocity - 1.0) * (settings.sensitivity * 7.0))
                * settings.output_level,
        }
    }

    pub fn compute(&mut self, settings: &OperatorSettings, modulation: f32) -> f32 {
        self.oscillator
            .compute(&settings.oscillator_settings, modulation)
            * self.envelope.compute(&settings.envelope_settings)
    }

    pub fn note_on(&mut self, settings: &OperatorSettings) {
        self.envelope
            .enter_state(&settings.envelope_settings, State::Delay);
    }

    pub fn note_off(&mut self, settings: &OperatorSettings) {
        self.envelope
            .enter_state(&settings.envelope_settings, State::Release);
    }

    pub fn is_finished(&self) -> bool {
        self.envelope.is_finished()
    }
}
