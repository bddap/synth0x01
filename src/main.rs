mod note;
mod song;
mod util;

use crate::util::map;
use note::Note;
use song::Song;

fn main() {
    let fraxss: Vec<Vec<usize>> = (3..8usize).map(|i| vec![i.pow(2)]).collect();

    let dur = 1. / 4.;
    let base_freq = 261.0;
    let notes_in_octave = 12.;
    let mut song = Song::default();

    for (i, chord) in fraxss.iter().cycle().take(1024).enumerate() {
        let time = i as f64 * dur;

        for c in chord.iter() {
            let freq = base_freq * (notes_in_octave / *c as f64);
            let note = Note {
                time,
                dur,
                attack_end: time + dur / 2.,
                decay_start: time + dur / 2.,
                freq,
                amp: 0.2,
                timbre: note::cral,
            };
            song.add_effect(note);
        }
    }
    
    song.dump();

    #[cfg(feature = "plotters")]
    util::plot(song.samples().iter().cloned().step_by(1));
}
