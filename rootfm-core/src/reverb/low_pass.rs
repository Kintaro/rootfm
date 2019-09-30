use crate::SAMPLE_RATE;
use libm::{cosf, sqrtf};

#[derive(Copy, Clone, Debug)]
pub struct LowPass {
    cutoff: f32,
    coefficient: f32,
    previous: f32,
}

impl LowPass {
    pub fn new(cutoff: f32) -> LowPass {
        let costh = 2.0 - cosf(2.0 * core::f32::consts::PI * cutoff / SAMPLE_RATE);
        LowPass {
            cutoff,
            coefficient: sqrtf(costh * costh - 1.0) - costh,
            previous: 0.0,
        }
    }

    #[allow(unused)]
    pub fn cutoff(&self) -> f32 {
        self.cutoff
    }

    #[allow(unused)]
    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.cutoff = cutoff;
        let costh = 2.0 - cosf(2.0 * core::f32::consts::PI * cutoff / SAMPLE_RATE);
        self.coefficient = sqrtf(costh * costh - 1.0) - costh;
    }

    pub fn compute(&mut self, sample: f32) -> f32 {
        self.previous = sample * (1.0 + self.coefficient) - (self.previous * self.coefficient);
        self.previous
    }
}
