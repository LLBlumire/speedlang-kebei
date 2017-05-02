extern crate rand;

use rand::Rng;
use rand::thread_rng;

const CONS: &'static [&'static str] = &[
    "m", "n", "n.", "q",
    "b", "d", "d.", "g", "'",
    "p", "t", "t.", "k",
    "f", "s", "s.", "x", "h"
];

const VOWEL: &'static [&'static str] = &[
    "i", "u",
    "e", "o",
    "ei", "oi",
    "eu", "ou",
];

const FRIC: &'static [&'static str] = &[
    "f", "s", "s.", "x", "h"
];

fn main() {
    let mut rng = thread_rng();

    for _ in 0..30 {
        for n in 0.. {
            print!("{}", rng.choose(CONS).unwrap());
            print!("{}", rng.choose(VOWEL).unwrap());
            if rng.gen_weighted_bool(4) {
                print!("{}", rng.choose(FRIC).unwrap());
            }
            if rng.gen_weighted_bool(2-n) {
                break;
            }
        }
        println!();
    }
}
