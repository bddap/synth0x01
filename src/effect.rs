use crate::map;

// a transformation over samples
pub trait Effect {
    fn apply(&self, samples: &mut [f64], sample_rate: usize);
    fn end_hint(&self) -> Option<f64> {
        None
    }
}

pub trait EffectExt: Effect + Sized {
    fn envelope(self, start: f64, dur: f64, attack_end: f64, decay_start: f64) -> Envelope<Self> {
        Envelope::new(self, start, dur, attack_end, decay_start)
    }
}

impl<T: Effect> EffectExt for T {}

// a function from time to sample
pub trait PureEffect {
    fn sample_at(&self, time: f64) -> f64;
}

pub trait PureEffectExt: PureEffect + Sized {
    fn amp(self, freq: f64) -> Freq<Self> {
        Freq { effect: self, freq }
    }

    fn freq(self, freq: f64) -> Freq<Self> {
        Freq { effect: self, freq }
    }
}

impl<T: PureEffect> PureEffectExt for T {}

impl<T: PureEffect> Effect for T {
    fn apply(&self, samples: &mut [f64], sample_rate: usize) {
        for (i, sample) in samples.iter_mut().enumerate() {
            let t = i as f64 / sample_rate as f64;
            *sample += self.sample_at(t);
        }
    }
}

impl<F: Fn(f64) -> f64> PureEffect for F {
    fn sample_at(&self, time: f64) -> f64 {
        (self)(time)
    }
}

impl<T: Effect, const N: usize> Effect for [T; N] {
    fn apply(&self, samples: &mut [f64], sample_rate: usize) {
        for a in self {
            a.apply(samples, sample_rate);
        }
    }
}

#[derive(Clone)]
pub struct Freq<T> {
    effect: T,
    freq: f64,
}

impl<T: PureEffect> PureEffect for Freq<T> {
    fn sample_at(&self, time: f64) -> f64 {
        self.effect.sample_at(time * self.freq)
    }
}

#[derive(Clone)]
pub struct Amp<T> {
    effect: T,
    amp: f64,
}

impl<T: PureEffect> PureEffect for Amp<T> {
    fn sample_at(&self, time: f64) -> f64 {
        self.effect.sample_at(time) * self.amp
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
        Self {
            effect,
            start,
            dur,
            attack_end,
            decay_start,
        }
    }
}

impl<T: PureEffect> Effect for Envelope<T> {
    fn apply(&self, samples: &mut [f64], sample_rate: usize) {
        let start_idx = (sample_rate as f64 * self.start) as usize;
        let stop_idx = (sample_rate as f64 * (self.start + self.dur)) as usize;
        let len = samples.len();
        let effected = &mut samples[start_idx.min(len)..stop_idx.min(len)];
        (|t| {
            let attack_amp = map(t, 0.0, self.attack_end - self.start, 0.0, 1.0);
            let sustain_amp = map(t, self.decay_start - self.start, self.dur, 1.0, 0.0);
            let envelope_amp = attack_amp.min(sustain_amp).max(0.0).min(1.0);
            self.effect.sample_at(t) * envelope_amp
        })
        .apply(effected, sample_rate);
    }

    fn end_hint(&self) -> Option<f64> {
        Some(self.start + self.dur)
    }
}

pub fn harmonics<T: PureEffect + Clone>(effect: T) -> [impl Effect; 9] {
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
    fn apply(&self, samples: &mut [f64], _sample_rate: usize) {
        for s in samples {
            *s = s.tanh();
        }
    }
}

pub struct Integral;

impl Effect for Integral {
    fn apply(&self, samples: &mut [f64], _sample_rate: usize) {
        let mut integrated: f64 = 0.0;
        for sample in samples {
            integrated += *sample;
            *sample = integrated;
        }
    }
}

pub struct Derivative;

impl Effect for Derivative {
    fn apply(&self, samples: &mut [f64], _sample_rate: usize) {
        let mut prev = 0.0;
        for sample in samples {
            let next = *sample - prev;
            prev = *sample;
            *sample = next;
        }
    }
}

pub struct Volume(pub f64);

impl Effect for Volume {
    fn apply(&self, samples: &mut [f64], _sample_rate: usize) {
        for sample in samples {
            *sample *= self.0;
        }
    }
}
