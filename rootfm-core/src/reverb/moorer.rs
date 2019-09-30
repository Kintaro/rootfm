use super::all_pass::AllPass;
use super::comb::Comb;
use super::delay_line::DelayLine;
use super::early_reflection_tap_delay_line::EarlyReflectionTapDelayLine;

const NUM_COMBS: usize = 6;

#[derive(Copy, Clone, Debug)]
pub struct Moorer {
    combs: [Comb; NUM_COMBS],
    all_pass: AllPass,
    er_generator: EarlyReflectionTapDelayLine,
    delay_line: DelayLine,
    bypass: bool,
}

impl Moorer {
    pub fn new(late_delay: f32) -> Moorer {
        let gain = 0.707;
        Moorer {
            combs: [
                Comb::new(50.0, 1.0, gain),
                Comb::new(56.0, 1.0, gain),
                Comb::new(61.0, 1.0, gain),
                Comb::new(68.0, 1.0, gain),
                Comb::new(72.0, 1.0, gain),
                Comb::new(78.0, 1.0, gain),
            ],
            all_pass: AllPass::new(6.0, 0.707),
            bypass: false,
            er_generator: EarlyReflectionTapDelayLine::new(late_delay),
            delay_line: DelayLine::new(late_delay),
        }
    }

    pub fn compute(&mut self, sample: f32) -> f32 {
        if self.bypass {
            return sample;
        }

        let ers = self.er_generator.compute(sample);
        let out = self
            .combs
            .iter_mut()
            .map(|comb| comb.compute(ers * 0.25))
            .sum();
        let pass_out = self.all_pass.compute(out);
        let shift = self.delay_line.compute(pass_out);
        ers * 0.25 + shift
    }
}
