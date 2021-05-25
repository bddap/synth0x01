use crate::effect::Effect;
use std::io::Write;

#[derive(Default)]
pub struct Song {
    transforms: Vec<Box<dyn Effect>>,
}

impl Song {
    pub fn add_effect(&mut self, e: impl Effect + 'static) {
        self.transforms.push(Box::new(e))
    }

    /// dump audio data to stdout as wav
    pub fn dump(&self) {
        let duration = self
            .transforms
            .iter()
            .filter_map(|tr| tr.end_hint())
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .expect("can't determine song length");
        let sample_rate = 44_100; // cd quality
        let num_samples = (duration * sample_rate as f64) as usize;
        let mut samples = vec![0.0; num_samples];
        for transform in &self.transforms {
            transform.apply(&mut samples, sample_rate);
        }
        samples.iter().for_each(|s| {
            assert!(!s.is_nan(), "NaN!");
            assert!(*s <= 1.0, "clipping!");
            assert!(*s >= -1.0, "clipping!");
        });
        let bd = wav::BitDepth::Sixteen(
            samples
                .iter()
                .map(|s| (i16::max_value() as f64 * s) as i16)
                .collect(),
        );
        let h = wav::Header::new(1, 1, sample_rate as u32, 16);
        let mut out: Vec<u8> = Vec::new();
        wav::write(h, &bd, &mut std::io::Cursor::new(&mut out)).unwrap();
        std::io::stdout().write_all(&out).unwrap();
    }
}
