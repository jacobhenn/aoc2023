use aocutil::prelude::*;

use std::ops::RangeInclusive;

const YEAR: usize = 2023;

const DAY: usize = 3;

fn solve<Part: AocPart>(input: &str) -> u32 {
    let mut grid: Grid<char> = input.lines().map(|line| line.chars()).collect();
    print!("{}", grid.render(|c| *c));

    let mut part_numbers: HashSet<(usize, RangeInclusive<usize>, u32)> = HashSet::new();

    for (y, row) in grid.rows().enumerate() {
        let mut cur_number_start = None;
        let mut cur_number = 0;
        let mut is_part_number = false;
        for (x, char) in row.iter().enumerate() {
            if char.is_digit(10) {
                if cur_number_start.is_none() {
                    cur_number_start = Some(x);
                }

                cur_number *= 10;
                cur_number += char.to_digit(10).unwrap();

                for (neighbor_delta_x, neighbor_delta_y) in [
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                    (0, 1),
                    (-1, 1),
                    (-1, 0),
                ] {
                    if let Some(c) = grid.get(v!(
                        x as isize + neighbor_delta_x,
                        y as isize + neighbor_delta_y
                    )) {
                        if *c != '.' && !c.is_digit(10) {
                            // trace!("symbol: {c}");
                            is_part_number = true;
                        }
                    }
                }
            } else {
                if is_part_number {
                    // trace!("found part number: {cur_number}");

                    part_numbers.insert((y, cur_number_start.unwrap()..=(x - 1), cur_number));

                    is_part_number = false;
                } else if cur_number != 0 {
                    // trace!("** found non-part number: {cur_number}");
                }

                cur_number = 0;
                cur_number_start = None;
            }
        }

        if is_part_number {
            // trace!("found part number: {cur_number}");

            part_numbers.insert((y, cur_number_start.unwrap()..=grid.width(), cur_number));

            is_part_number = false;
        } else if cur_number != 0 {
            // trace!("** found non-part number: {cur_number}");
        }
    }

    if Part::is_one() {
        return part_numbers.iter().map(|(_, _, num)| num).sum();
    }

    let mut res = 0;

    for (pos, c) in grid.iter_with_pos() {
        if *c == '*' {
            let adjacent_part_numbers = part_numbers
                .iter()
                .filter(|(y, xs, n)| {
                    [
                        v!(-1, -1),
                        v!(0, -1),
                        v!(1, -1),
                        v!(1, 0),
                        v!(1, 1),
                        v!(0, 1),
                        v!(-1, 1),
                        v!(-1, 0),
                    ]
                    .iter()
                    .any(|d| {
                        let neighbor_pos = pos + *d;
                        *y as isize == neighbor_pos[1] && xs.contains(&(neighbor_pos[0] as usize))
                    })
                })
                .vec();

            if adjacent_part_numbers.len() == 2 {
                trace!("found gear at {}", pos);
                let gear_ratio = adjacent_part_numbers[0].2 * adjacent_part_numbers[1].2;
                res += gear_ratio;
            } else {
                trace!("found non-gear at {pos}; adjacent part numbers: {adjacent_part_numbers:?}");
            }
        }
    }

    res
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
