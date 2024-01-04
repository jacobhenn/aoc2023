use aocutil::prelude::*;

pub const YEAR: usize = 2023;

pub const DAY: usize = 12;

#[derive(Clone, Debug, Default)]
struct Springs {
    len: usize,
    damaged: RangeSet<usize>,
    operational: RangeSet<usize>,
}

impl FromStr for Springs {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();

        let mut damaged = RangeSet::new();
        let mut operational = RangeSet::new();

        for (i, c) in s.chars().enumerate() {
            match c {
                '#' => {
                    damaged.insert(i);
                }
                '.' => {
                    operational.insert(i);
                }
                _ => (),
            }
        }

        Ok(Self {
            len,
            damaged,
            operational,
        })
    }
}

impl Springs {
    fn unfold(self) -> Self {
        let mut res = Self::default();

        res.len = (self.len * 5) + 4;

        for i in 0..5 {
            let offset = (self.len + 1) * i;

            for range in self.damaged.ranges() {
                res.damaged.insert(range.map(|x| x + offset));
            }

            for range in self.operational.ranges().cloned() {
                res.operational.insert(range.map(|x| x + offset));
            }
        }

        res
    }

    #[instrument(level = "trace", ret)]
    fn num_solutions(
        &self,
        start: usize,
        cur_run: usize,
        runs: &[usize],
        memory: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if let Some(prev) = memory.get(&(start, cur_run)) {
            return *prev;
        }

        if cur_run >= runs.len() {
            if self.damaged.intersects(&(start..=self.len)) {
                trace!("ran out of runs with unmatched left; 0 solutions");
                return 0;
            } else {
                trace!("no more runs; 1 solution");
                return 1;
            }
        }

        let first_run = runs[cur_run];

        let mut res = 0;

        for i in start..=(self.len - first_run) {
            let new_damaged_range = Range::new(i, i + first_run - 1);

            trace!("looking at {new_damaged_range:?}");

            // a run cannot contain a known operational spring
            if self.operational.intersects(&new_damaged_range) {
                trace!("  invalid: intersects operational spring");
                continue;
            }

            // a run cannot abut a known broken spring, or it would be recorded incorrectly
            if self.damaged.contains(&(new_damaged_range.end + 1)) {
                trace!("  invalid: abuts broken spring");
                continue;
            }

            // there cannot be another run before the match to the first one
            if new_damaged_range.start != 0
                && self
                    .damaged
                    .intersects(&Range::new(start, new_damaged_range.start - 1))
            {
                trace!("  invalid: passed unmatched broken range");
                break;
            }

            trace!("valid");

            res += self.num_solutions(new_damaged_range.end + 2, cur_run + 1, runs, memory);
        }

        memory.insert((start, cur_run), res);

        res
    }
}

pub fn solve<Part: AocPart>(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            debug!("done");
            let (springs, runs) = line.split_once(' ').expect("space in input");

            let mut springs = springs.parse::<Springs>().expect("valid springs");

            let mut runs: Vec<usize> = runs
                .split(',')
                .map(|r| r.parse::<usize>().expect("runs are numbers"))
                .collect();

            if Part::is_two() {
                springs = springs.unfold();

                let runs_len = runs.len();
                runs = runs.into_iter().cycle().take(runs_len * 5).collect();
            }

            let mut memory = HashMap::new();

            springs.num_solutions(0, 0, &runs, &mut memory)
        })
        .sum()
}

aoc_tests! {
    inputs {
        e0 = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
    }

    part::One {
        ea0: e0 => 21,
        ra: @input => 7843,
    }

    part::Two {
        eb0: e0 => 525152,
        rb: @input => 0,
    }
}
