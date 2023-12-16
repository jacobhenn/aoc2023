use std::ops::Neg;

use aocutil::prelude::*;

const YEAR: usize = 2023;

const DAY: usize = 10;

static PIPE_DIRECTIONS: phf::Map<char, [RookDirection; 2]> = phf::phf_map! {
    '|' => [RookDirection::MINUS_Y, RookDirection::PLUS_Y],
    '-' => [RookDirection::MINUS_X, RookDirection::PLUS_X],
    'L' => [RookDirection::PLUS_X, RookDirection::MINUS_Y],
    'J' => [RookDirection::MINUS_X, RookDirection::MINUS_Y],
    '7' => [RookDirection::MINUS_X, RookDirection::PLUS_Y],
    'F' => [RookDirection::PLUS_X, RookDirection::PLUS_Y],
};

fn uncover_start(mut grid: Grid<char>) -> (Grid<char>, GridPos) {
    trace!("determining pipe under 'S'");

    let (start_pos, _) = grid.iter_zm_with_pos().find(|(_, c)| **c == 'S').unwrap();

    let mut start_directions = RookDirection::iter().filter_map(|d| {
        let neighbor = grid.get(start_pos + d)?;
        trace!("  looking at neighbor {neighbor:?}");
        let res = PIPE_DIRECTIONS
            .get(neighbor)
            .and_then(|directions| directions.contains(&-d).then_some(d));
        trace!("  res = {res:?}");
        res
    });

    let start_directions = [
        start_directions.next().unwrap(),
        start_directions.next().unwrap(),
    ];

    let (&start_pipe, _) = PIPE_DIRECTIONS
        .entries()
        .find(|(_, v)| **v == start_directions)
        .expect("directions should correspond to a pipe");

    grid[start_pos] = start_pipe;

    debug!("  S = {start_pipe:?}");

    (grid, start_pos)
}

fn solve<Part: AocPart>(input: &str) -> usize {
    let grid: Grid<char> = input.lines().map(|line| line.chars()).collect();

    debug!("parsed grid:\n{}", grid.render(|_, c| *c));

    let (grid, start_pos) = uncover_start(grid);

    let start_dirs = PIPE_DIRECTIONS[&grid[start_pos]];

    let mut current = [start_pos + start_dirs[0], start_pos + start_dirs[1]];
    let mut prev_dir = [start_dirs[0], start_dirs[1]];

    let mut loop_pipes: HashSet<GridPos> = HashSet::new();
    loop_pipes.insert(start_pos);

    for distance in 2.. {
        for i in 0..2 {
            loop_pipes.insert(current[i]);

            let next_dir = *PIPE_DIRECTIONS[&grid[current[i]]]
                .iter()
                .filter(|dir| **dir != -prev_dir[i])
                .exactly_one()
                .unwrap();

            current[i] = current[i] + next_dir;
            prev_dir[i] = next_dir;
        }

        if current[0] == current[1] {
            loop_pipes.insert(current[0]);

            if Part::is_one() {
                return distance;
            } else {
                break;
            }
        }
    }

    debug!(
        "\n{}",
        grid.render(|p, c| if loop_pipes.contains(&p) { '#' } else { *c })
    );

    let mut area = 0;

    let mut inside = false;
    let mut entered = None;

    for (pos, tile) in grid.iter_zm_with_pos() {
        if loop_pipes.contains(&pos) {
            match tile {
                '|' => inside = !inside,
                '-' => (),
                'L' => entered = Some('L'),
                'J' => {
                    if entered.unwrap() == 'F' {
                        inside = !inside
                    }
                }
                '7' => {
                    if entered.unwrap() == 'L' {
                        inside = !inside
                    }
                }
                'F' => entered = Some('F'),
                other => panic!("{other} at {pos} is not a loop pipe"),
            }

            trace!("{pos} {tile:?}, inside = {inside}");
        } else if inside {
            trace!("found inside: {pos}");
            area += 1;
        }
    }

    area
}

example_tests! {
    - part one:
        a00: ".....
.S-7.
.|.|.
.L-J.
....."=> 4,
        a01: "-L|F7
7S-7|
L|7||
-L-J|
L|-JF"=> 4,
        a10: "..F7.
.FJ|.
SJ.L7
|F--J
LJ..." => 8,
        a11: "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ" => 8,
    - part two:
        b0: "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........." => 4,
        b1: ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..." => 8,
        b2: "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L" => 10,
}

#[test]
fn part_one() {
    let _ = aocutil::test_logger().try_init();
    assert_eq!(solve::<part::One>(&aocutil::get_input(YEAR, DAY)), 6714);
}

#[test]
fn part_two() {
    let _ = aocutil::test_logger().try_init();
    assert_eq!(solve::<part::Two>(&aocutil::get_input(YEAR, DAY)), 429);
}
