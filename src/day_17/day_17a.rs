use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub(super) enum Access {
    Horizontal,
    Vertical
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Node {
    i: usize,
    j: usize,
    orientation: Access,
}

struct NodeCost {
    n: Node,
    cost: i32,
}

impl Ord for NodeCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for NodeCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl PartialEq for NodeCost {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for NodeCost {}

fn dijkstra(city_map: &Vec<Vec<i32>>) {
    let mut visited = HashSet::<Node>::new();
    let mut frontier = HashM
    let mut queue = BinaryHeap::<&NodeCost>::new();

    let start_nodes = vec![
        Node {
            i: 0,
            j: 0,
            orientation: Access::Vertical,
        },
        Node {
            i: 0,
            j: 0,
            orientation: Access::Horizontal,
        },
    ];
    start_nodes.iter().for_each(|n| {
        queue.push(NodeCost {
            n: n.clone(),
            cost: city_map[0][0],
        });
    });

    let target1 = Node {
        i: city_map.len(),
        j: city_map[0].len(),
        orientation: Access::Horizontal,
    };
    let target2 = Node {
        i: city_map.len(),
        j: city_map[0].len(),
        orientation: Access::Vertical,
    };
    let curr = queue
        .pop()
        .unwrap_or_else(|| panic!("did not find a start node"));
    while curr.n != target1 && curr.n != target2 {
        let neighbors = neighbors(&curr, &city_map);
        neighbors.into_iter().for_each(|(node, step_cost)| {
            if visited.contains(&node) {
                return;
            }
            let cost = curr.cost + step_cost;
        })
    }
}

fn neighbors(node: &NodeCost, city_map: &Vec<Vec<i32>>) -> Vec<(Node, i32)> {
    let mut neighbors = vec![];
    match node.n.orientation {
        Access::Horizontal => {
            let north_south_pos = node.n.i as isize;
            let mut n_cost = 0;
            let mut s_cost = 0;
            for ns_offset in 1..=3 {
                if north_south_pos - ns_offset >= 0 {
                    let neighbor = Node {
                        i: (north_south_pos - ns_offset) as usize,
                        j: node.n.j,
                        orientation: Access::Vertical,
                    };
                    n_cost += city_map[neighbor.i][neighbor.j];
                    neighbors.push((neighbor, n_cost));
                }
                if north_south_pos + ns_offset < city_map.len() as isize {
                    let neighbor = Node {
                        i: (north_south_pos + ns_offset) as usize,
                        j: node.n.j,
                        orientation: Access::Vertical,
                    };
                    s_cost += city_map[neighbor.i][neighbor.j];
                    neighbors.push((neighbor, s_cost));
                }
            }
        }
        Access::Vertical => {
            let east_west_pos = node.n.j as isize;
            let mut w_cost = 0;
            let mut e_cost = 0;
            for ew_offset in 1..=3 {
                if east_west_pos - ew_offset >= 0 {
                    let neighbor = Node {
                        i: node.n.i as usize,
                        j: (east_west_pos - ew_offset) as usize,
                        orientation: Access::Horizontal,
                    };
                    w_cost += city_map[neighbor.i][neighbor.j];
                    neighbors.push((neighbor, w_cost));
                }
                if east_west_pos + ew_offset < city_map[0].len() as isize {
                    let neighbor = Node {
                        i: node.n.i as usize,
                        j: (east_west_pos + ew_offset) as usize,
                        orientation: Access::Horizontal,
                    };
                    e_cost += city_map[neighbor.i][neighbor.j];
                    neighbors.push((neighbor, e_cost));
                }
            }
        }
    }
    neighbors
}
