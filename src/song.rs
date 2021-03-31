use crate::note::Note;
use crate::util::{lin, map};
use std::io::Write;

/// raw audio
pub struct Song {
    samples: Vec<f64>,
    sample_rate: usize,
}

impl Song {
    pub fn new(duration: f64) -> Self {
        assert!(
            duration >= 0.0,
            "What kinda crazy mischevities are you attempting?"
        );
        let sample_rate = 44_100; // cd quality
        let num_samples = (duration * sample_rate as f64) as usize;
        let samples = vec![0.0; num_samples];
        Self {
            samples,
            sample_rate,
        }
    }

    pub fn add_note<T: Fn(f64) -> f64>(&mut self, note: Note<T>) {
        let start = note.time;
        let sample_start = (self.sample_rate as f64 * start) as usize;
        let end = note.time + note.dur;
        let sample_end = (self.sample_rate as f64 * end) as usize;
        let attack_end = map(note.attack_end, 0.0, 1.0, start, end);
        let sustain_start = map(note.decay_start, 0.0, 1.0, start, end);
        assert!(sample_end < self.samples.len());
        for i in sample_start..sample_end {
            let t = i as f64 / self.sample_rate as f64;

            let attack_amp = lin(start, 0.0, attack_end, 1.0, t);
            let sustatain_amp = lin(sustain_start, 1.0, end, 0.0, t);
            let envelope_amp = attack_amp.min(sustatain_amp).max(0.0).min(1.0);

            self.samples[i] += (note.timbre)(t * note.freq) * envelope_amp * note.amp;
        }
    }

    /// dump audio data to stdout as wav
    pub fn dump(&self) {
        let bd = wav::BitDepth::Sixteen(
            self.samples
                .iter()
                .map(|f| (i16::max_value() as f64 * f) as i16)
                .collect(),
        );
        let h = wav::Header::new(1, 1, self.sample_rate as u32, 16);
        let mut out: Vec<u8> = Vec::new();
        wav::write(h, &bd, &mut std::io::Cursor::new(&mut out)).unwrap();
        std::io::stdout().write_all(&out).unwrap();
    }

    pub fn samples(&self) -> &[f64] {
        &self.samples
    }
}
