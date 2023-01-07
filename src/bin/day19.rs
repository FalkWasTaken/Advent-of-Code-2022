use std::{
    ops::{Index, IndexMut},
    sync::Mutex,
};

use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, u32},
    sequence::{pair, preceded, terminated},
    IResult,
};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use utils::*;
use Resource::*;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct ResourceMap<T> {
    ore: T,
    clay: T,
    obsidian: T,
    geode: T,
}

impl<T> Index<Resource> for ResourceMap<T> {
    type Output = T;
    fn index(&self, index: Resource) -> &Self::Output {
        match index {
            Ore => &self.ore,
            Clay => &self.clay,
            Obsidian => &self.obsidian,
            Geode => &self.geode,
        }
    }
}

impl<T> IndexMut<Resource> for ResourceMap<T> {
    fn index_mut(&mut self, index: Resource) -> &mut Self::Output {
        match index {
            Ore => &mut self.ore,
            Clay => &mut self.clay,
            Obsidian => &mut self.obsidian,
            Geode => &mut self.geode,
        }
    }
}

impl<'a, T> IntoIterator for &'a ResourceMap<T> {
    type IntoIter = ResourceIter<'a, T>;
    type Item = (Resource, &'a T);
    fn into_iter(self) -> Self::IntoIter {
        ResourceIter {
            arr: [
                (Geode, &self.geode),
                (Obsidian, &self.obsidian),
                (Clay, &self.clay),
                (Ore, &self.ore),
            ],
            index: 0,
        }
    }
}

struct ResourceIter<'a, T> {
    arr: [(Resource, &'a T); 4],
    index: usize,
}

impl<'a, T> Iterator for ResourceIter<'a, T> {
    type Item = (Resource, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        if self.index - 1 < 4 {
            Some(self.arr[self.index - 1])
        } else {
            None
        }
    }
}

#[derive(Clone)]
struct Blueprint {
    id: u32,
    robot_costs: ResourceMap<Vec<(Resource, u32)>>,
    robots: ResourceMap<u32>,
    resources: ResourceMap<u32>,
}

fn parse1<'a>(s: &'a str, robot_type: &'a str) -> IResult<&'a str, u32> {
    preceded(
        tag(format!("Each {robot_type} robot costs ").as_str()),
        terminated(u32, pair(tag(" ore."), multispace0)),
    )(s)
}

fn parse2<'a>(
    s: &'a str,
    robot_type: &'a str,
    r1: &'a str,
    r2: &'a str,
) -> IResult<&'a str, (u32, u32)> {
    let (s, r1_cost) = preceded(
        tag(format!("Each {robot_type} robot costs ").as_str()),
        terminated(u32, tag(format!(" {r1}").as_str())),
    )(s)?;
    let (s, r2_cost) = preceded(
        tag(" and "),
        terminated(u32, pair(tag(format!(" {r2}.").as_str()), multispace0)),
    )(s)?;
    IResult::Ok((s, (r1_cost, r2_cost)))
}

impl Blueprint {
    fn parse(s: &str) -> IResult<&str, Blueprint> {
        let (s, id) = preceded(
            pair(tag("Blueprint"), multispace1),
            terminated(u32, pair(tag(":"), multispace0)),
        )(s)?;
        let (s, ore_cost) = parse1(s, "ore")?;
        let (s, clay_cost) = parse1(s, "clay")?;
        let (s, (obs_cost_ore, obs_cost_clay)) = parse2(s, "obsidian", "ore", "clay")?;
        let (s, (geode_cost_ore, geode_cost_obs)) = parse2(s, "geode", "ore", "obsidian")?;
        let mut robot_costs = ResourceMap::default();
        robot_costs[Ore] = vec![(Ore, ore_cost)];
        robot_costs[Clay] = vec![(Ore, clay_cost)];
        robot_costs[Obsidian] = vec![(Ore, obs_cost_ore), (Clay, obs_cost_clay)];
        robot_costs[Geode] = vec![(Ore, geode_cost_ore), (Obsidian, geode_cost_obs)];
        let mut robots = ResourceMap::default();
        robots[Ore] = 1;
        let resources = ResourceMap::default();
        Ok((
            s,
            Blueprint {
                id,
                robot_costs,
                robots,
                resources,
            },
        ))
    }

    fn available_robots(&self) -> Vec<Resource> {
        [Geode, Obsidian, Clay, Ore]
            .into_iter()
            .filter(|&t| self.can_produce(t))
            .collect()
    }

    fn can_produce(&self, robot_type: Resource) -> bool {
        self.robot_costs[robot_type]
            .iter()
            .all(|&(r, num)| self.resources[r] >= num)
    }

    fn collect_resources(&mut self) {
        for (robot_type, num) in &self.robots {
            self.resources[robot_type] += num;
        }
    }

    fn max_potential(&self, time: u32) -> u32 {
        self.resources[Geode] + self.robots[Geode] * time + (time - 1) * (time) / 2
    }

    fn max_geodes(&self, time: u32) -> u32 {
        self.clone().max_geo_rec(time, 0)
    }

    fn max_geo_rec(&mut self, time: u32, max: u32) -> u32 {
        if time == 0 {
            return self.resources[Geode];
        } else if self.max_potential(time) <= max {
            return 0;
        };
        let new = self.available_robots();
        self.collect_resources();
        let backup = self.resources.clone();
        let mut res = 0;
        for robot_type in new {
            self.robots[robot_type] += 1;
            for &(r, num) in &self.robot_costs[robot_type] {
                self.resources[r] -= num;
            }
            res = res.max(self.max_geo_rec(time - 1, res.max(max)));
            self.robots[robot_type] -= 1;
            self.resources = backup.clone();
        }
        res = res.max(self.max_geo_rec(time - 1, res.max(max)));
        self.resources = backup.clone();
        res
    }
}

fn solve1(blueprints: &Vec<Blueprint>) {
    let progress = Mutex::new(0);
    let res: u32 = blueprints
        .par_iter()
        .map(|b| (b.id, b.max_geodes(24)))
        .inspect(|(id, geodes)| {
            *progress.lock().unwrap() += 1;
            println!(
                "id: {id}\tmax_geodes: {geodes}\tprogress: {}/{}",
                progress.lock().unwrap(),
                blueprints.len()
            )
        })
        .map(|(id, geodes)| id * geodes)
        .sum();
    println!("Solution to problem 1: {res}");
}

fn solve2(blueprints: &Vec<Blueprint>) {
    let res: u32 = blueprints[0..3]
        .par_iter()
        .map(|b| (b.id, b.max_geodes(32)))
        .inspect(|(id, geodes)| println!("id: {id}, max geodes: {geodes}"))
        .map(|(_, geodes)| geodes)
        .product();
    println!("Solution to problem 2: {res}");
}

fn main() {
    let blueprints = input!()
        .lines()
        .flat_map(Blueprint::parse)
        .map(|res| res.1)
        .collect();
    solve1(&blueprints);
    solve2(&blueprints);
}
