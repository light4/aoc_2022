use anyhow::Result;
use aoc_2022::*;
use chrono::prelude::*;

fn main() -> Result<()> {
    let utc: DateTime<Utc> = Utc::now();
    let day = if let Some(s) = std::env::args().nth(1) {
        s.parse().unwrap_or_else(|_| utc.day())
    } else {
        utc.day()
    };

    let mod_name = format!("day{day:02}");
    match mod_name.as_str() {
        "day01" => day01::run(),
        "day02" => day02::run(),
        "day03" => day03::run(),
        "day04" => day04::run(),
        "day05" => day05::run(),
        "day06" => day06::run(),
        "day07" => day07::run(),
        "day08" => day08::run(),
        "day09" => day09::run(),
        "day10" => day10::run(),
        "day11" => day11::run(),
        "day12" => day12::run(),
        "day13" => day13::run(),
        "day14" => day14::run(),
        "day15" => day15::run(),
        "day16" => day16::run(),
        "day17" => day17::run(),
        "day18" => day18::run(),
        "day19" => day19::run(),
        "day20" => day20::run(),
        "day21" => day21::run(),
        "day22" => day22::run(),
        "day23" => day23::run(),
        "day24" => day24::run(),
        "day25" => day25::run(),
        "day26" => day26::run(),
        "day27" => day27::run(),
        "day28" => day28::run(),
        "day29" => day29::run(),
        "day30" => day30::run(),
        "day31" => day31::run(),
        _ => unreachable!(),
    };

    Ok(())
}
