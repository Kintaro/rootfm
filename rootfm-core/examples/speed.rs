use cpal::traits::*;
use cpal::*;
use hound;
use libm::floorf;
use midly::*;
use rootfm_core::*;
use std::fs;
use std::path;

pub static PRESETS: &[Preset] = &[PRESET_1, PRESET_2, PRESET_3, PRESET_4];
pub const NUM_PRESETS: usize = 4;
pub static ALGORITHMS: &[Algorithm] = &[
    ALGORITHM_1,
    ALGORITHM_2,
    ALGORITHM_3,
    ALGORITHM_4,
    ALGORITHM_5,
    ALGORITHM_6,
    ALGORITHM_7,
    ALGORITHM_8,
    ALGORITHM_9,
    ALGORITHM_10,
    ALGORITHM_11,
    ALGORITHM_12,
    ALGORITHM_13,
    ALGORITHM_14,
];

pub const NUM_ALGORITHMS: usize = 14;

#[derive(Copy, Clone)]
pub enum Note {
    On(u32, u32, u32),
    Off(u32, u32),
    Tempo(u32, u32),
}

impl Note {
    pub fn delta(&self) -> u32 {
        match self {
            &Note::On(_, _, d) => d,
            &Note::Off(_, d) => d,
            &Note::Tempo(_, d) => d,
        }
    }

    pub fn update_delta(&mut self, d: u32) {
        match self {
            Note::On(_, _, ref mut delta) => *delta -= d,
            Note::Off(_, ref mut delta) => *delta -= d,
            Note::Tempo(_, ref mut delta) => *delta -= d,
        }
    }
}

struct Track {
    notes: Vec<Note>,
}

impl Track {
    pub fn peek(&self) -> Option<Note> {
        if self.notes.is_empty() {
            None
        } else {
            Some(self.notes[0])
        }
    }

    pub fn update_delta(&mut self, d: u32) {
        if self.notes.is_empty() {
            return;
        }
        self.notes[0].update_delta(d);
    }
}

struct Song {
    tracks: Vec<Track>,
}

impl Song {
    pub fn preprocess(&mut self) -> Vec<Note> {
        let mut result = Vec::new();
        while let Some(note) = self.next_note() {
            result.push(note);
        }
        result
    }

    pub fn next_note(&mut self) -> Option<Note> {
        let mut track_index = std::usize::MAX;
        let mut delta = std::u32::MAX;

        for (track, note) in self.tracks.iter().map(|t| t.peek()).enumerate() {
            if let Some(n) = note {
                if n.delta() < delta {
                    track_index = track;
                    delta = n.delta();
                }
            }
        }

        if track_index == std::usize::MAX || delta == std::u32::MAX {
            return None;
        }

        let note = self.tracks[track_index].notes.remove(0);

        for i in 0..self.tracks.len() {
            if i == track_index {
                continue;
            }
            self.tracks[i].update_delta(note.delta())
        }

        Some(note)
    }
}

fn main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let path = path::Path::new("test.mid");
    let contents = fs::read(path).unwrap();
    let smf = Smf::parse(&contents).unwrap();
    let mut synthesizer = Synthesizer::new(PRESET_1);
    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();

    let mut song = Song {
        tracks: smf
            .tracks
            .iter()
            .map(|track| Track {
                notes: track
                    .iter()
                    .map(|event| match event.kind {
                        EventKind::Midi { message, .. } => match message {
                            MidiMessage::NoteOn { key, vel } if vel.as_int() == 0 => {
                                Some(Note::Off(key.as_int() as u32, event.delta.as_int() as u32))
                            }
                            MidiMessage::NoteOn { key, vel } => Some(Note::On(
                                key.as_int() as u32,
                                vel.as_int() as u32,
                                event.delta.as_int() as u32,
                            )),
                            MidiMessage::NoteOff { key, .. } => {
                                Some(Note::Off(key.as_int() as u32, event.delta.as_int() as u32))
                            }
                            _ => None,
                        },
                        EventKind::Meta(message) => match message {
                            MetaMessage::Tempo(tempo) => Some(Note::Tempo(
                                tempo.as_int() as u32,
                                event.delta.as_int() as u32,
                            )),
                            _ => None,
                        },
                        _ => None,
                    })
                    .filter(Option::is_some)
                    .map(|x| x.unwrap())
                    .collect(),
            })
            .collect(),
    };

    let mut us_per_beat = 500000;
    let ticks_per_beat = match smf.header.timing {
        Timing::Metrical(t) => t.as_int() as u32,
        _ => 0,
    };
    let mut us_per_tick = us_per_beat as f32 / ticks_per_beat as f32;

    println!("Sample rate: {}", format.sample_rate.0 as f32);

    let mut wait_us = 0.0; //us_per_tick * note.delta() as f32;
    let mut cycles = 0.0; //(wait_us * SAMPLE_RATE) / 1_000_000.0;
    let mut current_cycle = 0.0;
    let notes = song.preprocess();
    let mut current_note = 0;
    let mut counter = 0;

    for note in notes {
    }

                    counter += 1;
                    while current_cycle > cycles {
                        if current_note >= notes.len() {
                            return;
                        }
                        match notes[current_note] {
                            Note::On(key, velocity, _) => {
                                synthesizer.note_on(key, velocity as f32 / 127.0);
                            }
                            Note::Off(key, _) => {
                                synthesizer.note_off(key);
                            }
                            Note::Tempo(tempo, _) => {
                                us_per_beat = tempo;
                                us_per_tick = us_per_beat as f32 / ticks_per_beat as f32;
                            }
                        }
                        wait_us = us_per_tick * notes[current_note + 1].delta() as f32;
                        cycles = (wait_us * SAMPLE_RATE as f32) / 1_000_000.0;
                        current_cycle = 0.0;
                        current_note += 1;
                    }
                    let sample_value = synthesizer.compute();
                    for out in sample.iter_mut() {
                        *out = sample_value;
                    }
                    writer.write_sample(sample_value).unwrap();
                    current_cycle += 1.0;
                }
            }
            _ => (),
        }
    });
}
