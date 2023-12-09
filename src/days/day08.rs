use std::collections::HashMap;

pub fn solve(part: u32, lines: Vec<String>) -> u32 {
    if part == 1 {
        part1(lines)
    } else {
        part2(lines)
    }
}

fn part1(lines: Vec<String>) -> u32 {
    let (instructions, nodes) = parse(lines);
    
    count_steps(&instructions, &nodes, "AAA".to_string(), |x: String| x == "ZZZ")
}

fn part2(lines: Vec<String>) -> u32 {
    let (instructions, nodes) = parse(lines);

    let curr_nodes:Vec<u32> = nodes.keys().filter(|node| node.ends_with('A')).map(|x| count_steps(&instructions, &nodes, x.to_string(), |x| x.ends_with("Z"))).collect();
    dbg!(curr_nodes);
    println!("I couldn't decide if I wanna add crates or not so just uhh get the LCM of these numbers from somewhere");
    todo!()
}

fn count_steps<F>(instructions: &String, nodes: &HashMap<String, (String, String)>, node: String, end_condition: F) -> u32
where F: Fn(String) -> bool
{
    let _ = end_condition;
    let mut curr_node = node;
    let mut count = 0;
    for instr in instructions.chars().cycle() {
        count += 1;
        curr_node = match instr {
            'L' => nodes[&curr_node].0.clone(),
            'R' => nodes[&curr_node].1.clone(),
            _ => panic!(),
        };

        if end_condition(curr_node.clone()) { break; }
    }

    count
}

fn parse(lines: Vec<String>) -> (String, HashMap<String, (String, String)>) {
    let instructions = lines[0].clone();

    let mut nodes = HashMap::new();

    for line in &lines[2..] {
        let curr = line[0..3].to_owned();
        let left = line[7..10].to_owned();
        let right = line[12..15].to_owned();
        nodes.insert(curr, (left, right));
    }

    (instructions, nodes)
}