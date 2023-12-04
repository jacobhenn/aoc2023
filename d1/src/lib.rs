use aocutil::prelude::*;

const YEAR: usize = 2023;

const DAY: usize = 1;

fn solve<Part: AocPart>(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            debug!("{line}");
            let mut digits = ('0'..='9')
                .enumerate()
                .map(move |(digit, name)| {
                    line.match_indices(move |c| c == name)
                        .map(move |(i, _)| (i, digit))
                })
                .flatten()
                .vec();

            if Part::is_two() {
                digits.extend(
                    DIGIT_NAMES
                        .iter()
                        .enumerate()
                        .map(move |(digit, name)| {
                            line.match_indices(name).map(move |(i, _)| (i, digit))
                        })
                        .flatten(),
                );
            }

            digits.sort_by_key(|(pos, digit)| *pos);
            10 * digits[0].1 + digits.last().unwrap().1
        })
        .sum()
}

example_tests! {
    - part one:
        a0: "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet" => 142,
    - part two:
        b0: "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen" => 281,
}

#[test]
fn part_one() {
    let _ = aocutil::test_logger().try_init();
    assert_eq!(solve::<part::One>(&aocutil::get_input(YEAR, DAY)), 56465);
}

#[test]
fn part_two() {
    let _ = aocutil::test_logger().try_init();
    assert_eq!(solve::<part::Two>(&aocutil::get_input(YEAR, DAY)), 55902);
}
