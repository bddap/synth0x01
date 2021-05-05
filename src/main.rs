mod effect;
mod note;
mod song;
mod timbre;
mod util;

use crate::effect::EffectExt;
use effect::HyperbolicTangent;
use rand::random;
use rand::rngs::SmallRng;
use rand::Rng;
use rand::SeedableRng;
use song::Song;
use util::map;

fn main() {
    // let seed: usize = random();
    let seed: usize = 1;
    let mut rg = SmallRng::seed_from_u64(seed as u64);
    // let fraxss: Vec<Vec<usize>> = [rg.gen::<usize>() % 5 + 1]
    //     .iter()
    //     .cycle()
    //     .take(1)
    //     .map(|i| (0..=*i).map(|_| rg.gen::<usize>() % 12 + 1).collect())
    //     .collect();
    // dbg!(seed);
    // dbg!(&fraxss);

    let ofx = vec![
        vec![1, 3, 2],
        vec![6, 11],
        vec![6, 5, 4, 1],
        vec![5, 3, 8, 12, 10],
        vec![9, 6, 8],
    ];
    let fraxss: Vec<Vec<usize>> = (0..1024)
        .map(|i| {
            ofx.iter()
                .map(|chord| chord[i % chord.len()])
                .filter(|_| rg.gen::<f32>() < 0.3)
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
            let nowet = timbre::cral.freq(freq).amp(0.1).envelope(
                time,
                dur,
                time + dur / 2.,
                time + dur / 2.,
            );
            song.add_effect(nowet);
        }
    }

    song.add_effect(HyperbolicTangent);
    song.dump();

    #[cfg(feature = "plotters")]
    util::plot(song.samples().iter().cloned().step_by(1));
}
