use crate::{MINIMUM_LEVEL, SAMPLE_RATE};
use libm::{fmaxf, logf};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum State {
    Start,
    Delay,
    Attack,
    Hold,
    Decay,
    Sustain,
    Release,
    Off,
}

impl Default for State {
    fn default() -> State {
        State::Start
    }
}

impl State {
    fn next(&self) -> State {
        match self {
            State::Start => State::Delay,
            State::Delay => State::Attack,
            State::Attack => State::Hold,
            State::Hold => State::Decay,
            State::Decay => State::Sustain,
            State::Sustain => State::Release,
            State::Release => State::Off,
            State::Off => State::Off,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct EnvelopeSettings {
    delay: f32,
    attack: f32,
    hold: f32,
    decay: f32,
    sustain: f32,
    release: f32,
}

impl EnvelopeSettings {
    pub const fn new(
        delay: f32,
        attack: f32,
        hold: f32,
        decay: f32,
        sustain: f32,
        release: f32,
    ) -> EnvelopeSettings {
        EnvelopeSettings {
            delay,
            attack,
            hold,
            decay,
            sustain,
            release,
        }
    }
}

impl EnvelopeSettings {
    fn value_for_state(&self, state: State) -> f32 {
        match state {
            State::Start => 0.0,
            State::Delay => self.delay,
            State::Attack => self.attack,
            State::Hold => self.hold,
            State::Decay => self.decay,
            State::Sustain => self.sustain,
            State::Release => self.release,
            State::Off => 0.0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Envelope {
    state: State,
    current_level: f32,
    multiplier: f32,
    current_sample_index: f32,
    next_state_sample_index: f32,
}

impl Envelope {
    pub const fn new() -> Envelope {
        Envelope {
            state: State::Start,
            current_level: MINIMUM_LEVEL,
            multiplier: 1.0,
            current_sample_index: 0.0,
            next_state_sample_index: 0.0,
        }
    }

    /// The last computed output level
    ///
    /// # Returns
    ///
    /// The last computed output level
    pub const fn output_level(&self) -> f32 {
        self.current_level
    }

    pub fn is_finished(&self) -> bool {
        self.state == State::Off
    }

    /// Compute the next output level, next state, etc
    ///
    /// # Returns
    ///
    /// The new envelope with updated states, levels, etc.
    pub fn compute(&mut self, settings: &EnvelopeSettings) -> f32 {
        // If the envelope is off or in sustain, there's nothing to do
        // and we can return immediately
        if self.state == State::Off || self.state == State::Sustain {
            return self.current_level;
        }

        if self.current_sample_index >= self.next_state_sample_index {
            self.enter_state(settings, self.state.next());
        }

        self.current_level *= self.multiplier;
        self.current_sample_index += 1.0;

        self.current_level
    }

    pub fn enter_state(&mut self, settings: &EnvelopeSettings, state: State) {
        let next_sample_index = if state == State::Off || state == State::Sustain {
            0.0
        } else {
            settings.value_for_state(state) * SAMPLE_RATE
        };

        let (current_level, multiplier) = match state {
            // Nothing to do for Off
            State::Off => (0.0, 1.0),
            // Nothing to do for Start
            State::Start => (MINIMUM_LEVEL, 1.0),
            // In delay, we stay at 0
            State::Delay => (MINIMUM_LEVEL, 1.0),
            State::Attack => (
                MINIMUM_LEVEL,
                Envelope::multiplier(MINIMUM_LEVEL, 1.0, next_sample_index),
            ),
            State::Hold => (1.0, 1.0),
            State::Decay => (
                1.0,
                Envelope::multiplier(
                    1.0,
                    fmaxf(MINIMUM_LEVEL, settings.value_for_state(State::Sustain)),
                    next_sample_index,
                ),
            ),
            State::Sustain => (settings.value_for_state(State::Sustain), 1.0),
            State::Release => (
                self.current_level,
                Envelope::multiplier(self.current_level, MINIMUM_LEVEL, next_sample_index),
            ),
        };

        self.state = state;
        self.current_level = current_level;
        self.multiplier = multiplier;
        self.current_sample_index = 0.0;
        self.next_state_sample_index = next_sample_index;
    }

    fn multiplier(start_level: f32, end_level: f32, samples: f32) -> f32 {
        1.0 + (logf(end_level) - logf(fmaxf(start_level, MINIMUM_LEVEL))) / samples
    }
}
