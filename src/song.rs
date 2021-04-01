use std::io::Write;

pub trait Effect {
    fn effect(&self, t: f64, sample: &mut f64);

    // Optional start and stop of the effect.
    fn range_hint(&self) -> Option<(f64, f64)> {
        None
    }

    // apply self to song
    fn effects(&self, samples: &mut [f64], sample_rate: usize) {
        let (start, end) = self
            .range_hint()
            .unwrap_or((0.0, samples.len() as f64 / sample_rate as f64));
        let sample_start = (sample_rate as f64 * start) as usize;
        let sample_end = (sample_rate as f64 * end) as usize;
        for i in sample_start..sample_end {
            let t = i as f64 / sample_rate as f64;
            self.effect(t, &mut samples[i]);
        }
    }
}

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
        let ranges = self.transforms.iter().filter_map(|tr| tr.range_hint());
        let duration = fmax(ranges.map(|r| r.1)).expect("can't determine song length");
        let sample_rate = 44_100; // cd quality
        let num_samples = (duration * sample_rate as f64) as usize;
        let mut samples = vec![0.0; num_samples];
        for transform in &self.transforms {
            let (start, end) = transform.range_hint().unwrap_or((0.0, duration));
            let sample_start = (sample_rate as f64 * start) as usize;
            let sample_end = (sample_rate as f64 * end) as usize;
            for i in sample_start..sample_end {
                let t = i as f64 / sample_rate as f64;
                transform.effect(t, &mut samples[i]);
            }
        }
        let bd = wav::BitDepth::Sixteen(
            samples
                .iter()
                .map(|s| s.tanh()) // reduces unintentional clipping
                .map(|s| (i16::max_value() as f64 * s) as i16)
                .collect(),
        );
        let h = wav::Header::new(1, 1, sample_rate as u32, 16);
        let mut out: Vec<u8> = Vec::new();
        wav::write(h, &bd, &mut std::io::Cursor::new(&mut out)).unwrap();
        std::io::stdout().write_all(&out).unwrap();
    }
}

fn fmax(mut fs: impl Iterator<Item = f64>) -> Option<f64> {
    let mut ret = fs.next()?;
    for f in fs {
        if ret < f {
            ret = f;
        }
    }
    Some(ret)
}
