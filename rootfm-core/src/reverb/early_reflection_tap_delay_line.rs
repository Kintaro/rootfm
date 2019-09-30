use super::{interpolate, MAX_BUFFER_SIZE};
use crate::SAMPLE_RATE;
use libm::floorf;

const EARLY_REFLECTION_TAPS: [f32; 18] = [
    0.0043, 0.0215, 0.0225, 0.0268, 0.0270, 0.0298, 0.0458, 0.0485, 0.0572, 0.0587, 0.0595, 0.0612,
    0.0707, 0.0708, 0.0726, 0.0741, 0.0753, 0.0797,
];

const EARLY_REFLECTION_GAINS: [f32; 18] = [
    0.841, 0.504, 0.491, 0.379, 0.380, 0.346, 0.289, 0.272, 0.192, 0.193, 0.217, 0.181, 0.180,
    0.181, 0.176, 0.142, 0.167, 0.134,
];

#[derive(Copy, Clone, Debug)]
pub struct EarlyReflectionTapDelayLine {
    write_position: usize,
    read_position: usize,
    fraction: f32,
    buffer: [f32; MAX_BUFFER_SIZE],
    delay_samples: f32,
}

impl EarlyReflectionTapDelayLine {
    pub fn new(delay: f32) -> EarlyReflectionTapDelayLine {
        let delay_samples = SAMPLE_RATE * delay;
        EarlyReflectionTapDelayLine {
            write_position: 0,
            read_position: 0,
            buffer: [0.0; MAX_BUFFER_SIZE],
            delay_samples: floorf(delay_samples),
            fraction: delay_samples - floorf(delay_samples),
        }
    }

    fn set_delay(&mut self, delay: f32) {
        let delay_samples = SAMPLE_RATE * delay;
        self.delay_samples = floorf(delay_samples);
        self.read_position = if self.write_position < self.delay_samples as usize {
            self.write_position + MAX_BUFFER_SIZE - self.delay_samples as usize
        } else {
            self.write_position - self.delay_samples as usize
        };
    }

    pub fn read_delay(&mut self, sample: f32) -> f32 {
        sample
            + EARLY_REFLECTION_TAPS
                .iter()
                .zip(EARLY_REFLECTION_GAINS.iter())
                .map(|(tap, gain)| {
                    self.set_delay(*tap);
                    let y = self.buffer[self.read_position];
                    let y2 = self.buffer[self.read_position - 1];
                    let interpolation = interpolate(0.0, 1.0, y, y2, self.fraction);
                    interpolation * gain
                })
                .sum::<f32>()
    }

    pub fn write_delay(&mut self, sample: f32) {
        self.buffer[self.write_position] = sample;
        self.write_position = (self.write_position + 1) % MAX_BUFFER_SIZE;
        self.read_position = (self.read_position + 1) % MAX_BUFFER_SIZE;
    }

    pub fn compute(&mut self, sample: f32) -> f32 {
        let y = self.read_delay(sample);
        self.write_delay(sample);
        y
    }
}
