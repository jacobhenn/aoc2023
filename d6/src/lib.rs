use aocutil::prelude::*;

const YEAR: usize = 2023;

const DAY: usize = 6;

#[instrument(level = "trace")]
fn num_ways_to_win(race_time: f64, best_distance: f64) -> u128 {
    let vertex_time = race_time / 2.0;
    let discriminant = race_time.powi(2) - 4.0 * best_distance;
    let res = if discriminant < 0.0 {
        0
    } else {
        let pm = -discriminant.sqrt() / 2.0;
        let lower_bound = cmp::max(0, (vertex_time + pm + 1.0).floor() as u128);
        let upper_bound = (vertex_time - pm - 1.0).ceil() as u128;
        trace!("{lower_bound} .. {upper_bound}");
        upper_bound - lower_bound + 1
    };
    trace!("  {res}");
    res
}

fn solve<Part: AocPart>(input: &str) -> u128 {
    let mut lines = input.lines();

    if Part::is_one() {
        let race_times = ints::<i32>(lines.next().unwrap())
            .map(f64::from)
            .collect_vec();
        let best_distances = ints::<i32>(lines.next().unwrap())
            .map(f64::from)
            .collect_vec();
        iter::zip(race_times, best_distances)
            .map(|(race_time, best_distance)| num_ways_to_win(race_time, best_distance))
            .product()
    } else {
        let race_time: f64 = ints::<u64>(&lines.next().unwrap().replace(" ", ""))
            .next()
            .unwrap() as f64;

        let best_distance: f64 = ints::<u64>(&lines.next().unwrap().replace(" ", ""))
            .next()
            .unwrap() as f64;

        num_ways_to_win(race_time, best_distance)
    }
}

const EXAMPLE_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

example_tests! {
    - part one:
        a0: EXAMPLE_INPUT => 288,
    - part two:
        b0: EXAMPLE_INPUT => 71503,
}

#[test]
fn part_one() {
    let _ = aocutil::log::test_subscriber().try_init();
    assert_eq!(solve::<part::One>(&aocutil::get_input(YEAR, DAY)), 5133600);
}

#[test]
fn part_two() {
    let _ = aocutil::log::test_subscriber().try_init();
    assert_eq!(solve::<part::Two>(&aocutil::get_input(YEAR, DAY)), 40651271);
}
