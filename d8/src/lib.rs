use aocutil::prelude::*;

const YEAR: usize = 2023;

const DAY: usize = 8;

fn solve_a(instrs: &str, nodes: HashMap<&str, (&str, &str)>) -> usize {
    let mut node = "AAA";
    for (idx, instr) in instrs.chars().cycle().enumerate() {
        match instr {
            'L' => node = nodes.get(node).expect("node should be present").0,
            'R' => node = nodes.get(node).expect("node should be present").1,
            other => panic!("invalid R/L instruction {other}"),
        }

        if node == "ZZZ" {
            return idx + 1;
        }
    }

    unreachable!()
}

fn solve_b(instrs: &str, nodes: HashMap<&str, (&str, &str)>) -> usize {
    todo!();

    let mut current_nodes: HashSet<&str> = nodes
        .keys()
        .filter_map(|n| n.ends_with('A').then_some(*n))
        .collect();

    for (idx, instr) in instrs.chars().cycle().enumerate() {
        current_nodes = current_nodes
            .into_iter()
            .map(|node| match instr {
                'L' => nodes.get(node).expect("node should be present").0,
                'R' => nodes.get(node).expect("node should be present").1,
                other => panic!("invalid R/L instruction {other}"),
            })
            .collect();

        if current_nodes.iter().all(|n| n.ends_with('Z')) {
            return idx + 1;
        }
    }

    todo!()
}

fn solve<Part: AocPart>(input: &str) -> usize {
    let mut lines = input.lines();

    let instrs = lines.next().unwrap();
    lines.next(); // blank line

    let nodes: HashMap<&str, (&str, &str)> = lines
        .map(|line| {
            unlist!(
                line.split(|c: char| !c.is_ascii_alphanumeric())
                    .filter(|s| !s.is_empty()),
                name,
                l,
                r
            );
            (name, (l, r))
        })
        .collect();

    if Part::is_one() {
        solve_a(instrs, nodes)
    } else {
        solve_b(instrs, nodes)
    }
}

example_tests! {
    - part one:
        a0: "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)" => 2,
        a1: "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)" => 6,
    - part two:
        b0: "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)" => 6,
}

#[test]
fn part_one() {
    let _ = aocutil::test_logger().try_init();
    assert_eq!(solve::<part::One>(&aocutil::get_input(YEAR, DAY)), 23147);
}

#[test]
fn part_two() {
    let _ = aocutil::test_logger().try_init();
    assert_eq!(solve::<part::Two>(&aocutil::get_input(YEAR, DAY)), 0);
}
