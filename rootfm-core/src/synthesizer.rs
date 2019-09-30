use crate::{Moorer, Preset, Voice, NUM_VOICES, VOICE_LEVEL};

#[derive(Copy, Clone, Debug)]
pub struct Synthesizer {
    preset: Preset,
    voices: [Voice; NUM_VOICES],
    active_voices: usize,
    reverb: Moorer,
}

impl Synthesizer {
    pub fn new(preset: Preset) -> Synthesizer {
        Synthesizer {
            preset,
            voices: [Voice::new(&preset, 0, 0.0); NUM_VOICES],
            active_voices: 0,
            reverb: Moorer::new(0.1),
        }
    }

    pub fn preset_mut(&mut self) -> &mut Preset {
        &mut self.preset
    }

    pub fn set_preset(&mut self, preset: Preset) {
        self.preset = preset;
        for voice in self.voices.iter_mut() {
            *voice = Voice::new(&preset, voice.note(), voice.velocity());
        }
    }

    pub fn add_voice(&mut self, voice: Voice) {
        if self.active_voices == NUM_VOICES {
            self.shift_down(0);
        }

        self.voices[self.active_voices] = voice;
        self.active_voices += 1;
    }

    pub fn note_on(&mut self, note: u32, velocity: f32) {
        let voice = Voice::new(&self.preset, note, velocity);
        self.add_voice(voice);
    }

    pub fn note_off(&mut self, note: u32) {
        for voice in self.voices.iter_mut() {
            if voice.note() == note {
                voice.note_off(&self.preset)
            }
        }
    }

    pub fn compute(&mut self) -> f32 {
        let mut level = 0.0;
        let mut remove = None;
        for (voice_index, voice) in self.voices.iter_mut().take(self.active_voices).enumerate() {
            if voice.is_finished(self.preset.algorithm()) {
                remove = Some(voice_index);
                continue;
            }

            level += voice.compute(&self.preset);
        }

        if let Some(voice) = remove {
            self.shift_down(voice);
        }
        level * VOICE_LEVEL

        //self.reverb.compute(level * VOICE_LEVEL)
    }

    pub fn active_voices(&self) -> usize {
        self.active_voices
    }

    pub fn shift_down(&mut self, from: usize) {
        for i in from..NUM_VOICES - 1 {
            self.voices[i] = self.voices[i + 1];
        }
        self.active_voices -= 1;
    }
}
