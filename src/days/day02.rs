pub fn solve(part: u32, lines: Vec<String>) -> u32 {
    if part == 1 {
        part1(lines)
    } else {
        part2(lines)
    }
}

fn part1(lines: Vec<String>) -> u32 {
    let mut sum = 0;
    for line in lines {
        let line: Vec<&str> = line.split(": ").collect();
        let nr: Vec<&str> = line[0].split(" ").collect();
        let game_nr: u32 = nr[1].parse().unwrap();
        let sets = line[1]
            .split("; ")
            .map(|set| set.split(", "))
            .all(|mut set| set.all(check_if_valid));

        if sets {
            println!("game nr {game_nr} is possible");
            sum += game_nr;
        }
    }

    sum
}

fn check_if_valid(item: &str) -> bool {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let amt_colour: Vec<&str> = item.split(" ").collect();
    let amount: u32 = amt_colour[0].parse().unwrap();
    match amt_colour[1] {
        "red" => amount <= max_red,
        "green" => amount <= max_green,
        "blue" => amount <= max_blue,
        x => {
            dbg!(x);
            false
        }
    }
}

fn part2(lines: Vec<String>) -> u32 {
    let mut sum = 0;
    for line in lines {
        let line: Vec<&str> = line.split(": ").collect();
        let sets: Vec<_> = line[1]
            .split("; ")
            .map(|set| set.split(", "))
            .map(|set| set.map(parse_count).collect::<Vec<_>>())
            .collect();

        let mut max = (0, 0, 0);

        for set in sets {
            for cubes in set {
                match cubes.1 {
                    "red" => {
                        if cubes.0 > max.0 {
                            max.0 = cubes.0
                        }
                    }
                    "green" => {
                        if cubes.0 > max.1 {
                            max.1 = cubes.0
                        }
                    }
                    "blue" => {
                        if cubes.0 > max.2 {
                            max.2 = cubes.0
                        }
                    }
                    x => {
                        dbg!(x);
                    }
                }
            }
        }
        sum += max.0 * max.1 * max.2;
    }

    sum
}

fn parse_count(item: &str) -> (u32, &str) {
    let amt_colour: Vec<&str> = item.split(" ").collect();
    let amount: u32 = amt_colour[0].parse().unwrap();

    (amount, amt_colour[1])
}
