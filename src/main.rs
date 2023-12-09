mod days;
use std::error::Error;
use std::path::Path;
use std::{env, fs::File, io::BufRead, io::BufReader};

use days::{day01, day02, day03, day04, day05, day06, day08, day09};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let day: u32 = args[1].parse()?;
    let part: u32 = args[2].parse()?;

    let input = if args.len() == 4 {
        lines(&Path::new(&args[3]))
    } else {
        lines(&Path::new(&format!("./input/{day}_test.txt")))
    };

    let result = get_result(day, part, input);
    println!("Result for day {day}:{part}");
    println!("(☞ﾟヮﾟ)☞ {result}");

    Ok(())
}

fn get_result(day: u32, part: u32, lines: Vec<String>) -> u32 {
    match day {
        1 => day01::solve(part, lines),
        2 => day02::solve(part, lines),
        3 => day03::solve(part, lines),
        4 => day04::solve(part, lines),
        5 => day05::solve(part, lines),
        6 => day06::solve(part, lines),
        8 => day08::solve(part, lines),
        9 => day09::solve(part, lines),
        _ => todo!(),
    }
}

fn lines(file_name: &Path) -> Vec<String> {
    dbg!(file_name);
    let reader = BufReader::new(File::open(file_name).expect("file"));
    reader.lines().map(Result::unwrap).collect()
}
