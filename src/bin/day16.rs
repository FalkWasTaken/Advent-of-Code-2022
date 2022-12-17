use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::{Index, IndexMut},
};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::i32,
    combinator::map,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

use utils::*;

type Flow = i32;
type Neighbors<'a> = HashMap<&'a str, Vec<(&'a str, u8)>>;

#[derive(Clone)]
struct Valve<'a> {
    id: &'a str,
    flow: Flow,
    opened: bool,
    neighbors: Vec<(&'a str, u8)>,
}

impl Valve<'_> {
    fn parse(s: &str) -> IResult<&str, Valve> {
        let (s, _) = tag("Valve ")(s)?;
        let (s, id) = Valve::parse_id(s)?;
        let (s, flow_rate) = preceded(tag(" has flow rate="), i32)(s)?;
        let (s, neighbors) = alt((
            preceded(
                tag("; tunnels lead to valves "),
                separated_list1(tag(", "), Valve::parse_id),
            ),
            preceded(
                tag("; tunnel leads to valve "),
                map(Valve::parse_id, |id: &str| vec![id]),
            ),
        ))(s)?;
        let valve = Valve {
            id,
            flow: flow_rate,
            neighbors: neighbors.into_iter().map(|n| (n, 1)).collect(),
            opened: false,
        };
        Ok((s, valve))
    }

    fn parse_id(s: &str) -> IResult<&str, &str> {
        take(2 as usize)(s)
    }
}

struct Graph<'a> {
    valves: HashMap<&'a str, Valve<'a>>,
}

impl<'a> Index<&str> for Graph<'a> {
    type Output = Valve<'a>;
    fn index(&self, index: &str) -> &Self::Output {
        &self.valves[index]
    }
}

impl<'a> IndexMut<&str> for Graph<'a> {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        self.valves.get_mut(index).unwrap()
    }
}

impl<'a> Graph<'a> {
    fn new(valves: Vec<Valve>) -> Graph {
        let valves = valves.into_iter().map(|v| (v.id, v)).collect();
        Graph { valves }
    }

    fn reset(&mut self) {
        self.valves.values_mut().for_each(|v| v.opened = false)
    }

    fn build_neighbor_matrix<'b>(&self) -> Neighbors<'b>
    where
        'a: 'b,
    {
        let mut res: Neighbors = HashMap::new();
        for &id in self.valves.keys() {
            let mut queue = VecDeque::from([(id, 0)]);
            let mut visited = HashSet::new();
            while let Some((id2, depth)) = queue.pop_front() {
                if visited.contains(id2) {
                    continue;
                }
                visited.insert(id2);
                if id2 != id {
                    res.entry(id).or_default().push((id2, depth));
                }
                for &(next, _) in &self[id2].neighbors {
                    if next != id {
                        queue.push_back((next, depth + 1));
                    }
                }
            }
        }
        for valve in self.valves.values().filter(|v| v.flow == 0) {
            if valve.id != "AA" {
                res.remove(valve.id);
            }
            res.values_mut()
                .for_each(|neighbors| neighbors.retain(|&(id, _)| id != valve.id));
        }
        res
    }

    fn max_flow(&mut self) -> Flow {
        let neighbors = &self.build_neighbor_matrix();
        self.max_flow_dfs("AA", 0, 29, neighbors)
    }

    fn max_flow_dfs(
        &mut self,
        from: &str,
        current_flow: Flow,
        time: u8,
        neighbors: &Neighbors,
    ) -> Flow {
        if time == 0 {
            return current_flow;
        }
        let mut max_flow = current_flow;
        for &(next, dt) in &neighbors[from] {
            if dt >= time || self[next].opened {
                continue;
            }
            self[next].opened = true;
            let new_flow = self[next].flow * (time - dt) as Flow;
            max_flow = max_flow.max(self.max_flow_dfs(
                next,
                current_flow + new_flow,
                time - dt - 1,
                neighbors,
            ));
            self[next].opened = false;
        }
        max_flow
    }

    fn max_flow2(&mut self) -> Flow {
        let neighbors = self.build_neighbor_matrix();
        self.max_flow2_dfs("AA", "AA", 0, 25, 25, &neighbors)
    }

    fn max_flow2_dfs(
        &mut self,
        from1: &str,
        from2: &str,
        current_flow: Flow,
        t1: u8,
        t2: u8,
        neighbors: &Neighbors,
    ) -> Flow {
        if t1 == 0 && t2 == 0 {
            current_flow
        } else if t1 == 0 {
            self.max_flow_dfs(from2, current_flow, t2, neighbors)
        } else if t2 == 0 {
            self.max_flow_dfs(from1, current_flow, t1, neighbors)
        } else {
            let mut max_flow = current_flow;
            let has_next = |from: &str, t| {
                neighbors[from]
                    .iter()
                    .any(|&(next, dt)| !self[next].opened && dt < t)
            };
            if !has_next(from1, t1) {
                max_flow = max_flow.max(self.max_flow_dfs(from2, current_flow, t2, neighbors));
            } else if !has_next(from2, t2) {
                max_flow = max_flow.max(self.max_flow_dfs(from1, current_flow, t1, neighbors));
            } else {
                for &(next1, dt1) in &neighbors[from1] {
                    if dt1 >= t1 || self[next1].opened {
                        continue;
                    }
                    self[next1].opened = true;
                    for &(next2, dt2) in &neighbors[from2] {
                        if dt2 >= t2 || self[next2].opened {
                            continue;
                        }
                        self[next2].opened = true;
                        let new_flow = self[next1].flow * (t1 - dt1) as Flow
                            + self[next2].flow * (t2 - dt2) as Flow;
                        max_flow = max_flow.max(self.max_flow2_dfs(
                            next1,
                            next2,
                            current_flow + new_flow,
                            t1 - dt1 - 1,
                            t2 - dt2 - 1,
                            neighbors,
                        ));
                        self[next2].opened = false;
                    }
                    self[next1].opened = false;
                }
            }
            max_flow
        }
    }
}

fn solve1(graph: &mut Graph) {
    let max_flow = graph.max_flow();
    println!("Solution to problem 1: {max_flow}");
}

fn solve2(mut graph: Graph) {
    let max_flow = graph.max_flow2();
    println!("Solution to problem 2: {max_flow}");
}

fn main() {
    let input = input!();
    let valves = input
        .lines()
        .flat_map(Valve::parse)
        .map(|t| t.1)
        .collect_vec();
    let mut graph = Graph::new(valves);
    solve1(&mut graph);
    graph.reset();
    solve2(graph);
}
