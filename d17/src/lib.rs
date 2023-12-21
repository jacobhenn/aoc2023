use aocutil::prelude::*;

pub const YEAR: usize = 2023;

pub const DAY: usize = 17;

struct CrucibleGraph<P: AocPart + 'static> {
    blocks: Grid<u32>,
    _marker: PhantomData<P>,
}

// 235200

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct CrucibleNode {
    position: GridPos,
    direction: RookDirection,
    streak: usize,
}

impl<P: AocPart + 'static> Graph for CrucibleGraph<P> {
    type Node = CrucibleNode;

    type Distance = u32;

    type Neighbors<'a> = Box<dyn Iterator<Item = (u32, CrucibleNode)> + 'a>;

    fn neighbors<'a>(&'a self, center: &'a Self::Node) -> Self::Neighbors<'a> {
        Box::new(RookDirection::iter().filter_map(|d| {
            let streak_guard = match P::part() {
                Part::One => d == center.direction && center.streak == 3,
                Part::Two => match center.streak {
                    1..=3 => d != center.direction,
                    10.. => d == center.direction,
                    _ => false,
                },
            };

            if d == -center.direction || streak_guard {
                return None;
            };

            let distance = *self.blocks.get(center.position + d)?;

            Some((
                distance,
                CrucibleNode {
                    position: center.position + d,
                    direction: d,
                    streak: if d == center.direction {
                        center.streak + 1
                    } else {
                        1
                    },
                },
            ))
        }))
    }
}

pub fn solve<Part: AocPart + 'static>(input: &str) -> u32 {
    let blocks: Grid<u32> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("input should be all digits"))
        })
        .collect();

    let lower_right = v!(blocks.width() as isize - 1, blocks.height() as isize - 1);
    debug!("{:?}", lower_right);

    let graph = CrucibleGraph::<Part> {
        blocks,
        _marker: PhantomData,
    };

    let start = CrucibleNode {
        position: v!(0, 0),
        direction: RookDirection::PLUS_X,
        streak: 0,
    };

    let (end, shortest_paths) = graph.shortest_paths_dijkstra(start, |n| {
        n.position == lower_right && (Part::is_one() || n.streak >= 4)
    });

    let (path_length, _) = shortest_paths[&end.unwrap()];

    path_length
}

aoc_tests! {
    inputs {
        e0 = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",

        e1 = "111111111111
999999999991
999999999991
999999999991
999999999991",
    }

    part::One {
        ea0: e0 => 102,
        ra: @input => 866,
    }

    part::Two {
        eb0: e0 => 94,
        eb1: e1 => 71,
        rb: @input => 1010,
    }
}
