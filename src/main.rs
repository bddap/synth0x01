mod note;
mod song;
mod util;

use crate::util::map;
use note::Note;
use song::Song;

fn main() {
    let songlen = 40.;
    let mut song = Song::new(songlen);

    let attack_end = 0.01;
    let decay_start = 0.99;
    let base_freq = 261.0;

    let notes_in_octave = 12.; // sounds like an oxymoron :)

    let a = &[12, 10, 20];
    let b = &[6, 12, 20];
    let c = &[6, 8, 10];
    let d = &[4, 8, 14];
    let e = &[5, 8, 15];
    let fraxss: &[&[usize]] = &[
        a,
        b,
        c,
        d,
        e,
        b,
        b,
        d,
        e,
        b,
        b,
        &[3, 4, 5],
        &[2, 3, 4, 5],
        &[],
        &[2, 3, 4, 5],
        &[3, 4, 5],
        &[3, 4, 5, 6],
        b,
        e,
        d,
        e,
        b,
        b,
        d,
        e,
        b,
        b,
        e,
        b,
        a,
        d,
        e,
        b,
        b,
        d,
        e,
        b,
        b,
        &[2, 3, 4],
        &[20, 30, 40],
        &[20 / 2, 30 / 2, 40 / 2],
        &[20 / 3, 30 / 4, 40 / 5],
        &[20 / 4, 30 / 5, 40 / 6],
        &[20 / 5, 30 / 7, 10],
    ];

    let dur = songlen / fraxss.len() as f64;
    for (i, fraxs) in fraxss.iter().enumerate() {
        let time = map(i as f64, 0.0, fraxss.len() as f64, 0.0, songlen - 0.1 - dur);
        for frac in *fraxs {
            let note = Note {
                time,
                dur,
                attack_end,
                decay_start,
                freq: base_freq * (notes_in_octave / *frac as f64),
                amp: 0.2,
                timbre: note::cray(note::sin, |s, t| {
                    let base = ((t / 8.).sin() + 2.) * 2.;
                    let a = s.abs().powf(base);
                    a * s.signum()
                }),
            };
            song.add_note(note);
        }
    }

    song.dump();

    #[cfg(feature = "plotters")]
    util::plot(song.samples().iter().cloned().step_by(1));
}
