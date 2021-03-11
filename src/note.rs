pub struct Note {
    pub time: f64,
    pub dur: f64,
    pub attack_end: f64,  // in range (0.0, 1.0)
    pub decay_start: f64, // in range (0.0, 1.0)
    pub freq: f64,
    pub amp: f64, // in range (0.0, 1.0)
    pub timbre: fn(f64) -> f64,
}

pub fn saw(t: f64) -> f64 {
    (t + 1.0) % 2.0 - 1.0
}

pub fn sin(t: f64) -> f64 {
    (t * core::f64::consts::TAU).sin()
}
