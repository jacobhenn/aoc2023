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
            Part::Two => match color.chars().last().expect("color is non-empty") {
                '0' => RookDirection::PLUS_X,
                '1' => RookDirection::PLUS_Y,
                '2' => RookDirection::MINUS_X,
                '3' => RookDirection::MINUS_Y,
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

fn trench_slices(
    steps: impl IntoIterator<Item = Step>,
) -> HashMap<i64, BTreeMap<i64, HashSet<RookDirection>>> {
    let mut slices: HashMap<i64, BTreeMap<i64, HashSet<RookDirection>>> = HashMap::new();

    let mut pos = v!(0, 0);

    for step in steps {
        for i in 0..=step.length {
            if i != 0 {
                pos += step.direction;
            }

            slices
                .entry(pos[1])
                .or_default()
                .entry(pos[0])
                .or_default()
                .insert(step.direction);
        }
    }

    slices
}

#[instrument(level = "trace", ret)]
fn slice_area(slice: &BTreeMap<i64, HashSet<RookDirection>>) -> i64 {
    let mut area = 0;

    let mut entered_lagoon: Option<i64> = None;

    let mut entered_trench: Option<(i64, LineDirection)> = None;

    for (&x, directions) in slice {
        if directions.len() == 1 {
            // straight horizontal or vertical
            if directions
                .iter()
                .next()
                .expect("should be exactly one direction")
                .is_vertical()
            {
                // vertical tile; flip inside-outside
                if let Some(entered_x) = entered_lagoon {
                    area += x - entered_x + 1;
                    entered_lagoon = None;
                } else {
                    entered_lagoon = Some(x);
                }
            } else {
                // horizontal tile; do nothing
            }
        } else {
            // we're at a bend

            let vertical_direction = directions
                .iter()
                .filter(|d| d.is_vertical())
                .next()
                .expect("should be two directions, one vertical");

            if let Some((entered_trench_x, entered_trench_direction)) = entered_trench {
                // we are at the end of a stretch
                if entered_trench_direction != vertical_direction.sign() {
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
                assert_eq!(entered_trench, None);
                entered_trench = Some((x, vertical_direction.sign()));
            }
        }
    }

    area
}

pub fn solve<Part: AocPart>(input: &str) -> i64 {
    if Part::is_two() {
        todo!("part 2 currently might cause an out-of-memory crash");
    }

    let steps = input.lines().map(Step::from_str::<Part>);

    let slices = trench_slices(steps);

    trace!("{:#?}", slices);

    slices
        .iter()
        .map(|(i, s)| {
            trace!("slice {i}");
            slice_area(s)
        })
        .sum()
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
        rb: @input => 0,
    }
}
