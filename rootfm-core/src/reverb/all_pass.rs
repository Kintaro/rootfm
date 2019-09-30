use super::delay_line::DelayLine;

#[derive(Copy, Clone, Debug)]
pub struct AllPass {
    gain: f32,
    delay: DelayLine,
}

impl AllPass {
    pub fn new(delay: f32, gain: f32) -> AllPass {
        AllPass {
            gain,
            delay: DelayLine::new(delay),
        }
    }

    pub fn compute(&mut self, sample: f32) -> f32 {
        let delay = self.delay.read_delay();
        let filter = sample + (self.gain * delay);
        let out = -self.gain * filter + delay;
        self.delay.write_delay(filter);
        out
    }
}
