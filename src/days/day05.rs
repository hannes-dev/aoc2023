pub fn solve(part: u32, lines: Vec<String>) -> u32 {
    if part == 1 {
        part1(lines)
    } else {
        part2(lines)
    }
}

const VEC: Vec<MapRange> = Vec::new();
struct MapRange {
    source_start: usize,
    destination_start: usize,
    amount: usize,
}

impl MapRange {
    fn source_end(&self) -> usize {
        self.source_start + self.amount - 1
    }

    fn map(&self, o: usize) -> usize {
        self.destination_start + (o - self.source_start)
    }
}

#[derive(Debug)]
struct SeedRange {
    start: usize,
    amount: usize,
}

impl SeedRange {
    fn end(&self) -> usize {
        self.start + self.amount - 1
    }
}

fn part1(lines: Vec<String>) -> u32 {
    let (seeds, maps) = parse(lines);

    let mut destinations: Vec<usize> = Vec::new();
    for mut seed in seeds {
        'map: for map in &maps {
            for range in map {
                if seed >= range.source_start && seed < range.source_start + range.amount {
                    seed = range.destination_start + (seed - range.source_start);
                    continue 'map;
                }
            }
        }
        destinations.push(seed);
    }
    
    *destinations.iter().min().unwrap() as u32
}

fn part2(lines: Vec<String>) -> u32 {
    let (seeds, maps) = parse(lines);

    let mut seed_ranges = Vec::new();
    for ch in seeds.chunks(2) {
        seed_ranges.push(SeedRange {
            start: ch[0],
            amount: ch[1],
        })
    }

    let mut mapped_seeds = Vec::new();
    for seed in seed_ranges {
        mapped_seeds.extend(map_seeds(&maps, 0, seed));
    }
    
    mapped_seeds.iter().map(|seed| seed.start).min().unwrap() as u32
}

fn map_seeds(maps: &[Vec<MapRange>; 7], next_map: usize, seed: SeedRange) -> Vec<SeedRange> {
    if next_map == maps.len() {
        return vec![seed];
    }

    let map = &maps[next_map];
    let mut mapped_seeds = Vec::new();
    for range in map {
        if seed.start < range.source_start { // S__
            let first = SeedRange { start: seed.start, amount: range.source_start - seed.start};
            if seed.end() > range.source_end() { // S__R__R__S
                let middle = SeedRange { start: range.map(range.source_start), amount: range.amount};
                let last = SeedRange { start: range.source_end() + 1, amount: seed.end() - range.source_end()};

                mapped_seeds.extend(map_seeds(maps, next_map, first));
                mapped_seeds.extend(map_seeds(maps, next_map + 1, middle));
                mapped_seeds.extend(map_seeds(maps, next_map, last));
                return mapped_seeds;

            } else if seed.end() > range.source_start { // S__R__S--R
                let last = SeedRange { start: range.destination_start, amount: seed.end() - range.source_start + 1};

                mapped_seeds.extend(map_seeds(maps, next_map, first));
                mapped_seeds.extend(map_seeds(maps, next_map + 1, last));
                return mapped_seeds;
            } // S__S--R--R
        } else { // R--
            if seed.end() <= range.source_end() { // R--S__S--R
                let mapped_seed = SeedRange { start: range.map(seed.start), amount: seed.amount };

                mapped_seeds.extend(map_seeds(maps, next_map + 1, mapped_seed));
                return  mapped_seeds;

            } else if seed.start <= range.source_end() { // R--S__R__S
                let first = SeedRange { start: range.map(seed.start), amount: range.source_end() - seed.start + 1};
                let last = SeedRange { start: range.source_end() + 1, amount: seed.end() - range.source_end()};

                mapped_seeds.extend(map_seeds(maps, next_map + 1, first));
                mapped_seeds.extend(map_seeds(maps, next_map, last));
                return  mapped_seeds;
            } // R--R--S__S
        }
    }

    map_seeds(maps, next_map + 1, seed)
}

fn parse(lines: Vec<String>) -> (Vec<usize>, [Vec<MapRange>; 7]) {
    let seeds: Vec<usize> = lines[0][7..].split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();

    let mut maps: [Vec<MapRange>; 7] = [VEC; 7];
    let mut i= 0;
    for line in &lines[3..] {
        if line.is_empty() { continue; }
        if line.starts_with(|c: char| c.is_alphabetic()) {
            i += 1;
            continue;
        }
        maps[i].push(line_to_nums(line));
    }
    
    (seeds, maps)
}

fn line_to_nums(line: &String) -> MapRange {
    let numbers: Vec<usize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();

    MapRange {
        destination_start: numbers[0],
        source_start: numbers[1],
        amount: numbers[2],
    }
}

