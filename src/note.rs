use crate::map;
use crate::song::Effect;
use crate::util::lin;

pub struct Note<T> {
    pub time: f64,
    pub dur: f64,
    pub attack_end: f64,  // in range (0.0, 1.0)
    pub decay_start: f64, // in range (0.0, 1.0)
    pub freq: f64,
    pub amp: f64, // in range (0.0, 1.0)
    pub timbre: T,
}

impl<T: Fn(f64) -> f64> Effect for Note<T> {
    fn effect(&self, t: f64, sample: &mut f64) {
        let attack_amp = map(t, self.time, self.attack_end, 0.0, 1.0);
        let sustain_amp = map(t, self.decay_start, self.time + self.dur, 1.0, 0.0);
        let envelope_amp = attack_amp.min(sustain_amp).max(0.0).min(1.0);
        *sample += (self.timbre)(t * self.freq) * envelope_amp * self.amp;
    }

    fn range_hint(&self) -> Option<(f64, f64)> {
        Some((self.time, self.time + self.dur))
    }
}

pub fn saw(t: f64) -> f64 {
    (t + 1.0) % 2.0 - 1.0
}

pub fn sin(t: f64) -> f64 {
    (t * core::f64::consts::TAU).sin()
}

pub fn cwav(f: impl Fn(f64) -> f64) -> impl Fn(f64) -> f64 {
    move |t| {
        let s = f(t);
        let a = s.abs().sqrt();
        a * s.signum()
    }
}

pub fn swav(f: impl Fn(f64) -> f64) -> impl Fn(f64) -> f64 {
    move |t| {
        let s = f(t);
        let a = s.abs().exp2();
        a * s.signum()
    }
}

pub fn modwav(f: impl Fn(f64) -> f64, modr: impl Fn(f64) -> f64) -> impl Fn(f64) -> f64 {
    move |t| modr(f(t))
}

pub fn cray(f: impl Fn(f64) -> f64, modr: impl Fn(f64, f64) -> f64) -> impl Fn(f64) -> f64 {
    move |t| modr(f(t), t)
}

pub fn cral(t: f64) -> f64 {
    let s = sin(t);
    let base = ((t / 8.).sin() + 2.) * 2.;
    let a = s.abs().powf(base);
    a * s.signum()
}
