use aocutil::prelude::*;

use std::ops::RangeInclusive;

const YEAR: usize = 2023;

const DAY: usize = 3;

#[derive(PartialEq, Eq, Hash, Debug)]
struct PartNumber {
    row: usize,
    cols: RangeInclusive<usize>,
    num: u32,
}

fn solve<Part: AocPart>(input: &str) -> u32 {
    let mut grid: Grid<char> = input.lines().map(|line| line.chars()).collect();
    print!("{}", grid.render(|c| *c));

    let mut part_numbers: HashSet<PartNumber> = HashSet::new();

    let mut part_numbers: HashSet<PartNumber> = input
        .lines()
        .enumerate()
        .map(|(y, row)| intsranges::<u32>(row).map(|(cols, num)| PartNumber { row: y, cols, num }))
        .flatten()
        .collect();
}

example_tests! {
    - part one:
        a0: "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.." => 4361,
    - part two:
        b0: "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.." => 467835,
}

#[test]
fn part_one() {
    let _ = aocutil::test_logger().try_init();
    assert_eq!(solve::<part::One>(&aocutil::get_input(YEAR, DAY)), 554003);
}

#[test]
fn part_two() {
    let _ = aocutil::test_logger().try_init();
    assert_eq!(solve::<part::Two>(&aocutil::get_input(YEAR, DAY)), 87263515);
}
