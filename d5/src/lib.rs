use aocutil::prelude::*;

const YEAR: usize = 2023;

const DAY: usize = 5;

fn solve_b(input: &str) -> i64 {
    let mut pars = input.split("\n\n");

    let mut seeds: RangeSet<i64> = RangeSet::new();
    for [a, b] in ints::<i64>(pars.next().unwrap()).util_array_chunks() {
        seeds.insert(Range::new(a, a + b - 1));
    }

    for par in pars {
        debug!("{seeds:?}");
        let map = par
            .lines()
            .skip(1)
            .map(|line| {
                let ns = ints::<i64>(line).vec();
                (Range::new(ns[1], ns[1] + ns[2] - 1), ns[0] - ns[1])
            })
            .vec();

        let mut new_seeds: RangeSet<i64> = RangeSet::new();
        let mut remaining_seeds: RangeSet<i64> = seeds.clone();

        for (source_range, delta) in map {
            debug!("  mapping {source_range:?}: {delta}");
            for seed_range in seeds.ranges() {
                debug!("    looking at seed range {seed_range:?}");
                let intersection = source_range.intersection(*seed_range);
                if intersection.start <= intersection.end {
                    debug!("      found intersection {intersection:?}");
                    new_seeds.insert(intersection.map(|bound| bound + delta));
                    remaining_seeds.remove(intersection);
                }
            }
        }

        debug!("  remaining seeds: {remaining_seeds:?}");

        for seed_range in remaining_seeds.into_ranges() {
            new_seeds.insert(seed_range);
        }

        seeds = new_seeds;
    }

    *seeds.min().unwrap()
}

fn solve_a(input: &str) -> i64 {
    let mut pars = input.split("\n\n");

    let mut seeds = ints::<i64>(pars.next().unwrap()).vec();

    for par in pars {
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

pub fn solve<P: AocPart>(input: &str) -> i64 {
    match P::part() {
        Part::One => solve_a(input),
        Part::Two => solve_b(input),
    }
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
        b0: EXAMPLE_INPUT => 46,
}

#[test]
fn part_one() {
    let _ = aocutil::log::test_subscriber().try_init();
    assert_eq!(
        solve::<part::One>(&aocutil::get_input(YEAR, DAY)),
        457535844
    );
}

#[test]
fn part_two() {
    let _ = aocutil::log::test_subscriber().try_init();
    assert_eq!(solve::<part::Two>(&aocutil::get_input(YEAR, DAY)), 41222968);
}
