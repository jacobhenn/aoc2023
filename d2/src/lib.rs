use aocutil::prelude::*;

const YEAR: usize = 2023;

const DAY: usize = 2;

#[derive(Default)]
struct Draw {
    red: usize,
    green: usize,
    blue: usize,
}

impl Draw {
    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }

    fn intersect(l: Self, r: Self) -> Self {
        Draw {
            red: cmp::max(l.red, r.red),
            green: cmp::max(l.green, r.green),
            blue: cmp::max(l.blue, r.blue),
        }
    }
}

impl FromStr for Draw {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = Self::default();

        for cube_count in s.split(",") {
            let (count, cube) = cube_count
                .trim()
                .split_once(' ')
                .context("no space in cube count")?;
            let count = count.parse::<usize>()?;
            match cube {
                "red" => res.red = count,
                "green" => res.green = count,
                "blue" => res.blue = count,
                other => unreachable!("bad cube name '{other}'"),
            }
        }

        Ok(res)
    }
}

fn solve<P: AocPart>(input: &str) -> usize {
    let games = input.lines().map(|line| {
        let (game_id, game) = line.split_once(':').unwrap();
        let game_id = ints::<usize>(game_id).next().unwrap();
        (
            game_id,
            game.split(';').map(|draw| draw.parse::<Draw>().unwrap()),
        )
    });

    match P::part() {
        Part::One => games
            .filter_map(|(id, mut game)| {
                let is_possible =
                    game.all(|draw| draw.red <= 12 && draw.green <= 13 && draw.blue <= 14);
                is_possible.then_some(id)
            })
            .sum(),
        Part::Two => games
            .map(|(_id, game)| {
                game.reduce(|ldraw, rdraw| Draw::intersect(ldraw, rdraw))
                    .unwrap()
                    .power()
            })
            .log_dbg()
            .sum(),
    }
}

const EXAMPLE_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

example_tests! {
    - part one:
        a0: EXAMPLE_INPUT => 8,
    - part two:
        b0: EXAMPLE_INPUT => 2286,
}

#[test]
fn part_one() {
    let _ = aocutil::test_logger().try_init();
    assert_eq!(solve::<part::One>(&aocutil::get_input(YEAR, DAY)), 2317);
}

#[test]
fn part_two() {
    let _ = aocutil::test_logger().try_init();
    assert_eq!(solve::<part::Two>(&aocutil::get_input(YEAR, DAY)), 0);
}
