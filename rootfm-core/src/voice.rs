use crate::{Algorithm, Operator, Preset, NUM_OPERATORS};
use libm::powf;

fn note_to_frequency(note: u32) -> f32 {
    440.0 * powf(2.0, (note as f32 - 69.0) / 12.0)
}

#[derive(Copy, Clone, Debug)]
pub struct Voice {
    operators: [Operator; NUM_OPERATORS],
    note: u32,
    velocity: f32,
}

impl Voice {
    pub fn new(preset: &Preset, note: u32, velocity: f32) -> Voice {
        let frequency = note_to_frequency(note);
        let mut operators =
            [Operator::with_frequency(&preset.operator_settings()[0], frequency, velocity);
                NUM_OPERATORS];
        for i in 0..NUM_OPERATORS {
            operators[i] =
                Operator::with_frequency(&preset.operator_settings()[i], frequency, velocity);
        }

        Voice {
            operators,
            note,
            velocity,
        }
    }

    pub fn compute(&mut self, preset: &Preset) -> f32 {
        let mut values = [0.0; NUM_OPERATORS];

        self.operators
            .iter_mut()
            .enumerate()
            .rev()
            .fold(
                [0.0; NUM_OPERATORS],
                |mut values, (operator_index, operator)| {
                    let modulators = preset.algorithm().get_modulators_for(operator_index as u32);

                    let modulation: f32 = (0u8..8u8)
                        .filter(|x| (1u8 << x) & modulators != 0)
                        .map(|x| values[x as usize])
                        .sum();

                    let operator_settings = &preset.operator_settings()[operator_index];
                    let level = operator.compute(operator_settings, modulation);
                    values[operator_index] = if operator_settings.enabled() {
                        if modulators & (1 << operator_index) != 0 {
                            level * preset.feedback()
                        } else {
                            level * operator_settings.output_level()
                        }
                    } else {
                        0.0
                    };
                    values
                },
            )
            .iter()
            .enumerate()
            .filter(|(index, _)| preset.algorithm().is_carrier(*index as u32))
            .map(|(_, value)| value)
            .sum()
    }

    pub fn note(&self) -> u32 {
        self.note
    }

    pub fn velocity(&self) -> f32 {
        self.velocity
    }

    pub fn note_on(&mut self, preset: &Preset) {
        for (operator_index, operator) in self.operators.iter_mut().enumerate() {
            operator.note_on(&preset.operator_settings()[operator_index])
        }
    }

    pub fn note_off(&mut self, preset: &Preset) {
        for (operator_index, operator) in self.operators.iter_mut().enumerate() {
            operator.note_off(&preset.operator_settings()[operator_index])
        }
    }

    pub fn is_finished(&self, algorithm: &Algorithm) -> bool {
        self.operators
            .iter()
            .enumerate()
            .filter(|(operator_index, _)| algorithm.is_carrier(*operator_index as u32))
            .all(|(_, operator)| operator.is_finished())
    }
}
