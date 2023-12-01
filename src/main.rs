mod days;
use std::{env, fs::File, io::BufReader, io::BufRead};
use std::error::Error;

use days::day01;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let day: u32 = args[1].parse()?;
    let part: u32 = args[2].parse()?;

    let test = lines(format!("{day}_test.txt"));
    let full = lines(format!("{day}.txt"));

    let test_result = get_result(day, part, test);
    let full_result = get_result(day, part, full);

    println!("Result for day {day}:{part}");
    println!("  Test = {test_result}");
    println!("  Full = {full_result}");

    Ok(())
}

fn get_result(day: u32, part: u32, lines: Vec<String>) -> u32 {
    match day {
        1 => day01::solve(part, lines),
         _ => todo!()
     }
}

fn lines(file_name: String) -> Vec<String> {
    let reader = BufReader::new(File::open(format!("src/inputs/{file_name}")).expect("file"));
    reader.lines().map(Result::unwrap).collect()
}
