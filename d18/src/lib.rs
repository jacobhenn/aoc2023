use aocutil::prelude::*;

pub const YEAR: usize = 2023;

pub const DAY: usize = 18;

#[derive(Clone, Copy)]
struct Step {
    direction: RookDirection,
    length: i64,
}

impl Step {
    fn from_str<P: AocPart>(s: &str) -> Self {
        unlist!(s.split_whitespace(), dir, len, col);

        let color = col.trim_matches(&['(', ')', '#'][..]);

        let direction = match P::part() {
            Part::One => RookDirection::from_udlr_positive_down(dir).expect("direction is valid"),
            Part::Two => match &color[5..] {
                "0" => RookDirection::PLUS_X,
                "1" => RookDirection::PLUS_Y,
                "2" => RookDirection::MINUS_X,
                "3" => RookDirection::MINUS_Y,
                other => panic!("invalid direction suffix '{other}'"),
            },
        };

        let length = match P::part() {
            Part::One => len.parse().expect("step length is valid"),
            Part::Two => i64::from_str_radix(&color[..5], 16).expect("color is valid hex"),
        };

        Self { direction, length }
    }
}

fn corner_ys(steps: &[Step]) -> BTreeSet<i64> {
    let mut corner_ys = BTreeSet::new();

    let mut pos = v!(0, 0);

    for step in steps {
        pos += step.direction.unit_vector::<i64>() * step.length;

        corner_ys.insert(pos[1]);
    }

    corner_ys
}

/// The pairs are (x, y)
fn trench_slice(steps: &[Step], slice_y: i64) -> BTreeMap<i64, (bool, LineDirection)> {
    let mut slice = BTreeMap::new();

    let mut pos = v!(0, 0);

    for step in steps {
        let dst = pos + (step.direction.unit_vector::<i64>() * step.length);

        if step.direction.is_vertical() {
            if Range::ordered(pos[1], dst[1]).contains(&slice_y) {
                slice
                    .entry(pos[0])
                    .or_insert((false, step.direction.sign()))
                    .1 = step.direction.sign();
            }
        } else {
            if pos[1] == slice_y {
                slice
                    .entry(pos[0])
                    .or_insert((true, LineDirection::Negative))
                    .0 = true;

                slice
                    .entry(dst[0])
                    .or_insert((true, LineDirection::Negative))
                    .0 = true;
            }
        }

        pos = dst;
    }

    slice
}

fn slice_area(slice: &BTreeMap<i64, (bool, LineDirection)>) -> i64 {
    let mut area = 0;

    let mut entered_lagoon: Option<i64> = None;

    let mut entered_trench: Option<(i64, LineDirection)> = None;

    for (&x, &(is_corner, vertical_direction)) in slice {
        if is_corner {
            if let Some((entered_trench_x, entered_trench_direction)) = entered_trench {
                // we are at the end of a stretch
                if entered_trench_direction != vertical_direction {
                    // in-and-back stretch
                    if entered_lagoon.is_none() {
                        // we were outside the lagoon; just add this stretch
                        area += x - entered_trench_x + 1;
                    }
                    // if we are inside the lagoon, this changes nothing
                } else {
                    // in-and-through stretch; flip inside-outside
                    if let Some(entered_x) = entered_lagoon {
                        area += x - entered_x + 1;
                        entered_lagoon = None;
                    } else {
                        entered_lagoon = Some(entered_trench_x);
                    }
                }

                entered_trench = None;
            } else {
                // we are at the beginning of a stretch
                entered_trench = Some((x, vertical_direction));
            }
        } else {
            if let Some(entered_x) = entered_lagoon {
                area += x - entered_x + 1;
                entered_lagoon = None;
            } else {
                entered_lagoon = Some(x);
            }
        }
    }

    area
}

pub fn solve<Part: AocPart>(input: &str) -> i64 {
    let steps: Vec<Step> = input.lines().map(Step::from_str::<Part>).collect();

    let corner_ys = corner_ys(&steps);

    let mut area = 0;

    for (&corner_y, &next_corner_y) in corner_ys.iter().tuple_windows() {
        area += slice_area(&trench_slice(&steps, corner_y));

        let next_gap_len = next_corner_y - corner_y - 1;
        area += next_gap_len * slice_area(&trench_slice(&steps, corner_y + 1));
    }

    let last_corner_y = corner_ys.iter().last().expect("at least one corner y");
    area += slice_area(&trench_slice(&steps, *last_corner_y));

    area
}

aoc_tests! {
    inputs {
        e0 = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",

        e1 = "D 6 .
R 2 .
U 2 .
R 2 .
D 2 .
R 2 .
U 6 .
L 2 .
D 2 .
L 2 .
U 2 .
L 2 .",
    }

    part::One {
        ea0: e0 => 62,
        ea1: e1 => 45,
        ra: @input => 76387,
    }

    part::Two {
        eb0: e0 => 952408144115,
        rb: @input => 250022188522074,
    }
}
