// Note - Not my solution
// I tried to understand and reproduce solution in https://exercism.io/tracks/rust/exercises/dominoes/solutions/d6d2be34860c4b3e9a0ff628ea3f1197
// Still wrapping my head around the problem.
// The hardest part, for me to account for, was perhaps the duplicate nodes.


use std::collections::HashMap;
use std::cmp::max;

type Domino = (u8, u8);

struct Graph {
    edge_set: HashMap<Domino, u8>,
    map: [[bool; 256]; 256],
    max_node: u8,
}

impl Graph {
    pub fn new(input: &[(u8, u8)]) -> Self {
        let mut map = [[false; 256]; 256];
        let mut node_set = HashMap::new();
        let mut max_node = 0;

        input.iter().for_each(|(x, y)| {
            let (i, j) = (*x as usize, *y as usize);
            // add to map
            map[i][j] = true;
            map[j][i] = true;

            //add to node set
            *node_set.entry(Self::normalized((*x, *y))).or_insert(0) += 1;

            // set max node value
            max_node = max(max_node, max(*x, *y));
        });

        Graph {
            edge_set: node_set,
            map,
            max_node,
        }
    }

    pub fn make_chain(mut self) -> Option<Vec<Domino>> {
        for i in 1..=self.max_node {
            // println!("i: {:?}", i);
            let mut chain = vec![i];
            self.try_make_chain_from_node(i, &mut chain);

            // println!("Chain: {:?}", chain);

            if chain.len() > 1 && chain[0] == *chain.last().unwrap() {
                return Some(chain
                    .windows(2)
                    .map(|chunk| (chunk[0], chunk[1]))
                    .collect::<Vec<Domino>>());
            }
        }
        None
    }

    fn try_make_chain_from_node<'a>(&mut self, start: u8, chain: &'a mut Vec<u8>) -> Option<&'a mut Vec<u8>> {
        for i in 1..=self.max_node {
            let curr_edge = Self::normalized((start, i));
            if self.map[start as usize][i as usize] && self.edge_available(curr_edge) {
                // TODO: The edge should exist. But this seems wrong.
                *self.edge_set.entry(curr_edge).or_insert(0) -= 1;

                chain.push(i);

                if self.try_make_chain_from_node(i, chain).is_some() {
                    // Backtrack - return the current edge to the graph's edge set
                    *self.edge_set.entry(curr_edge).or_insert(0) += 1;
                    return Some(chain)
                } else {
                    // Backtrack - return the current edge to the graph's edge set
                    *self.edge_set.entry(curr_edge).or_insert(0) += 1;
                    chain.pop().unwrap();
                }
            }
        }

        let all_edges_used = self.edge_set.values().map(|x| *x).sum::<u8>() == 0u8;
        if all_edges_used {
            // We have a solution???
            // println!("Chain: {:?}", chain);
            return Some(chain);
        }

        None
    }

    fn edge_available(&self, edge: (u8, u8)) -> bool {
        *self.edge_set.get(&edge).unwrap() != 0u8
    }

    fn normalized(tuple: Domino) -> Domino {
        if tuple.0 > tuple.1 {
            return (tuple.1, tuple.0);
        }

        tuple
    }
}

pub fn chain(input: &[Domino]) -> Option<Vec<Domino>> {
    match input.len() {
        0 => Some(vec![]),
        _ => Graph::new(input).make_chain()
    }
}