use std::collections::{HashMap, HashSet};

pub fn solve(part: u32, lines: Vec<String>) -> u32 {
    if part == 1 {
        part1(lines)
    } else {
        part2(lines)
    }
}

const ADJ: [(isize, isize); 9] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 0),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn part1(lines: Vec<String>) -> u32 {
    let (symbol_locations, number_locations) = parse(lines);

    let mut sum = 0;
    'number: for number in number_locations {
        for letter_index in number.1 {
            for tf in ADJ {
                if symbol_locations.contains_key(&(number.0 + tf.0, letter_index + tf.1)) {
                    sum += number.2;
                    continue 'number;
                }
            }
        }
    }

    sum
}

fn part2(lines: Vec<String>) -> u32 {
    let (symbol_locations, number_locations) = parse(lines);

    let mut ajdacent_numbers = HashSet::new();
    let mut sum = 0;
    for gear in symbol_locations
        .iter()
        .filter(|(_, symbol)| **symbol == '*')
    {
        let positions = positions(gear.0 .0, gear.0 .1);
        for number in number_locations.iter() {
            for position in positions {
                if position.0 == number.0 && number.1.contains(&position.1) {
                    ajdacent_numbers.insert(number);
                }
            }
        }
        if ajdacent_numbers.len() == 2 {
            sum += ajdacent_numbers.iter().map(|x| x.2).product::<u32>();
        }
        ajdacent_numbers.clear();
    }

    sum
}

fn positions(x: isize, y: isize) -> [(isize, isize); 9] {
    let mut positions: [(isize, isize); 9] = [(0, 0); 9];
    for (index, tf) in ADJ.iter().enumerate() {
        positions[index] = (tf.0 + x, tf.1 + y);
    }

    positions
}

fn parse(lines: Vec<String>) -> (HashMap<(isize, isize), char>, Vec<(isize, Vec<isize>, u32)>) {
    let mut symbol_locations = HashMap::new();
    let mut number_locations = Vec::new();
    for (line_index, line) in lines.iter().enumerate() {
        let mut number = String::new();
        let mut number_indexes: Vec<isize> = Vec::new();

        for (letter_index, letter) in line.char_indices() {
            if letter.is_digit(10) {
                number.push(letter);
                number_indexes.push(letter_index as isize);
                continue;
            } else if !number.is_empty() {
                let parsed_number = number.parse().unwrap();
                number_locations.push((line_index as isize, number_indexes.clone(), parsed_number));
                number_indexes.clear();
                number.clear();
            }

            if !(letter == '.') {
                symbol_locations.insert((line_index as isize, letter_index as isize), letter);
            }
        }
        if !number.is_empty() {
            let parsed_number = number.parse().unwrap();
            number_locations.push((line_index as isize, number_indexes.clone(), parsed_number));
            number_indexes.clear();
            number.clear();
        }
    }

    (symbol_locations, number_locations)
}
