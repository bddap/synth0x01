mod effect;
mod note;
mod song;
mod timbre;
mod util;

use crate::effect::EffectExt;
use effect::HyperBolicTangent;
use song::Song;
use util::map;

fn main() {
    let fraxss: Vec<Vec<usize>> = (3..8usize).map(|i| vec![i.pow(2)]).collect();

    let dur = 1. / 4.;
    let base_freq = 261.0;
    let notes_in_octave = 12.;
    let mut song = Song::default();

    for (i, chord) in fraxss.iter().cycle().take(1024).enumerate() {
        let time = i as f64 * dur;

        for c in chord.iter() {
            let freq = base_freq * (notes_in_octave / *c as f64) / 2.;
            let nowet = effect::harmonics(timbre::sin.freq(freq).amp(0.1)).envelope(
                time,
                dur,
                time + dur / 2.,
                time + dur / 2.,
            );
            song.add_effect(nowet);
        }
    }

    song.add_effect(HyperBolicTangent);
    song.dump();

    #[cfg(feature = "plotters")]
    util::plot(song.samples().iter().cloned().step_by(1));
}
