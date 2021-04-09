use crate::effect::{Effect, EffectExt};

pub fn note(
    time: f64,
    dur: f64,
    attack_end: f64,  // in range (0.0, 1.0)
    decay_start: f64, // in range (0.0, 1.0)
    freq: f64,
    amp: f64, // in range (0.0, 1.0)
    timbre: impl Effect,
) -> impl Effect {
    timbre
        .freq(freq)
        .amp(amp)
        .envelope(time, dur, attack_end, decay_start)
}
