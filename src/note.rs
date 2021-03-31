pub struct Note<T> {
    pub time: f64,
    pub dur: f64,
    pub attack_end: f64,  // in range (0.0, 1.0)
    pub decay_start: f64, // in range (0.0, 1.0)
    pub freq: f64,
    pub amp: f64, // in range (0.0, 1.0)
    pub timbre: T,
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
