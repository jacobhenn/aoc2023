use aocutil::prelude::*;

const YEAR: usize = 2023;

const DAY: usize = 4;

fn score(card: &str) -> (usize, usize) {
    let (x, y) = card.split_once("|").unwrap();

    let winning: HashSet<u32> = ints(x).skip(1).collect();
    let mut score = 0;
    let mut matching = 0;

    for have in ints::<u32>(y) {
        if winning.contains(&have) {
            matching += 1;
            if score == 0 {
                score = 1;
            } else {
                score *= 2;
            }
        }
    }

    (score, matching)
}

fn solve<Part: AocPart>(input: &str) -> usize {
    let mut cards: Vec<(&str, usize)> = input.lines().map(|line| (line, 1)).collect();

    if Part::is_one() {
        return cards.iter().map(|(card, _)| score(card).0).sum();
    }

    for idx in 0..cards.len() {
        let (card, copies) = cards[idx];
        trace!("looking at Card {}", idx + 1);

        let (score, matching) = score(&card);

        trace!("  score = {score}, and there are {copies} copies");

        for i in (idx + 1)..=(idx + matching) {
            cards[i].1 += copies;
        }
    }

    cards.iter().map(|(card, count)| count).sum()
}

const EXAMPLE_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

example_tests! {
    - part one:
        a0: EXAMPLE_INPUT => 13,
    - part two:
        b0: EXAMPLE_INPUT => 30,
}

#[test]
fn part_one() {
    let _ = aocutil::test_logger().try_init();
    assert_eq!(solve::<part::One>(&aocutil::get_input(YEAR, DAY)), 21959);
}

#[test]
fn part_two() {
    let _ = aocutil::test_logger().try_init();
    assert_eq!(solve::<part::Two>(&aocutil::get_input(YEAR, DAY)), 5132675);
}
