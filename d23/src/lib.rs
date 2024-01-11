use aocutil::prelude::*;

pub const YEAR: usize = 2023;

pub const DAY: usize = 23;

#[derive(Debug)]
struct ForestGraph {
    start_pos: GridPos,
    end_pos: GridPos,
    vertices: HashMap<GridPos, HashMap<GridPos, usize>>,
}

impl ForestGraph {
    fn from_grid<Part: AocPart>(grid: &Grid<char>) -> Self {
        let start_x = grid
            .rows()
            .next()
            .expect("grid is not empty")
            .iter()
            .position(|&c| c == '.')
            .expect("start tile exists");

        let end_x = grid
            .rows()
            .last()
            .expect("grid is not empty")
            .iter()
            .position(|&c| c == '.')
            .expect("start tile exists");

        let start_pos = v!(start_x as isize, 0);

        let end_pos = v!(end_x as isize, grid.height() as isize - 1);

        // build graph from grid

        let mut vertices = HashMap::new();

        vertices.insert(start_pos, HashMap::new());

        #[derive(Debug)]
        struct Probe {
            origin: GridPos,
            distance: usize,
            position: GridPos,
            direction: RookDirection,
        }

        let mut probes = VecDeque::new();

        probes.push_back(Probe {
            origin: start_pos,
            distance: 0,
            position: start_pos,
            direction: RookDirection::PLUS_Y,
        });

        while let Some(probe) = probes.pop_front() {
            let s = trace_span!("probe");
            let _g = s.enter();

            trace!("probe: {probe:?}");

            let neighbors: Vec<(GridPos, RookDirection)> = RookDirection::iter()
                .filter_map(|neighbor_dir| {
                    if neighbor_dir == -probe.direction {
                        return None;
                    }

                    if let Some(slope_dir) = RookDirection::from_ascii_arrow(grid[probe.position]) {
                        if Part::is_one() && neighbor_dir != slope_dir {
                            return None;
                        }
                    }

                    let neighbor_pos = probe.position + neighbor_dir;
                    let neighbor_char = *grid.get(neighbor_pos)?;

                    if neighbor_char == '#' {
                        return None;
                    }

                    Some((neighbor_pos, neighbor_dir))
                })
                .collect();

            trace!("neighbors: {neighbors:?}");

            if neighbors.is_empty() && probe.position != end_pos {
                // dead end
                continue;
            }

            if let [(neighbor_pos, neighbor_dir)] = neighbors[..] {
                let new_probe = Probe {
                    origin: probe.origin,
                    distance: probe.distance + 1,
                    position: neighbor_pos,
                    direction: neighbor_dir,
                };

                trace!("new probe: {new_probe:?}");

                probes.push_back(new_probe);

                continue;
            }

            // there are multiple new neighbors, meaning this is a vertex.

            let s = trace_span!("vertex");
            let _g = s.enter();

            // insert this node as a new outbound edge from this probe's origin

            trace!(
                "inserting {} <- ({} => {})",
                probe.origin,
                probe.position,
                probe.distance
            );

            vertices
                .get_mut(&probe.origin)
                .expect("origin should be inserted already")
                .insert(probe.position, probe.distance);

            let vertex_already_existed = vertices.contains_key(&probe.position);

            // if this is part 2, also insert an outbound edge from where *we* are connecting back
            if Part::is_two() {
                trace!(
                    "inserting {} <- ({} => {})",
                    probe.position,
                    probe.origin,
                    probe.distance
                );

                vertices
                    .entry(probe.position)
                    .or_default()
                    .insert(probe.origin, probe.distance);
            }

            // if we already exist, drop the probes for this branch and move on
            if vertex_already_existed {
                trace!("old node; continue");

                continue;
            }

            vertices.entry(probe.position).or_default();

            trace!("new node");

            for (neighbor_pos, neighbor_dir) in neighbors {
                let new_probe = Probe {
                    origin: probe.position,
                    distance: 1,
                    position: neighbor_pos,
                    direction: neighbor_dir,
                };

                trace!("new probe: {new_probe:?}");

                probes.push_back(new_probe);
            }
        }

        Self {
            start_pos,
            end_pos,
            vertices,
        }
    }

    fn longest_path_impl(&self, current: GridPos, visited: &mut HashSet<GridPos>) -> usize {
        trace!("finding longest path from {current} with visited={visited:?}");

        visited.insert(current);

        let res = self.vertices[&current]
            .iter()
            .filter_map(|(&neighbor_pos, &neighbor_distance)| {
                (!visited.contains(&neighbor_pos))
                    .then(|| neighbor_distance + self.longest_path_impl(neighbor_pos, visited))
            })
            .max()
            .unwrap_or(0);

        visited.remove(&current);

        res
    }

    fn longest_path(&self) -> usize {
        let mut visited = HashSet::new();

        self.longest_path_impl(self.start_pos, &mut visited)
    }
}

pub fn solve<Part: AocPart>(input: &str) -> usize {
    let grid = input.lines().map(str::chars).collect();

    let graph = ForestGraph::from_grid::<Part>(&grid);

    trace!("built graph: {graph:#?}");

    graph.longest_path()
}

aoc_tests! {
    inputs {
        e0 = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#",
    }

    part::One {
        ea0: e0 => 94,
        ra: @input => 2074,
    }

    part::Two {
        eb0: e0 => 154,
        rb: @input => 6494,
    }
}
