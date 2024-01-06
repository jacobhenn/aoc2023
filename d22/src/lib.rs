use aocutil::prelude::*;

pub const YEAR: usize = 2023;

pub const DAY: usize = 22;

#[derive(Hash, Copy, Clone, PartialEq, Eq)]
struct Slab {
    range: MultiRange<i32, 3>,
    id: usize,
}

impl Debug for Slab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(c) = CAPITAL_LETTERS.get(self.id) {
            write!(f, "{c}")?;
        } else {
            write!(f, "{}", self.id)?;
        }

        write!(f, "{:?}", self.range)?;

        Ok(())
    }
}

/// `slabs` is always sorted by z. before falling, it is sorted by z start; after falling by z end
#[derive(Debug)]
struct Slabs {
    slabs: Vec<Slab>,
}

impl FromStr for Slabs {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut slabs: Vec<Slab> = s
            .lines()
            .enumerate()
            .map(|(id, line)| {
                let mut ints = ints(line);
                let start = Vector::new(ints.fill_array::<3>().context("should be 6 ints")?);
                let end = Vector::new(ints.fill_array::<3>().context("should be 6 ints")?);

                let range = MultiRange::from_corners(start, end);

                Ok(Slab { range, id })
            })
            .collect::<anyhow::Result<_>>()?;

        // sort by z start
        slabs.sort_by_key(|slab| slab.range[2].start);

        Ok(Self { slabs })
    }
}

impl Slabs {
    /// - The first thing returned is a map of slab ids to pairs of vectors. For each slab `x`,
    ///     - The first vector is the ids of all the slabs that support `x`.
    ///     - The second vector is the ids of all the slabs that `x` supports.
    /// - The second thing returned is the resultant structure of slabs after they have fallen,
    /// sorted by z end.
    fn fallen(self) -> (HashMap<usize, [Vec<usize>; 2]>, Self) {
        // fallen slabs sorted by z end
        let mut fallen_slabs: Vec<Slab> = Vec::new();

        let mut support_structure: HashMap<usize, [Vec<usize>; 2]> = HashMap::new();

        // iterate in order of z start
        for slab in self.slabs {
            let s = trace_span!("falling", ?slab);
            let _g = s.enter();

            let shadow = slab.range.with_component(2, slab.range[2].with_start(1));
            trace!("shadow: {shadow:?}");

            let mut under_slabs = fallen_slabs
                .iter()
                .rev()
                .filter(|slab| slab.range.intersects(&shadow));

            let first_collision = under_slabs.next();
            trace!("first collision: {first_collision:?}");

            // first, update the support structure

            if let Some(first_collision) = first_collision {
                let s = trace_span!("updating support structure");
                let _g = s.enter();

                for supporting_slab in iter::once(first_collision).chain(
                    under_slabs
                        .take_while(|slab| slab.range[2].end == first_collision.range[2].end),
                ) {
                    trace!("found supporting slab {supporting_slab:?}");

                    support_structure.entry(slab.id).or_default()[0].push(supporting_slab.id);
                    support_structure.entry(supporting_slab.id).or_default()[1].push(slab.id);
                }
            }

            // now, fall the slab

            let fall_z = first_collision.map_or(1, |slab| slab.range[2].end + 1);
            trace!("fall_z: {fall_z}");

            let z_diff = slab.range[2].start - fall_z;
            trace!("z_diff: {z_diff}");

            let fallen_slab = Slab {
                range: slab
                    .range
                    .with_component(2, slab.range[2].map(|z| z - z_diff)),
                ..slab
            };
            trace!("fallen_slab: {fallen_slab:?}");

            let i =
                fallen_slabs.partition_point(|slab| slab.range[2].end < fallen_slab.range[2].end);
            trace!("inserting at {i}");

            fallen_slabs.insert(i, fallen_slab);
            trace!("new fallen_slabs: {fallen_slabs:#?}");
        }

        (
            support_structure,
            Self {
                slabs: fallen_slabs,
            },
        )
    }

    fn count_disintegrable(&self, support_structure: &HashMap<usize, [Vec<usize>; 2]>) -> usize {
        self.slabs
            .iter()
            .filter(|slab| {
                let Some([_, slabs_above]) = support_structure.get(&slab.id) else {
                    return true;
                };

                slabs_above.iter().all(|above| {
                    let [slabs_below, _] = &support_structure[above];

                    slabs_below.len() > 1
                })
            })
            .count()
    }

    #[instrument(level = "trace", skip(support_structure), ret)]
    fn chain_reaction_size(
        support_structure: &HashMap<usize, [Vec<usize>; 2]>,
        visited: &mut HashSet<usize>,
        start: usize,
    ) -> usize {
        if !visited.insert(start) {
            return 0;
        }

        let Some([_, slabs_above]) = support_structure.get(&start) else {
            return 0;
        };

        slabs_above
            .iter()
            .map(|&above| {
                if visited.contains(&above) {
                    return 0;
                }

                let [slabs_below, _] = &support_structure[&above];

                let above_will_fall = slabs_below
                    .iter()
                    .filter(|below| !visited.contains(below))
                    .count()
                    == 0;

                if above_will_fall {
                    1 + Self::chain_reaction_size(support_structure, visited, above)
                } else {
                    0
                }
            })
            .sum()
    }
}

pub fn solve<P: AocPart>(input: &str) -> usize {
    let slabs = Slabs::from_str(input).expect("input is valid");

    debug!("parsed slabs: \n{slabs:#?}");

    let (support_structure, fallen_slabs) = slabs.fallen();

    debug!("fallen slabs: \n{fallen_slabs:#?}");
    debug!("support structure: \n{support_structure:#?}");

    match P::part() {
        Part::One => fallen_slabs.count_disintegrable(&support_structure),
        Part::Two => fallen_slabs
            .slabs
            .iter()
            .map(|slab| {
                Slabs::chain_reaction_size(&support_structure, &mut HashSet::new(), slab.id)
            })
            .sum(),
    }
}

aoc_tests! {
    inputs {
        e0 = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9",
    }

    part::One {
        ea0: e0 => 5,
        ra: @input => 430,
    }

    part::Two {
        eb0: e0 => 7,
        rb: @input => 60558,
    }
}
