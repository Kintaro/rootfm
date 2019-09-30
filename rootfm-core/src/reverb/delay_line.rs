use super::{interpolate, MAX_BUFFER_SIZE};
use crate::SAMPLE_RATE;
use libm::floorf;

#[derive(Copy, Clone, Debug)]
pub struct DelayLine {
    buffer: [f32; MAX_BUFFER_SIZE],
    read_position: usize,
    write_position: usize,
    fraction: f32,
    delay_samples: f32,
    delay_ms: f32,
}

fn number_samples_from_ms(delay: f32) -> f32 {
    SAMPLE_RATE * delay * 0.001
}

impl DelayLine {
    pub fn new(delay: f32) -> DelayLine {
        let samples = delay as f32 * 0.001 * SAMPLE_RATE;
        let fraction = floorf(number_samples_from_ms(delay) - samples);
        DelayLine {
            buffer: [0.0; MAX_BUFFER_SIZE],
            read_position: 0,
            write_position: 0,
            fraction,
            delay_ms: delay,
            delay_samples: floorf(number_samples_from_ms(delay)),
        }
    }

    #[allow(unused)]
    pub fn set_delay(&mut self, delay: f32) {
        let delay_samples = number_samples_from_ms(delay);
        self.delay_samples = floorf(delay_samples);
        self.fraction = delay_samples - self.delay_samples;
        self.read_position = self.write_position - self.delay_samples as usize;
        if self.write_position < self.delay_samples as usize {
            self.write_position += MAX_BUFFER_SIZE;
        }
    }

    pub fn read_delay(&mut self) -> f32 {
        let sample = self.buffer[self.read_position];
        let previous_read_position = if self.read_position < 1 {
            MAX_BUFFER_SIZE - 1
        } else {
            self.read_position - 1
        };
        let previous_sample = self.buffer[previous_read_position];
        interpolate(0.0, 1.0, sample, previous_sample, self.fraction)
    }

    pub fn write_delay(&mut self, sample: f32) {
        self.buffer[self.write_position] = sample;
        self.write_position = (self.write_position + 1) % MAX_BUFFER_SIZE;
        self.read_position = (self.write_position + 1) % MAX_BUFFER_SIZE;
    }

    pub fn compute(&mut self, sample: f32) -> f32 {
        let val = if self.delay_samples == 0.0 {
            sample
        } else {
            self.read_delay()
        };
        self.write_delay(sample);
        val
    }
}
