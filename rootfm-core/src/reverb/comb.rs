use super::delay_line::DelayLine;
use super::low_pass::LowPass;

#[derive(Copy, Clone, Debug)]
pub struct Comb {
    gain: f32,
    delay: DelayLine,
    low_pass: LowPass,
}

impl Comb {
    pub fn new(delay: f32, cutoff: f32, gain: f32) -> Comb {
        Comb {
            gain,
            delay: DelayLine::new(delay),
            low_pass: LowPass::new(cutoff),
        }
    }

    pub fn gain(&self) -> f32 {
        self.gain
    }

    pub fn compute(&mut self, sample: f32) -> f32 {
        let delay = self.delay.read_delay();
        let delay_attenuation = delay * self.gain;
        let lowpass = self.low_pass.compute(delay_attenuation);

        let d = sample + lowpass;
        self.delay.write_delay(d);
        delay
    }
}
