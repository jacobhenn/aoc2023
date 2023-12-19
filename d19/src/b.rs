use super::*;

#[derive(Clone)]
pub struct PartRange {
    rating_ranges: HashMap<char, Range<u64>>,
}

impl PartRange {
    fn empty() -> Self {
        Self {
            rating_ranges: HashMap::new(),
        }
    }

    pub fn full() -> Self {
        Self {
            rating_ranges: ['x', 'm', 'a', 's']
                .into_iter()
                .map(|c| (c, Range::new(1, 4000)))
                .collect(),
        }
    }

    fn volume(&self) -> u64 {
        self.rating_ranges
            .values()
            .map(|range| range.end - range.start + 1)
            .product()
    }
}

impl Filter {
    /// First element: the range of parts which satisfy both `part_range` and this filter
    /// Second element: the range of parts which satisfy `part_range` but not this filter
    fn constrain(&self, part_range: &PartRange) -> (PartRange, PartRange) {
        match self {
            Filter::Comparison {
                category,
                ordering,
                compare_to,
            } => {
                let rating_range = part_range.rating_ranges[category];
                let (mut sect, mut diff) = (part_range.clone(), part_range.clone());
                let (sect_range, diff_range) = match ordering {
                    Ordering::Less => (
                        Range::new(rating_range.start, compare_to - 1),
                        Range::new(*compare_to, rating_range.end),
                    ),
                    Ordering::Greater => (
                        Range::new(compare_to + 1, rating_range.end),
                        Range::new(rating_range.start, *compare_to),
                    ),
                    Ordering::Equal => panic!(),
                };

                sect.rating_ranges.insert(*category, sect_range);
                diff.rating_ranges.insert(*category, diff_range);

                (sect, diff)
            }
            Filter::Unconditional => (part_range.clone(), PartRange::empty()),
        }
    }
}

pub fn num_accepted(
    part_range: PartRange,
    workflow_name: &str,
    workflows: &HashMap<&str, Workflow>,
) -> u64 {
    let mut res = 0;

    let mut remaining = part_range;

    for rule in &workflows[workflow_name].rules {
        let (passing, new_remaining) = rule.filter.constrain(&remaining);

        match rule.destination {
            "A" => res += passing.volume(),
            "R" => (),
            dst => res += num_accepted(passing, dst, workflows),
        }

        remaining = new_remaining;
    }

    res
}
