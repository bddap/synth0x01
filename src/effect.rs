use crate::map;
use crate::util::fmax;
use crate::util::fmin;

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

pub trait EffectExt: Effect + Sized {
    fn freq(self, freq: f64) -> Freq<Self> {
        Freq { effect: self, freq }
    }

    fn envelope(self, start: f64, dur: f64, attack_end: f64, decay_start: f64) -> Envelope<Self> {
        Envelope::new(self, start, dur, attack_end, decay_start)
    }

    fn amp(self, amp: f64) -> Amp<Self> {
        Amp { effect: self, amp }
    }
}

impl<F: Fn(f64) -> f64> Effect for F {
    fn effect(&self, t: f64, sample: &mut f64) {
        *sample += (self)(t);
    }
}

impl<T: Effect> EffectExt for T {}

impl<T: Effect, const N: usize> Effect for [T; N] {
    fn effect(&self, t: f64, sample: &mut f64) {
        for e in self {
            e.effect(t, sample);
        }
    }

    // Optional start and stop of the effect.
    fn range_hint(&self) -> Option<(f64, f64)> {
        let ranges = self.iter().filter_map(|e| e.range_hint());
        let max = fmax(ranges.clone().map(|(_min, max)| max))?;
        let min = fmin(ranges.clone().map(|(min, _max)| min))?;
        Some((min, max))
    }
}

#[derive(Clone)]
pub struct Freq<T> {
    effect: T,
    freq: f64,
}

impl<T: Effect> Effect for Freq<T> {
    fn effect(&self, t: f64, sample: &mut f64) {
        self.effect.effect(t * self.freq, sample);
    }

    fn range_hint(&self) -> Option<(f64, f64)> {
        // range is not effected by shift, only pitch
        self.effect.range_hint()
    }
}

#[derive(Clone)]
pub struct Amp<T> {
    effect: T,
    amp: f64,
}

impl<T: Effect> Effect for Amp<T> {
    fn effect(&self, t: f64, sample: &mut f64) {
        let mut s2 = 0.0;
        self.effect.effect(t, &mut s2);
        *sample += s2 * self.amp;
    }

    fn range_hint(&self) -> Option<(f64, f64)> {
        self.effect.range_hint()
    }
}

#[derive(Clone)]
pub struct Envelope<T> {
    effect: T,
    start: f64,
    dur: f64,         // end = time + dur
    attack_end: f64,  // in range (start, start + dur)
    decay_start: f64, // in range (start, start + dur)
}

impl<T: Effect> Envelope<T> {
    fn new(effect: T, start: f64, dur: f64, attack_end: f64, decay_start: f64) -> Self {
        assert!(attack_end >= start);
        assert!(attack_end <= start + dur);
        assert!(decay_start >= start);
        assert!(decay_start <= start + dur);
        if let Some((min, max)) = effect.range_hint() {
            assert!(min <= start);
            assert!(max >= start + dur);
        }
        Self {
            effect,
            start,
            dur,
            attack_end,
            decay_start,
        }
    }
}

impl<T: Effect> Effect for Envelope<T> {
    fn effect(&self, t: f64, sample: &mut f64) {
        let attack_amp = map(t, self.start, self.attack_end, 0.0, 1.0);
        let sustain_amp = map(t, self.decay_start, self.start + self.dur, 1.0, 0.0);
        let envelope_amp = attack_amp.min(sustain_amp).max(0.0).min(1.0);
        let mut s2 = 0.0;
        self.effect.effect(t, &mut s2);
        *sample += s2 * envelope_amp;
    }

    fn range_hint(&self) -> Option<(f64, f64)> {
        let ret = if let Some((min, max)) = self.effect.range_hint() {
            (min.max(self.start), max.min(self.start + self.dur))
        } else {
            (self.start, self.start + self.dur)
        };
        Some(ret)
    }
}

pub fn harmonics<T: Effect + Clone>(effect: T) -> [impl Effect; 9] {
    [
        effect.clone().freq(1. / 16.),
        effect.clone().freq(1. / 8.),
        effect.clone().freq(1. / 4.),
        effect.clone().freq(1. / 2.),
        effect.clone().freq(1.),
        effect.clone().freq(1. * 2.),
        effect.clone().freq(1. * 4.),
        effect.clone().freq(1. * 8.),
        effect.clone().freq(1. * 16.),
    ]
}

// handy for preventing clipping
pub struct HyperbolicTangent;

impl Effect for HyperbolicTangent {
    fn effect(&self, _t: f64, sample: &mut f64) {
        *sample = sample.tanh();
    }

    fn range_hint(&self) -> Option<(f64, f64)> {
        None
    }
}
