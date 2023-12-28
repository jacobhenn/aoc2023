use aocutil::prelude::*;

pub const YEAR: usize = 2023;

pub const DAY: usize = 15;

fn holiday_hash(input: &str) -> usize {
    input.chars().fold(0, |h, c| ((h + c as usize) * 17) % 256)
}

pub fn solve<Part: AocPart>(input: &str) -> usize {
    let steps = input.split(',');

    if Part::is_one() {
        return steps.map(holiday_hash).sum();
    }

    let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];

    for step in steps {
        let (label, focal_length) = step.split_once(['-', '=']).unwrap();
        let hash = holiday_hash(label);

        if focal_length.is_empty() {
            boxes[hash].retain(|(l, _)| l != &label);
        } else {
            let focal_length = focal_length.parse::<usize>().unwrap();
            if let Some((_, f)) = boxes[hash].iter_mut().find(|(l, _)| l == &label) {
                *f = focal_length;
            } else {
                boxes[hash].push((label, focal_length));
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(i, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(move |(j, (_, f))| (1 + i) * (1 + j) * f)
        })
        .flatten()
        .sum()
}

aoc_tests! {
    inputs {
        e0 = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7",
    }

    part::One {
        ea0: e0 => 1320,
        ra: @input => 506269,
    }

    part::Two {
        eb0: e0 => 145,
        rb: @input => 264021,
    }
}
