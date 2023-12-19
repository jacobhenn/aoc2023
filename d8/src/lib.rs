use aocutil::prelude::*;

pub const YEAR: usize = 2023;

pub const DAY: usize = 8;

fn solve_a(steps: &[Step], nodes: HashMap<Node, (Node, Node)>) -> usize {
    let mut node = Node::aaa();
    for (idx, step) in steps.iter().cycle().enumerate() {
        let (l, r) = nodes[&node];

        match step {
            Step::Left => node = l,
            Step::Right => node = r,
        }

        if node.is_zzz() {
            return idx + 1;
        }
    }

    unreachable!()
}

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
enum Step {
    Left,
    Right,
}

impl Step {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            other => panic!("step should be L or R, found {other}"),
        }
    }
}

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
struct Node {
    name: [char; 3],
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in self.name {
            f.write_char(c)?;
        }
        Ok(())
    }
}

impl FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            name: s
                .chars()
                .fill_array::<3>()
                .context("node names should be three letters")?,
        })
    }
}

impl Node {
    fn aaa() -> Self {
        Self {
            name: ['A', 'A', 'A'],
        }
    }

    fn is_zzz(&self) -> bool {
        self.name == ['Z', 'Z', 'Z']
    }

    fn is_z(&self) -> bool {
        self.name[2] == 'Z'
    }

    fn is_a(&self) -> bool {
        self.name[2] == 'A'
    }
}

/// All the information you need to store any infinite sequence of nodes, as they must repeat.
#[derive(Clone)]
struct Orbit {
    /// The first element of a pair represents the step that the corresponding node was visited on.
    nodes: Vec<(usize, Node)>,

    /// The index of the first node of the cyclic part. If you extended `nodes` to include one more,
    /// it would the same as this one.
    cycle_start: usize,

    /// The offset you must add to the steps in the cyclic part to get to the next cycle. If you
    /// extended `nodes` to include one more, this would be `nodes.last().0 - nodes[cycle_start].0`
    cycle_step: usize,
}

impl Display for Orbit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (t, node) in &self.nodes[..self.cycle_start] {
            write!(f, "(t={t} {node}) ")?;
        }

        write!(f, "r[")?;

        for (t, node) in &self.nodes[self.cycle_start..] {
            write!(f, "(t={t} {node}) ")?;
        }

        write!(f, "]")?;

        write!(f, " step: {}", self.cycle_step)?;

        Ok(())
    }
}

impl Orbit {
    fn cycle_length(&self) -> usize {
        self.nodes.len() - self.cycle_start
    }

    fn into_iter(&self) -> impl Iterator<Item = (usize, Node)> + '_ {
        self.nodes[0..self.cycle_start].iter().copied().chain(
            (0..)
                .map(move |cycle| {
                    self.nodes[self.cycle_start..]
                        .iter()
                        .copied()
                        .map(move |(step, node)| (step + cycle * self.cycle_step, node))
                })
                .flatten(),
        )
    }

    /// Fast-forward to the nth entry in the sequence (not the value at the nth global step).
    fn nth(&self, n: usize) -> (usize, Node) {
        if n < self.cycle_step {
            self.nodes[n]
        } else {
            let (t, node) =
                self.nodes[self.cycle_start + (n - self.cycle_start) % self.cycle_length()];

            (
                t + self.cycle_step * ((n - self.cycle_start) / self.cycle_length()),
                node,
            )
        }
    }

    /// Takes as input:
    /// - the sequence of global L/R steps
    /// - an iterator of pairs of un-modded step numbers and nodes
    #[instrument(level = "trace", skip_all)]
    fn from_iter<'a>(steps: &[Step], nodes_iter: impl Iterator<Item = (usize, Node)>) -> Self {
        // TMP: this is just for debugging purposes
        let mut prev_max_t = 0;

        // keys are pairs of modded step numbers and nodes. values are indices of `nodes` that
        // these occurred on.
        let mut history: HashMap<(usize, Node), usize> = HashMap::new();

        // just a collection of items from `nodes` - step numbers remain unmodded.
        let mut nodes: Vec<(usize, Node)> = Vec::new();

        for (t, node) in nodes_iter {
            trace!("got (t={t} {node})");

            if t < prev_max_t {
                panic!("t should be strictly increasing");
            } else {
                prev_max_t = t;
            }

            if let Some(cycle_start) = history.insert((t % steps.len(), node), nodes.len()) {
                trace!("found cycle; start={cycle_start}");

                return Self {
                    cycle_step: t - nodes[cycle_start].0,
                    nodes,
                    cycle_start,
                };
            }

            nodes.push((t, node));
        }

        unreachable!("sequence should be infinite")
    }
}

