pub fn solve(part: u32, lines: Vec<String>) -> u32 {
    if part == 1 {
        part1(lines)
    } else {
        part2(lines)
    }
}

fn part1(lines: Vec<String>) -> u32 {
    lines.iter().map(find_number).sum()
}

fn part2(lines: Vec<String>) -> u32 {
    5
}

fn find_number(line: &String) -> u32 {
    let first = line.chars().find(|x| x.is_digit(10)).unwrap();
    let last = line.chars().rev().find(|x| x.is_digit(10)).unwrap();

    let mut num = String::new();
    num.push(first);
    num.push(last);

    num.parse().unwrap()
}
