use seq_macro::seq;

mod common;
seq!(N in 1..=15 {
   mod day_~N;
});

use std::fs;
use std::time::Instant;

const fn log_day(day: usize) -> bool {
    return day == 15;
}

fn main() {
    seq!(N in 1..=15 {
        if log_day(N) {
            let now = Instant::now();
            let sol_1 = crate::day_~N::part_1(fs::File::open(concat!("./data/day_", stringify!(N), ".txt")).unwrap());
            let sol_2 = crate::day_~N::part_2(fs::File::open(concat!("./data/day_", stringify!(N), ".txt")).unwrap());
            println!(
                concat!("Day ", stringify!(N), ": Part 1 = {}; Part 2 = {}; Took {:.2}ms;"),
                sol_1, sol_2, now.elapsed().as_secs_f32() * 1000.0f32
            );
        }
    });
}
