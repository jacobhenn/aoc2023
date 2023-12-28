use std::cmp::Ordering;

use aocutil::prelude::*;

pub const YEAR: usize = 2023;

pub const DAY: usize = 7;

const CARD_ORDER_A: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

const CARD_ORDER_B: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
struct Card(char);

impl Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(self.0)
    }
}

impl Card {
    fn rank<P: AocPart>(&self) -> usize {
        match P::part() {
            Part::One => CARD_ORDER_A,
            Part::Two => CARD_ORDER_B,
        }
        .iter()
        .position(|c| *c == self.0)
        .expect("card should be valid")
    }

    fn cmp<Part: AocPart>(&self, other: &Self) -> cmp::Ordering {
        self.rank::<Part>().cmp(&other.rank::<Part>())
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Hand {
    cards: [Card; 5],
}

impl Debug for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for card in self.cards {
            f.write_char(card.0)?;
        }
        Ok(())
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            bail!("expected hand length 5, got length {}", s.len());
        }

        Ok(Self {
            cards: s.chars().fill_array().unwrap().map(Card),
        })
    }
}

impl Hand {
    /// returns the counts of the two most common cards in this hand: (greater, lower)
    #[instrument(level = "trace", ret)]
    fn hand_type<Part: AocPart>(&self) -> (usize, usize) {
        let mut card_counts: HashMap<Card, usize> = self.cards.into_iter().counts();

        if Part::is_two() {
            if let Some(j_count) = card_counts.remove(&Card('J')) {
                let Some((high_count_card, _)) = card_counts.iter().max_by_key(|(_, count)| *count)
                else {
                    return (5, 0);
                };

                *card_counts.entry(*high_count_card).or_insert(0) += j_count;
            }
        }

        let mut counts: Vec<usize> = card_counts.into_values().collect();
        counts.sort();

        (
            *counts
                .last()
                .expect("there is at least one card in the hand"),
            if counts.len() >= 2 {
                counts[counts.len() - 2]
            } else {
                0
            },
        )
    }

    #[instrument(level = "trace", ret)]
    fn cmp<Part: AocPart>(&self, other: &Self) -> cmp::Ordering {
        let type_ord = self.hand_type::<Part>().cmp(&other.hand_type::<Part>());
        let lexi_ord = iter::zip(self.cards, other.cards)
            .fold(Ordering::Equal, |ord, (c, d)| ord.then(c.cmp::<Part>(&d)));
        type_ord.then(lexi_ord)
    }
}

pub fn solve<Part: AocPart>(input: &str) -> u32 {
    let mut hands: Vec<(Hand, u32)> = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').expect("line should parse");
            let hand: Hand = hand.parse().expect("hand should parse");
            let bid: u32 = bid.parse().expect("bid should parse");
            (hand, bid)
        })
        .collect();

    hands.sort_by(|(l_hand, _), (r_hand, _)| l_hand.cmp::<Part>(r_hand));

    trace!("{hands:?}");

    hands
        .into_iter()
        .enumerate()
        .map(|(rank, (hand, bid))| (rank + 1) as u32 * bid)
        .sum()
}

const EXAMPLE_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

example_tests! {
    - part one:
        a0: EXAMPLE_INPUT => 6440,
    - part two:
        b0: EXAMPLE_INPUT => 5905,
}

#[test]
fn part_one() {
    let _ = aocutil::log::test_subscriber().try_init();
    assert_eq!(
        solve::<part::One>(&aocutil::get_input(YEAR, DAY)),
        248836197
    );
}

#[test]
fn part_two() {
    let _ = aocutil::log::test_subscriber().try_init();
    assert_eq!(
        solve::<part::Two>(&aocutil::get_input(YEAR, DAY)),
        251195607
    );
}
