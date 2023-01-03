use std::{
    collections::HashMap,
    ops::{Index, IndexMut}, sync::Mutex,
};

use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, u32},
    sequence::{pair, preceded, terminated},
    IResult,
};

use rand::prelude::random;

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use utils::*;
use Resource::*;

const TEST: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, Ord)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Resource {
    fn priority(&self) -> u8 {
        match self {
            Ore => 3,
            Clay => 2,
            Obsidian => 1,
            Geode => 0,
        }
    }
}

impl PartialOrd for Resource {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.priority().partial_cmp(&other.priority())
    }
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

impl<T> From<[(Resource, T); 4]> for ResourceMap<T> {
    fn from(mut arr: [(Resource, T); 4]) -> Self {
        arr.sort_by_key(|t| t.0);
        match arr {
            [(Geode, geode), (Obsidian, obsidian), (Clay, clay), (Ore, ore)] => ResourceMap {
                ore,
                clay,
                obsidian,
                geode,
            },
            _ => panic!(),
        }
    }
}

impl<T> ResourceMap<T> {
    fn iter(&self) -> ResourceIter<T> {
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

impl<T: Ord> PartialOrd for ResourceMap<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let ge = |f: fn(&ResourceMap<T>) -> &T| f(&self) > f(other);
        let le = |f: fn(&ResourceMap<T>) -> &T| f(&self) < f(other);
        let eq = |f: fn(&ResourceMap<T>) -> &T| f(&self) == f(other);
        if ge(|r| &r.ore) && ge(|r| &r.clay) && ge(|r| &r.obsidian) && ge(|r| &r.geode) {
            Some(std::cmp::Ordering::Greater)
        } else if eq(|r| &r.ore) && eq(|r| &r.clay) && eq(|r| &r.obsidian) && eq(|r| &r.geode) {
            Some(std::cmp::Ordering::Equal)
        } else if le(|r| &r.ore) && le(|r| &r.clay) && le(|r| &r.obsidian) && le(|r| &r.geode) {
            Some(std::cmp::Ordering::Less)
        } else {
            None
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

#[derive(Clone, Debug)]
struct Cacher {
    seeds: ResourceMap<u32>,
    cache: HashMap<u32, ResourceMap<u32>>,
}

impl Cacher {
    fn new() -> Cacher {
        Cacher {
            seeds: [
                (Ore, random()),
                (Clay, random()),
                (Obsidian, random()),
                (Geode, random()),
            ]
            .into(),
            cache: HashMap::new(),
        }
    }

    fn cache_robots(&self, robots: &ResourceMap<u32>, time: u32) -> u32 {
        let hash = |resource| self.seeds[resource] ^ robots[resource];
        hash(Ore) ^ hash(Clay) ^ hash(Obsidian) ^ hash(Geode) ^ time
    }

    fn get(&self, k: &u32) -> Option<&ResourceMap<u32>> {
        self.cache.get(k)
    }

    fn insert(&mut self, k: u32, v: ResourceMap<u32>) {
        self.cache.insert(k, v);
    }
}

#[derive(Debug, Clone)]
struct Blueprint {
    id: u32,
    robot_costs: ResourceMap<Vec<(Resource, u32)>>,
    robots: ResourceMap<u32>,
    resources: ResourceMap<u32>,
    cache: Cacher,
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
        let robot_costs = [
            (Ore, vec![(Ore, ore_cost)]),
            (Clay, vec![(Ore, clay_cost)]),
            (Obsidian, vec![(Ore, obs_cost_ore), (Clay, obs_cost_clay)]),
            (
                Geode,
                vec![(Ore, geode_cost_ore), (Obsidian, geode_cost_obs)],
            ),
        ]
        .into();
        let robots = [(Ore, 1), (Clay, 0), (Obsidian, 0), (Geode, 0)].into();
        let resources = [(Ore, 0), (Clay, 0), (Obsidian, 0), (Geode, 0)].into();
        Ok((
            s,
            Blueprint {
                id,
                robot_costs,
                robots,
                resources,
                cache: Cacher::new(),
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

    fn construct_robot(&mut self, robot_type: Resource) {
        self.robots[robot_type] += 1;
        for &(r, num) in &self.robot_costs[robot_type] {
            self.resources[r] -= num;
        }
    }

    fn destroy_robot(&mut self, robot_type: Resource) {
        self.robots[robot_type] -= 1;
    }

    fn max_geodes(&self, time: u32) -> u32 {
        self.clone().max_geo_rec(time)
    }

    fn collect_resources(&mut self) {
        for (robot_type, num) in self.robots.iter() {
            self.resources[robot_type] += num;
        }
    }

    fn backup_resources(&mut self) -> ResourceMap<u32> {
        self.resources.clone()
    }

    fn revert_backup(&mut self, backup: &ResourceMap<u32>) {
        self.resources = backup.clone();
    }

    fn max_geo_rec(&mut self, time: u32) -> u32 {
        if time == 0 {
            return self.resources[Geode];
        }
        let robot_hash = self.cache.cache_robots(&self.robots, time);
        if let Some(resources) = self.cache.get(&robot_hash) {
            if *resources >= self.resources {
                return 0;
            } else {
                self.cache.insert(robot_hash, self.resources.clone());
            }
        } else {
            self.cache.insert(robot_hash, self.resources.clone());
        }
        let new = self.available_robots();
        self.collect_resources(); 
        let backup = self.backup_resources();
        let mut res = 0;
        if new.contains(&Geode) {
            self.construct_robot(Geode);
            res = self.max_geo_rec(time - 1);
            self.destroy_robot(Geode);
        } else {
            for robot_type in [Obsidian, Ore, Clay] {
                if !new.contains(&robot_type) {
                    continue;
                }
                self.construct_robot(robot_type);
                res = res.max(self.max_geo_rec(time - 1));
                self.destroy_robot(robot_type);
                self.revert_backup(&backup);
            }
            res = res.max(self.max_geo_rec(time - 1));
        }
        self.revert_backup(&backup);
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
            println!("id: {id}\tmax_geodes: {geodes}\tprogress: {}/{}", progress.lock().unwrap(), blueprints.len())
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
    //println!("{blueprints:?}");
    solve1(&blueprints);
    solve2(&blueprints);
}
