mod common;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;

use std::fs;
use std::time::Instant;

const DAY_1: bool = false;
const DAY_2: bool = false;
const DAY_3: bool = false;
const DAY_4: bool = false;
const DAY_5: bool = false;
const DAY_6: bool = false;
const DAY_7: bool = false;
const DAY_8: bool = true;

fn main() {
    let mut now = Instant::now();

    if DAY_1 {
        now = Instant::now();
        let sol_1_1 = crate::day_1::part_1(fs::File::open("./data/day_1.txt").unwrap());
        let sol_1_2 = crate::day_1::part_2(fs::File::open("./data/day_1.txt").unwrap());
        println!("Day One: Part1 = {}; Part 2 = {}; Took {:.2}ms;", sol_1_1, sol_1_2, now.elapsed().as_secs_f32() * 1000.0f32);
    }

    if DAY_2 {
        now = Instant::now();
        let sol_2_1 = crate::day_2::part_1(fs::File::open("./data/day_2.txt").unwrap());
        let sol_2_2 = crate::day_2::part_2(fs::File::open("./data/day_2.txt").unwrap());
        println!("Day Two: Part1 = {}; Part 2 = {}; Took {:.2}ms;", sol_2_1, sol_2_2, now.elapsed().as_secs_f32() * 1000.0f32);
    }

    if DAY_3 {
        now = Instant::now();
        let sol_3_1 = crate::day_3::part_1(fs::File::open("./data/day_3.txt").unwrap());
        let sol_3_2 = crate::day_3::part_2(fs::File::open("./data/day_3.txt").unwrap());
        println!("Day Three: Part1 = {}; Part 2 = {}; Took {:.2}ms;", sol_3_1, sol_3_2, now.elapsed().as_secs_f32() * 1000.0f32);
    }

    if DAY_4 {
        now = Instant::now();
        let sol_4_1 = crate::day_4::part_1(fs::File::open("./data/day_4.txt").unwrap());
        let sol_4_2 = crate::day_4::part_2(fs::File::open("./data/day_4.txt").unwrap());
        println!("Day Four: Part1 = {}; Part 2 = {}; Took {:.2}ms;", sol_4_1, sol_4_2, now.elapsed().as_secs_f32() * 1000.0f32);
    }

    if DAY_5 {
        now = Instant::now();
        let sol_5_1 = crate::day_5::part_1(fs::File::open("./data/day_5.txt").unwrap());
        let sol_5_2 = crate::day_5::part_2(fs::File::open("./data/day_5.txt").unwrap());
        println!("Day Five: Part1 = {}; Part 2 = {}; Took {:.2}ms;", sol_5_1, sol_5_2, now.elapsed().as_secs_f32() * 1000.0f32);
    }

    if DAY_6 {
        now = Instant::now();
        let sol_6_1 = crate::day_6::part_1(fs::File::open("./data/day_6.txt").unwrap());
        let sol_6_2 = crate::day_6::part_2(fs::File::open("./data/day_6.txt").unwrap());
        println!("Day Six: Part1 = {}; Part 2 = {}; Took {:.2}ms;", sol_6_1, sol_6_2, now.elapsed().as_secs_f32() * 1000.0f32);
    }

    if DAY_7 {
        now = Instant::now();
        let sol_7_1 = crate::day_7::part_1(fs::File::open("./data/day_7.txt").unwrap());
        let sol_7_2 = crate::day_7::part_2(fs::File::open("./data/day_7.txt").unwrap());
        println!("Day Seven: Part1 = {}; Part 2 = {}; Took {:.2}ms;", sol_7_1, sol_7_2, now.elapsed().as_secs_f32() * 1000.0f32);
    }

    if DAY_8 {
        now = Instant::now();
        let sol_8_1 = crate::day_8::part_1(fs::File::open("./data/day_8.txt").unwrap());
        let sol_8_2 = crate::day_8::part_2(fs::File::open("./data/day_8.txt").unwrap());
        println!("Day Eight: Part1 = {}; Part 2 = {}; Took {:.2}ms;", sol_8_1, sol_8_2, now.elapsed().as_secs_f32() * 1000.0f32);
    }
}
