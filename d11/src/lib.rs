use aocutil::prelude::*;

const YEAR: usize = 2023;

const DAY: usize = 11;

fn solve<P: AocPart>(input: &str, expansion_factor: usize) -> usize {
    let grid: Grid<char> = input.lines().map(|line| line.chars()).collect();

    let mut galaxies: Vec<GridPos> = grid.positions_zm().filter(|p| grid[*p] == '#').collect();

    debug!("galaxies: {galaxies:?}");

    let expanding_columns: HashSet<usize> = (0..grid.dimension(0))
        .filter(|x| galaxies.iter().all(|g| g[0] != *x as isize))
        .collect();

    let expanding_rows: HashSet<usize> = (0..grid.dimension(1))
        .filter(|y| galaxies.iter().all(|g| g[1] != *y as isize))
        .collect();

    // debug!("expanding rows: {expanding_rows:?}");
    debug!("expanding columns: {expanding_columns:?}");

    let mut total_dist = 0;
    while let Some(src_galaxy) = galaxies.pop() {
        for (i, dst_galaxy) in galaxies.iter().enumerate() {
            // find distance between src and dst in expanded universe

            let mut x_dist = 0;
            let src_x = cmp::min(src_galaxy[0], dst_galaxy[0]);
            let dst_x = cmp::max(src_galaxy[0], dst_galaxy[0]);
            for x in src_x..dst_x {
                if expanding_columns.contains(&(x as usize)) {
                    x_dist += expansion_factor;
                } else {
                    x_dist += 1;
                }
            }

            let mut y_dist = 0;
            let src_y = cmp::min(src_galaxy[1], dst_galaxy[1]);
            let dst_y = cmp::max(src_galaxy[1], dst_galaxy[1]);
            for y in src_y..dst_y {
                if expanding_rows.contains(&(y as usize)) {
                    y_dist += expansion_factor;
                } else {
                    y_dist += 1;
                }
            }

            trace!(
                "between galaxy {} and {}: {}",
                i + 1,
                galaxies.len() + 1,
                x_dist + y_dist
            );
            total_dist += x_dist + y_dist;
        }
    }

    total_dist
}

aoc_tests! {
    inputs {
        ex = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",

    }

    part::One {
        a0: ex, 2 => 374,
        a: @input 2 => 10077850,
    }

    part::Two {
        b0: ex, 1000000 => 82000210,
        b: @input 1000000 => 504715068438,
    }
}
