use aocutil::prelude::*;

const YEAR: usize = 2023;

const DAY: usize = 5;

fn solve<Part: AocPart>(input: &str) -> i64 {
    let mut pars = input.split("\n\n");

    debug!("hi");
    let mut seeds = ints::<i64>(pars.next().unwrap()).vec();

    for par in pars {
        debug!("meow");
        let map = par
            .lines()
            .skip(1)
            .map(|line| {
                let ns = ints::<i64>(line).vec();
                (ns[1]..(ns[1] + ns[2]), ns[0] - ns[1])
            })
            .vec();

        for seed in &mut seeds {
            if let Some((range, delta)) = map.iter().find(|(range, _)| range.contains(&seed)) {
                *seed += delta;
            }
        }
    }

    *seeds.iter().min().unwrap()
}

const EXAMPLE_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

example_tests! {
    - part one:
        a0: EXAMPLE_INPUT => 35,
    - part two:
        b0: EXAMPLE_INPUT => 0,
}

#[test]
fn part_one() {
    let _ = aocutil::test_logger().try_init();
    assert_eq!(solve::<part::One>(&aocutil::get_input(YEAR, DAY)), 0);
}

#[test]
fn part_two() {
    let _ = aocutil::test_logger().try_init();
    assert_eq!(solve::<part::Two>(&aocutil::get_input(YEAR, DAY)), 0);
}
