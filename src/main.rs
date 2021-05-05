mod effect;
mod note;
mod song;
mod timbre;
mod util;

use crate::effect::EffectExt;
use effect::HyperbolicTangent;
use rand::rngs::SmallRng;
use rand::Rng;
use rand::SeedableRng;
use song::Song;
use util::map;

fn char_to_fra(c: char) -> usize {
    (c as u32 as usize) % 24
}

fn to_chord(word: &str) -> Vec<usize> {
    word.chars().map(char_to_fra).collect()
}

fn to_chords(words: &str) -> Vec<Vec<usize>> {
    words.split('\n').map(to_chord).collect()
}

fn main() {
    let seed: usize = 2;
    let mut rg = SmallRng::seed_from_u64(seed as u64);

    let seedword = "hello there my friend boy ol pal";
    let ofx: Vec<Vec<usize>> = to_chords(seedword);

    let fraxss: Vec<Vec<usize>> = (0..1024)
        .map(|i| {
            ofx.iter()
                .map(|chord| chord[i % chord.len()])
                .filter(|_| rg.gen::<f32>() < 0.5)
                .collect()
        })
        .collect();

    let dur = 1. / 4.;
    let base_freq = 261.0;
    let notes_in_octave = 12.;
    let mut song = Song::default();

    for (i, chord) in fraxss.iter().cycle().take(1024).enumerate() {
        let time = i as f64 * dur;

        for c in chord.iter() {
            let freq = base_freq * (notes_in_octave / *c as f64) / 2.;
            let nowet = timbre::cral
                .freq(freq)
                .amp(map(*c as f64, 23.0, 0., 0.5, 0.1))
                .envelope(time, dur, time + dur / 2., time + dur / 2.);
            song.add_effect(nowet);
        }
    }

    song.add_effect(HyperbolicTangent);
    song.dump();

    #[cfg(feature = "plotters")]
    util::plot(song.samples().iter().cloned().step_by(1));
}
