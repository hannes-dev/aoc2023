pub fn solve(part: u32, lines: Vec<String>) -> u32 {
    if part == 1 {
        part1(lines)
    } else {
        part2(lines)
    }
}

fn part1(lines: Vec<String>) -> u32 {
    let times: Vec<usize> = parse_nums(&lines[0]);
    let distances: Vec<usize> = parse_nums(&lines[1]);

    let mut prod: usize = 1;
    for (i, time) in times.iter().enumerate() {
        prod *= (0..*time)
            .map(|x| x * (time - x))
            .filter(|x| x > &distances[i])
            .count();
    }

    prod as u32
}

fn part2(lines: Vec<String>) -> u32 {
    let time: usize = parse_as_one(&lines[0]);
    let distance: usize = parse_as_one(&lines[1]);

    (0..time)
        .map(|x| x * (time - x))
        .filter(|x| x > &distance)
        .count() as u32
}

fn parse_nums(line: &str) -> Vec<usize> {
    line[9..]
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn parse_as_one(line: &str) -> usize {
    line[9..]
        .chars()
        .filter(|x| !x.is_whitespace())
        .collect::<String>()
        .parse()
        .unwrap()
}
