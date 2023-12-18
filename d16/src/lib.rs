use aocutil::prelude::*;

pub const YEAR: usize = 2023;

pub const DAY: usize = 16;

pub fn energize(grid: &Grid<char>, start_pos: GridPos, start_dir: RookDirection) -> usize {
    let mut all_beams = HashSet::new();
    let mut beams = Vec::new();
    beams.push((start_pos, start_dir));

    while beams.len() != 0 {
        let mut new_beams = Vec::new();

        for (mut beam_pos, mut beam_dir) in &beams {
            if grid.contains_pos(beam_pos) && !all_beams.insert((beam_pos, beam_dir)) {
                continue;
            }

            beam_pos += GridPos::from(beam_dir);

            match grid.get(beam_pos) {
                Some('/') => {
                    beam_dir = RookDirection::new(beam_dir.perpendicular_axis(), -beam_dir.sign());
                }
                Some('\\') => {
                    beam_dir = beam_dir.with_axis(beam_dir.perpendicular_axis());
                }
                Some('|') => {
                    if beam_dir.axis() == 0 {
                        beam_dir = beam_dir.with_axis(beam_dir.perpendicular_axis());
                        new_beams.push((beam_pos, -beam_dir));
                    }
                }
                Some('-') => {
                    if beam_dir.axis() == 1 {
                        beam_dir = beam_dir.with_axis(beam_dir.perpendicular_axis());
                        new_beams.push((beam_pos, -beam_dir));
                    }
                }
                Some(_) => (),
                None => continue,
            }

            new_beams.push((beam_pos, beam_dir));
        }

        beams = new_beams;
    }

    all_beams
        .into_iter()
        .map(|(p, _)| p)
        .collect::<HashSet<_>>()
        .len()
}

pub fn solve<Part: AocPart>(input: &str) -> usize {
    let grid: Grid<char> = input.lines().map(str::chars).collect();

    if Part::is_one() {
        return energize(&grid, v!(-1, 0), RookDirection::PLUS_X);
    }

    let mut max_energy = 0;

    for x in 0..grid.width() {
        trace!("x: {x} of {}", grid.width());

        let energy = energize(&grid, v!(x as isize, -1), RookDirection::PLUS_Y);
        max_energy = cmp::max(max_energy, energy);

        let energy = energize(
            &grid,
            v!(x as isize, grid.height() as isize),
            RookDirection::MINUS_Y,
        );
        max_energy = cmp::max(max_energy, energy);
    }

    for y in 0..grid.height() {
        trace!("x: {y} of {}", grid.height());

        let energy = energize(&grid, v!(-1, y as isize), RookDirection::PLUS_X);
        max_energy = cmp::max(max_energy, energy);

        let energy = energize(
            &grid,
            v!(grid.width() as isize, y as isize),
            RookDirection::MINUS_X,
        );
        max_energy = cmp::max(max_energy, energy);
    }

    max_energy
}

aoc_tests! {
    inputs {
        e0 = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
    }

    part::One {
        ea0: e0 => 46,
        ra: @input => 6906,
    }

    part::Two {
        eb0: e0 => 51,
        rb: @input => 7330,
    }
}
