use std::collections::HashMap;

pub fn solve(part: u32, lines: Vec<String>) -> u32 {
    if part == 1 {
        part1(lines)
    } else {
        part2(lines)
    }
}

fn part1(lines: Vec<String>) -> u32 {
    lines.iter().map(find_number_p1).sum()
}

fn part2(lines: Vec<String>) -> u32 {
    lines.iter().map(find_number_or_text).sum()
}

fn find_number_or_text(line: &String) -> u32 {
    let numbers = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let words: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let first = numbers
        .iter()
        .chain(words.keys())
        .filter_map(|num| line.find(num).map(|idx| (idx, num.len())))
        .min_by_key(|&(idx, _)| idx)
        .unwrap();
    let first = &line[(first.0)..(first.0 + first.1)];

    let last = numbers
        .iter()
        .chain(words.keys())
        .filter_map(|num| line.rfind(num).map(|idx| (idx, num.len())))
        .max_by_key(|&(idx, _)| idx)
        .unwrap();
    let last = &line[(last.0)..(last.0 + last.1)];
    let first = words.get(first).unwrap_or(&first);
    let last = words.get(last).unwrap_or(&last);

    ((*first).to_owned() + last).parse().unwrap()
}

fn find_number_p1(line: &String) -> u32 {
    let first = line.chars().find(|x| x.is_digit(10)).unwrap();
    let last = line.chars().rev().find(|x| x.is_digit(10)).unwrap();

    let mut num = String::new();
    num.push(first);
    num.push(last);

    num.parse().unwrap()
}
