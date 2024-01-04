use aocutil::prelude::*;

pub const YEAR: usize = 2023;

pub const DAY: usize = 13;

fn one_different<'a>(
    lhs: impl IntoIterator<Item = &'a [char]>,
    rhs: impl IntoIterator<Item = &'a [char]>,
) -> bool {
    let mut differences = 0;

    for (l, r) in iter::zip(lhs, rhs) {
        for (l, r) in iter::zip(l, r) {
            if l != r {
                differences += 1;
            }
        }
    }

    differences == 1
}

/// Returns the greater of the two indices flanking the line of reflection.
fn index_of_reflection<P: AocPart>(pattern: &Grid<char>) -> Option<usize> {
    for i in 1..pattern.height() {
        let mut l: Vec<&[char]> = pattern.rows().take(i).collect();
        l.reverse();

        let r: Vec<&[char]> = pattern.rows().skip(i).collect();

        if match P::part() {
            Part::One => iter::zip(l, r).all(|(l, r)| l == r),
            Part::Two => one_different(l, r),
        } {
            return Some(i);
        }
    }

    None
}

fn reflection_line<P: AocPart>(pattern: Grid<char>) -> usize {
    if let Some(horizontal_line) = index_of_reflection::<P>(&pattern) {
        100 * horizontal_line
    } else {
        index_of_reflection::<P>(&pattern.transposed())
            .expect("there should be a line of reflection")
    }
}

pub fn solve<P: AocPart>(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|pattern| {
            let pattern: Grid<char> = pattern.lines().map(str::chars).collect();

            reflection_line::<P>(pattern)
        })
        .sum()
}

aoc_tests! {
    inputs {
        e0 = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
    }

    part::One {
        ea0: e0 => 405,
        ra: @input => 34889,
    }

    part::Two {
        eb0: e0 => 400,
        rb: @input => 34224,
    }
}
