use aocutil::prelude::*;

pub const YEAR: usize = 2023;

pub const DAY: usize = 25;

#[derive(Debug)]
struct Components<'a> {
    components: HashMap<&'a str, HashSet<&'a str>>,
}

impl<'a> Components<'a> {
    fn from_str(s: &'a str) -> Self {
        let mut components: HashMap<&'a str, HashSet<&'a str>> = HashMap::new();

        for line in s.lines() {
            let (name, neighbors) = line.split_once(":").expect("':' in line");

            components
                .entry(name)
                .or_default()
                .extend(neighbors.split_whitespace());

            for neighbor in neighbors.split_whitespace() {
                components.entry(neighbor).or_default().insert(name);
            }
        }

        Self { components }
    }

    fn remove_edge(&mut self, Edge(a, b): Edge) {
        self.components
            .get_mut(a)
            .expect("endpoint of edge should exist")
            .remove(b);

        self.components
            .get_mut(b)
            .expect("endpoint of edge should exist")
            .remove(a);
    }

    fn traverse_depth_first(&self, current: &'a str, visited: &mut HashSet<&'a str>) {
        if !visited.insert(current) {
            return;
        }

        for neighbor in &self.components[current] {
            self.traverse_depth_first(neighbor, visited);
        }
    }

    fn island_size(&self, start: &'a str) -> usize {
        let mut visited = HashSet::new();

        self.traverse_depth_first(start, &mut visited);

        visited.len()
    }
}

impl<'a> Graph for Components<'a> {
    type Distance = usize;

    type Node = &'a str;

    fn neighbors<'b>(
        &'b self,
        center: &'b Self::Node,
    ) -> impl Iterator<Item = (Self::Distance, Self::Node)> + 'b {
        self.components[center].iter().map(|&s| (1, s))
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Edge<'a>(&'a str, &'a str);

impl<'a> Edge<'a> {
    fn new(a: &'a str, b: &'a str) -> Self {
        if a <= b {
            Self(a, b)
        } else {
            Self(b, a)
        }
    }
}

pub fn solve<Part: AocPart>(input: &str) -> usize {
    let mut components = Components::from_str(input);

    debug!("{:#?}", components.components.len());

    let mut frequencies: HashMap<Edge, usize> = HashMap::new();

    for start_component in components.components.keys() {
        let (_dist, spanning_tree) =
            components.shortest_paths_dijkstra(start_component, |_| false, usize::cmp);

        for (&(mut succ), (_dist, mut pred)) in &spanning_tree {
            while let Some((_dist, new_pred)) = spanning_tree.get(pred) {
                *frequencies.entry(Edge::new(succ, pred)).or_insert(0) += 1;
                succ = pred;
                pred = new_pred;
            }
        }
    }

    let mut frequencies_sorted = frequencies.into_iter().collect_vec();
    frequencies_sorted.sort_by_key(|(_edge, count)| *count);

    let top_3 = frequencies_sorted.iter().rev().take(3);

    for (edge, _count) in top_3 {
        components.remove_edge(*edge);
    }

    let (Edge(l, r), _count) = frequencies_sorted.last().expect("at least one edge");

    components.island_size(l) * components.island_size(r)
}

aoc_tests! {
    inputs {
        e0 = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr",
    }

    part::One {
        ea0: e0 => 54,
        ra: @input => 589036,
    }

    part::Two {
        eb0: e0 => 0,
        rb: @input => 0,
    }
}
