use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let days: Vec<_> = match args.len() {
        1 => (1..=25).collect(),
        _ => args.iter().skip(1).map(|d| d.parse().unwrap()).collect(),
    };
    for day in days {
        println!("Day {}:", day);
        let path = format!("./data/day{:02}.txt", day);
        let input = fs::read_to_string(&path);
        if let Ok(input) = input {
            let input = input.trim_end();
            let day_func = match day {
                1 => aoc2019::day01::run,
                2 => aoc2019::day02::run,
                3 => aoc2019::day03::run,
                4 => aoc2019::day04::run,
                5 => aoc2019::day05::run,
                6 => aoc2019::day06::run,
                7 => aoc2019::day07::run,
                8 => aoc2019::day08::run,
                9 => aoc2019::day09::run,
                10 => aoc2019::day10::run,
                11 => aoc2019::day11::run,
                12 => aoc2019::day12::run,
                13 => aoc2019::day13::run,
                14 => aoc2019::day14::run,
                15 => aoc2019::day15::run,
                16 => aoc2019::day16::run,
                17 => aoc2019::day17::run,
                18 => aoc2019::day18::run,
                19 => aoc2019::day19::run,
                20 => aoc2019::day20::run,
                21 => aoc2019::day21::run,
                22 => aoc2019::day22::run,
                23 => aoc2019::day23::run,
                24 => aoc2019::day24::run,
                25 => aoc2019::day25::run,
                _ => unreachable!(),
            };
            day_func(input);
        } else {
            println!("ERROR: no data");
        }
    }
}