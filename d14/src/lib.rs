use aocutil::prelude::*;

pub const YEAR: usize = 2023;

pub const DAY: usize = 14;

fn tilt_platform(grid: &Grid<char>, round_rocks: &mut HashSet<GridPos>, direction: RookDirection) {
    for i in 0..grid.dimension(direction.perpendicular_axis()) {
        let fall_axis_dimension = grid.dimension(direction.axis()) as isize;

        let mut falling_point = match direction.sign() {
            LineDirection::Negative => 0,
            LineDirection::Positive => fall_axis_dimension - 1,
        };

        let range: Box<dyn Iterator<Item = isize>> = match direction.sign() {
            LineDirection::Negative => Box::new(0..fall_axis_dimension),
            LineDirection::Positive => Box::new((0..fall_axis_dimension).rev()),
        };

        for j in range {
            let mut pos = v!(0, 0);
            pos[direction.axis()] = j;
            pos[direction.perpendicular_axis()] = i as isize;

            if round_rocks.remove(&pos) {
                pos[direction.axis()] = falling_point as isize;
                round_rocks.insert(pos);
                falling_point -= direction.sign().to_num::<isize>();
            } else if grid[pos] == '#' {
                falling_point = j - direction.sign().to_num::<isize>();
            }
        }
    }
}

fn spin_cycle(grid: &Grid<char>, round_rocks: &mut HashSet<GridPos>) {
    tilt_platform(grid, round_rocks, RookDirection::MINUS_Y);
    tilt_platform(grid, round_rocks, RookDirection::MINUS_X);
    tilt_platform(grid, round_rocks, RookDirection::PLUS_Y);
    tilt_platform(grid, round_rocks, RookDirection::PLUS_X);
}

fn north_total_load(grid: &Grid<char>, round_rocks: &HashSet<GridPos>) -> isize {
    round_rocks
        .iter()
        .map(|round_rock| grid.height() as isize - round_rock[1])
        .sum()
}

fn log_state(grid: &Grid<char>, round_rocks: &HashSet<GridPos>) {
    trace!(
        "\n{}",
        grid.render(|pos, &c| if c == '#' {
            '#'
        } else if round_rocks.contains(&pos) {
            'O'
        } else {
            '.'
        })
    );
}

pub fn solve<Part: AocPart>(input: &str) -> isize {
    let grid: Grid<char> = input.lines().map(str::chars).collect();

    let mut round_rocks: HashSet<GridPos> = grid
        .positions_zm()
        .filter(|&pos| grid[pos] == 'O')
        .collect();

    log_state(&grid, &round_rocks);

    if Part::is_one() {
        tilt_platform(&grid, &mut round_rocks, RookDirection::MINUS_Y);

        log_state(&grid, &round_rocks);

        return north_total_load(&grid, &round_rocks);
    }

    let mut cache: HashMap<u64, usize> = HashMap::new();

    let billion = 1_000_000_000;
    for i in 0..billion {
        spin_cycle(&grid, &mut round_rocks);

        trace!(i);
        log_state(&grid, &round_rocks);

        // std::thread::sleep(std::time::Duration::from_secs(1));

        let hash = round_rocks.iter().hash();
        if let Some(prev_i) = cache.insert(hash, i) {
            trace!(prev_i, i);
            let remaining_i = billion - i;
            let cycle_step = i - prev_i;
            let remaining_cycles = remaining_i / cycle_step;
            let jump_i = 1 + i + cycle_step * remaining_cycles;
            for _ in jump_i..billion {
                spin_cycle(&grid, &mut round_rocks);
            }
            break;
        }
    }

    north_total_load(&grid, &round_rocks)
}

aoc_tests! {
    inputs {
        ex = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
    }

    part::One {
        ea0: ex => 136,
        ra: @input => 105784,
    }

    part::Two {
        eb0: ex => 64,
        rb: @input => 91286,
    }
}
