use aocutil::prelude::*;

const YEAR: usize = 2023;

const DAY: usize = 9;

fn solve<P: AocPart>(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut sequences: Vec<Vec<i32>> = vec![ints(line).collect()];
            while !sequences.last().unwrap().iter().all_equal() {
                let sequence = sequences.last().unwrap();
                let diffs = sequence
                    .iter()
                    .tuple_windows()
                    .map(|(n, m)| m - n)
                    .collect();
                sequences.push(diffs);
            }

            while sequences.len() > 1 {
                let tmp = sequences.pop().unwrap();
                let prev_seq = sequences.last_mut().unwrap();

                match P::part() {
                    Part::One => {
                        let diff = tmp.last().unwrap();
                        let prev_last = prev_seq.last().unwrap();
                        prev_seq.push(prev_last + diff);
                    }
                    Part::Two => {
                        let diff = tmp[0];
                        let prev_first = prev_seq[0];
                        prev_seq.insert(0, prev_first - diff);
                    }
                }
            }

            match P::part() {
                Part::One => *sequences[0].last().unwrap(),
                Part::Two => sequences[0][0],
            }
        })
        .sum()
}

const EXAMPLE_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

example_tests! {
    - part one:
        a0: EXAMPLE_INPUT => 114,
    - part two:
        b0: EXAMPLE_INPUT => 2,
}

#[test]
fn part_one() {
    let _ = aocutil::test_logger().try_init();
    assert_eq!(
        solve::<part::One>(&aocutil::get_input(YEAR, DAY)),
        1898776583
    );
}

#[test]
fn part_two() {
    let _ = aocutil::test_logger().try_init();
    assert_eq!(solve::<part::Two>(&aocutil::get_input(YEAR, DAY)), 1100);
}
