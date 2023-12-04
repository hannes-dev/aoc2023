use std::collections::HashMap;

pub fn solve(part: u32, lines: Vec<String>) -> u32 {
    if part == 1 {
        part1(lines)
    } else {
        part2(lines)
    }
}

fn part1(lines: Vec<String>) -> u32 {
    let cards = parse(lines);
    let mut sum = 0;
    for (_, winning_amt) in cards {
        if winning_amt > 0 {
            sum += 2_u32.pow((winning_amt - 1) as u32);
        }
    }

    sum
}

fn part2(lines: Vec<String>) -> u32 {
    let cards = parse(lines);
    let mut stack: Vec<usize> = cards.keys().copied().collect();

    let mut sum = 0;
    while let Some(next) = stack.pop() {
        sum += 1;
        let winnings = cards[&next];
        stack.extend((next+1)..(next + 1 + winnings));
    }

    sum
}

fn parse(lines: Vec<String>) -> HashMap<usize, usize> {
    let mut winnings = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        let mut parsed = line.split(": ").last().unwrap().split(" | ").map(|x| {
            x.split(" ")
                .filter_map(|num| num.parse::<usize>().ok())
                .collect::<Vec<usize>>()
        });
        let winning = parsed.next().unwrap();
        let have = parsed.next().unwrap();

        let mut matches = 0;
        for nr in have {
            if winning.contains(&nr) {
                matches += 1;
            }
        }
        winnings.insert(i+1, matches);
    }

    winnings
}