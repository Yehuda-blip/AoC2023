use anyhow::{anyhow, Context, Result};
use petgraph::{algo::dijkstra, graphmap::DiGraphMap};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub(super) enum Access {
    Horizontal,
    Vertical,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Node {
    i: usize,
    j: usize,
    orientation: Access,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ordering::Equal)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}

pub(super) fn min_cost<const MIN_STEP: isize, const MAX_STEP: isize>(city_map: &Vec<Vec<u32>>) -> u32 {
    let nodes = city_map.iter().enumerate().flat_map(|(i, row)| {
        row.iter().enumerate().flat_map(move |(j, _)| {
            [
                Node {
                    i,
                    j,
                    orientation: Access::Vertical,
                },
                Node {
                    i,
                    j,
                    orientation: Access::Horizontal,
                },
            ]
        })
    });
    let edges = nodes.flat_map(|from_vertex| {
        neighbors::<MIN_STEP, MAX_STEP>(&from_vertex, city_map)
            .into_iter()
            .map(move |(to_vertex, weight)| (from_vertex, to_vertex, weight))
    }).collect::<Vec<(Node, Node, u32)>>();
    let mut graph = DiGraphMap::<Node, u32>::from_edges(edges.into_iter());

    // this shit confuses me too...
    // because the neighbors function computes cost of traversing to the neighbor, without the cost
    // of the source vertex, the way 'edges' computes means that the first vertex (more precisely,
    // the first two vertices) has no weight.
    // for this reason we add the virtual node with usize::MAX coordinates on the map as the start
    // node that lead to city_map[0][0].
    // edit: apparently we don't add the first step. I left this added start node so that I would 
    // be able to run the dijkstra only once.
    graph.add_edge(
        Node {
            i: usize::MAX,
            j: usize::MAX,
            orientation: Access::Horizontal,
        },
        Node {
            i: 0,
            j: 0,
            orientation: Access::Vertical,
        },
        0,
    );
    graph.add_edge(
        Node {
            i: usize::MAX,
            j: usize::MAX,
            orientation: Access::Horizontal,
        },
        Node {
            i: 0,
            j: 0,
            orientation: Access::Horizontal,
        },
        0,
    );

    let costs = dijkstra(
        &graph,
        Node {
            i: usize::MAX,
            j: usize::MAX,
            orientation: Access::Horizontal,
        },
        None,
        |e| *e.2,
    );
    let target1 = Node {
        i: city_map.len() - 1,
        j: city_map[0].len() - 1,
        orientation: Access::Horizontal,
    };
    let target2 = Node {
        i: city_map.len() - 1,
        j: city_map[0].len() - 1,
        orientation: Access::Vertical,
    };
    return *costs
        .get(&target1)
        .unwrap_or(&u32::MAX)
        .min(costs.get(&target2).unwrap_or(&u32::MAX));
}

fn neighbors<const MIN_STEP: isize, const MAX_STEP: isize>(node: &Node, city_map: &Vec<Vec<u32>>) -> Vec<(Node, u32)> {
    let mut neighbors = vec![];
    match node.orientation {
        Access::Horizontal => {
            let north_south_pos = node.i as isize;
            let mut n_cost = 0;
            let mut s_cost = 0;
            for ns_offset in 1..=MAX_STEP {
                if north_south_pos - ns_offset >= 0 {
                    let neighbor = Node {
                        i: (north_south_pos - ns_offset) as usize,
                        j: node.j,
                        orientation: Access::Vertical,
                    };
                    n_cost += city_map[neighbor.i][neighbor.j];
                    if ns_offset >= MIN_STEP {
                        neighbors.push((neighbor, n_cost));
                    }
                }
                if north_south_pos + ns_offset < city_map.len() as isize {
                    let neighbor = Node {
                        i: (north_south_pos + ns_offset) as usize,
                        j: node.j,
                        orientation: Access::Vertical,
                    };
                    s_cost += city_map[neighbor.i][neighbor.j];
                    if ns_offset >= MIN_STEP {
                        neighbors.push((neighbor, s_cost));
                    }
                }
            }
        }
        Access::Vertical => {
            let east_west_pos = node.j as isize;
            let mut w_cost = 0;
            let mut e_cost = 0;
            for ew_offset in 1..=MAX_STEP {
                if east_west_pos - ew_offset >= 0 {
                    let neighbor = Node {
                        i: node.i as usize,
                        j: (east_west_pos - ew_offset) as usize,
                        orientation: Access::Horizontal,
                    };
                    w_cost += city_map[neighbor.i][neighbor.j];
                    if ew_offset >= MIN_STEP {
                        neighbors.push((neighbor, w_cost));
                    }
                }
                if east_west_pos + ew_offset < city_map[0].len() as isize {
                    let neighbor = Node {
                        i: node.i as usize,
                        j: (east_west_pos + ew_offset) as usize,
                        orientation: Access::Horizontal,
                    };
                    e_cost += city_map[neighbor.i][neighbor.j];
                    if ew_offset >= MIN_STEP {
                        neighbors.push((neighbor, e_cost));
                    }
                }
            }
        }
    }
    neighbors
}

pub fn solve(input: &String) -> Result<String> {
    let city_map = input
        .lines()
        .map(|line| {
            itertools::process_results(
                line.chars()
                    .map(|c| Ok(c.to_digit(10).context(anyhow!("not a digit"))?)),
                |it| it.collect(),
            )
        })
        .collect::<Result<Vec<Vec<u32>>>>()?;

    let val = min_cost::<1, 3>(&city_map);
    Ok(val.to_string())
}
