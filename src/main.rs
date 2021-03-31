mod note;
mod song;
mod util;

// TODO: Try transforming all samples in song by Hyperbolic Tangent
//       HT will bound signal to (-1,1) and may sound cool.
//       It may also allow for many notes to be played at the same time
//       without clipping.
//       Also try some veriation of sigmoid as it seems similar.
//       I'm pretty sure `sig(x * 2) * 2 - 1 = tanh(x)`. 

use crate::util::map;
use note::Note;
// use rand::Rng;
// use rand::SeedableRng;
use song::Song;

fn main() {
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

    // let mut rng = rand::rngs::SmallRng::from_seed([0xff; 32]);

    let section_len = 20.;
    let sections = 32;
    let songlen = section_len * sections as f64;
    let mut song = Song::new(songlen);

    for section in 0..sections {
        let attack_end = 0.01;
        let decay_start = 0.99;
        let base_freq = 261.0;

        let notes_in_octave = 12.; // sounds like an oxymoron :)

        // let fraxend: usize = (rng.gen::<usize>() % fraxss.len()).max(1);
        // let fraxstart: usize = (rng.gen::<usize>() % fraxend).min(fraxend - 1);
        let fraxend: usize = fraxss.len();
        let fraxstart: usize = 0;
        let fraxss = &fraxss[fraxstart..fraxend];

        let sectionstart = section as f64 * section_len;
        let dur = section_len / fraxss.len() as f64;
        let sectionend = (section + 1) as f64 * section_len - dur;
        for (i, fraxs) in fraxss.iter().enumerate() {
            let time = map(
                i as f64,
                0.0,
                fraxss.len() as f64 - 1.,
                sectionstart,
                sectionend - 0.00001,
            );
            dbg!(time / songlen);
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
    }

    song.dump();

    #[cfg(feature = "plotters")]
    util::plot(song.samples().iter().cloned().step_by(1));
}
