mod note;
mod song;
mod util;

use crate::util::map;
use note::Note;
use rand::seq::SliceRandom;
use rand::Rng;
use rand::SeedableRng;
use song::Song;

fn main() {
    let songlen = 60. * 60.;
    let mut rng = rand::rngs::SmallRng::from_seed([0xff; 32]);
    let mut song = Song::new(songlen);

    let some_small_integers = [2.0, 2.0, 3.0, 3.0, 5.0, 7.0, 11.0];

    let dur = 0.1;
    let attack_end = 0.1;
    let decay_start = 0.9;
    let base_freq = 261.0;
    let spacing = 4.0;

    for _ in 0..songlen as usize * 10 {
        let note = Note {
            time: (map(rng.gen(), 0.0, 1.0, 0.0, songlen - dur - 1.0) * spacing).round() / spacing,
            dur,
            attack_end,
            decay_start,
            freq: base_freq / some_small_integers.choose(&mut rng).unwrap()
                * some_small_integers.choose(&mut rng).unwrap(),
            amp: 0.1,
            timbre: note::sin,
        };
        song.add_note(note);
    }

    song.dump();

    #[cfg(feature = "plotters")]
    util::plot(song.samples().iter().cloned().step_by(1));
}