fn iterate_sequence<'a>(
    steps: &'a [Step],
    nodes: &'a HashMap<Node, (Node, Node)>,
    mut node: Node,
) -> impl Iterator<Item = (usize, Node)> + 'a {
    steps.iter().cycle().enumerate().map(move |(t, step)| {
        let res = (t, node);

        let &(l, r) = nodes.get(&node).expect("node should be in map");
        match step {
            Step::Left => node = l,
            Step::Right => node = r,
        }

        res
    })
}

fn solve_b(steps: &[Step], nodes: &HashMap<Node, (Node, Node)>) -> usize {
    let mut a_node_orbits: HashMap<Node, Orbit> = HashMap::new();

    // STEP 1: find the orbits of all A nodes so that finding their nth iterates is trivial

    for &a_node in nodes.keys().filter(|node| node.is_a()) {
        let orbit = Orbit::from_iter(steps, iterate_sequence(steps, nodes, a_node));

        debug!("node {a_node}: {orbit}");

        a_node_orbits.insert(a_node, orbit);
    }

    // STEP 2: look at the orbits of successive A nodes under the sequences of Z nodes of the
    // previous node.

    let mut a_nodes = nodes.keys().filter(|node| node.is_a());

    let first_a_node = *a_nodes.next().expect("there is at least one A node");
    trace!("starting with {first_a_node}");

    let mut current_orbit = a_node_orbits[&first_a_node].clone();

    for a_node in a_nodes {
        trace!("looking at {a_node}");
        // trace!("current orbit: {current_orbit}");

        let a_node_orbit = &a_node_orbits[a_node];

        trace!(
            "current_orbit cycle_start: {}, cycle_step: {}",
            current_orbit.cycle_start,
            current_orbit.cycle_step
        );

        current_orbit = Orbit::from_iter(
            steps,
            current_orbit
                .into_iter()
                .filter(|(t, node)| {
                    if node.is_z() {
                        trace!("found z node in current orbit: t={t} {node}");
                    }
                    node.is_z()
                })
                .map(|(t, _)| {
                    let (u, res) = a_node_orbit.nth(t);
                    trace!("{t}th {a_node} iterate: t={u} {res}");
                    (u, res)
                }),
        );
    }

    // STEP 3: find the first Z node in the remaining orbit

    let (first_z_step, _) = current_orbit
        .into_iter()
        .find(|(_, node)| node.is_z())
        .expect("orbits should be infinite");

    first_z_step
}

pub fn solve<Part: AocPart>(input: &str) -> usize {
    let mut lines = input.lines();

    let steps: Vec<Step> = lines.next().unwrap().chars().map(Step::from_char).collect();

    let nodes: HashMap<Node, (Node, Node)> = lines
        .skip(1)
        .map(|line| {
            unlist!(
                line.split(|c: char| !c.is_ascii_alphanumeric())
                    .filter_map(|s| s.parse::<Node>().ok()),
                name,
                l,
                r
            );
            (name, (l, r))
        })
        .collect();

    if Part::is_one() {
        solve_a(&steps, nodes)
    } else {
        solve_b(&steps, &nodes)
    }
}

example_tests! {
    - part one:
        a0: "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)" => 2,
        a1: "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)" => 6,
    - part two:
        b0: "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)" => 6,
}

#[test]
fn part_one() {
    let _ = aocutil::test_logger().try_init();
    assert_eq!(solve::<part::One>(&aocutil::get_input(YEAR, DAY)), 23147);
}

#[test]
fn part_two() {
    let _ = aocutil::test_logger().try_init();
    assert_eq!(
        solve::<part::Two>(&aocutil::get_input(YEAR, DAY)),
        22289513667691
    );
}
