pub fn solve(part: u32, lines: Vec<String>) -> u32 {
    if part == 1 {
        part1(lines)
    } else {
        part2(lines)
    }
}

fn part1(lines: Vec<String>) -> u32 {
    let mut history: Vec<Vec<isize>> = Vec::new();
    let mut sum = 0;
    for line in lines {
        update_history(line, &mut history);

        let mut prev_last = 0;
        for i in (0..history.len() - 1).rev() {
            prev_last = history[i].last().unwrap() + prev_last;
        }
        sum += prev_last;
    }

    sum as u32
}

fn part2(lines: Vec<String>) -> u32 {
    let mut history: Vec<Vec<isize>> = Vec::new();
    let mut sum = 0;
    for line in lines {
        update_history(line, &mut history);

        let mut prev_last = 0;
        for i in (0..history.len() - 1).rev() {
            prev_last = history[i][0] - prev_last;
        }
        sum += prev_last;
    }

    sum as u32

}

fn update_history(line: String, history: &mut Vec<Vec<isize>>) {
    history.clear();

    let seq: Vec<isize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
    history.push(seq);
    
    while history.last().unwrap().iter().filter(|x| **x != 0).count() > 0 {
        let next_seq: Vec<isize> = history.last().unwrap().windows(2).map(|val| val[1] - val[0]).collect();
        history.push(next_seq);
    }
}