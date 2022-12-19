use std::collections::VecDeque;
use std::fmt::Display;

use anyhow::anyhow;
use hashbrown::HashSet;
use nom::bytes::complete::tag;
use nom::character::complete::u64;
use nom::character::streaming::line_ending;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded, separated_pair, tuple};
use nom::IResult;
use rayon::prelude::*;

pub fn run(input: &str) -> anyhow::Result<(u64, u64)> {
    let blueprints = parse_blueprints(input)?;

    let part_1: u64 = blueprints
        .par_iter()
        .map(|blueprint| blueprint.id * max(blueprint, 24))
        .sum();

    let part_2: u64 = blueprints[0..3]
        .par_iter()
        .map(|blueprint| max(blueprint, 32))
        .product();

    Ok((part_1, part_2))
}

const ORE_MASK: u64 = 255;
const ORE_ROBOT_MASK: u64 = 255 << 8;
const CLAY_MASK: u64 = 255 << 16;
const CLAY_ROBOT_MASK: u64 = 255 << 24;
const OBSIDIAN_MASK: u64 = 255 << 32;
const OBSIDIAN_ROBOT_MASK: u64 = 255 << 40;
const GEODE_MASK: u64 = 255 << 48;
const GEODE_ROBOT_MASK: u64 = 255 << 56;

// Blueprint 1:
//   Each ore robot costs 4 ore.
//   Each clay robot costs 2 ore.
//   Each obsidian robot costs 3 ore and 14 clay.
//   Each geode robot costs 2 ore and 7 obsidian.
struct Blueprint {
    id: u64,
    ore_robot: u64,
    clay_robot: u64,
    obsidian_robot: u64,
    geode_robot: u64,
}

impl Blueprint {
    fn ore_spend(&self) -> u64 {
        (self.ore_robot & ORE_MASK)
            .max(self.clay_robot & ORE_MASK)
            .max(self.geode_robot & ORE_MASK)
    }

    fn clay_spend(&self) -> u64 {
        (self.obsidian_robot & CLAY_MASK) >> 16
    }

    fn obsidian_spend(&self) -> u64 {
        (self.geode_robot & OBSIDIAN_MASK) >> 32
    }
}

fn max(blueprint: &Blueprint, time_limit: u64) -> u64 {
    let mut visited: HashSet<(u64, Resources)> = HashSet::new();
    let resources = Resources::new();

    let ore_spend = blueprint.ore_spend();
    let clay_spend = blueprint.clay_spend();
    let obsidian_spend = blueprint.obsidian_spend();

    let mut stack: Vec<(u64, Resources)> = Vec::new();
    stack.push((0, resources));
    let mut max_geodes = 0;

    while let Some((time, resources)) = stack.pop() {
        let time_left = time_limit - time;
        let geode_count = resources.geode_count();

        max_geodes = max_geodes.max(resources.geode_count());

        if resources.ore_robot_count() > ore_spend
            || resources.obsidian_robot_count() > obsidian_spend
            || resources.clay_robot_count() > clay_spend
        {
            continue;
        }

        if visited.contains(&(time, resources)) {
            continue;
        }

        if geode_count + resources.geode_robot_count() * time_left + time_left * (time_left + 1) / 2
            < max_geodes
            && resources.obsidian_robot_count() <= obsidian_spend
        {
            continue;
        }

        visited.insert((time, resources));

        if time < time_limit {
            let next = resources.tick();
            stack.push((time + 1, next));

            let geode_robot = resources.build_geode_robot(blueprint);
            if !(geode_robot == next) && !visited.contains(&(time + 1, geode_robot)) {
                stack.push((time + 1, geode_robot));
            }

            let obsidian_robot = resources.build_obsidian_robot(blueprint);
            if !(obsidian_robot == next) && !visited.contains(&(time + 1, obsidian_robot)) {
                stack.push((time + 1, obsidian_robot));
            }

            let clay_robot = resources.build_clay_robot(blueprint);
            if !(clay_robot == next) && !visited.contains(&(time + 1, clay_robot)) {
                stack.push((time + 1, clay_robot));
            }

            let ore_robot = resources.build_ore_robot(blueprint);
            if !(ore_robot == next) && !visited.contains(&(time + 1, ore_robot)) {
                stack.push((time + 1, ore_robot));
            }
        }
    }

    max_geodes
}

#[allow(dead_code)]
fn max_queue(blueprint: &Blueprint) -> u64 {
    let mut visited: HashSet<Resources> = HashSet::new();
    let resources = Resources::new();

    let ore_spend = blueprint.ore_spend();
    let clay_spend = blueprint.clay_spend();
    let obsidian_spend = blueprint.obsidian_spend();

    dbg!(ore_spend);
    dbg!(clay_spend);
    dbg!(obsidian_spend);

    let mut queue: VecDeque<(u64, Resources)> = VecDeque::new();
    queue.push_back((0, resources));
    let time_limit = 24;
    let mut max_geodes = 0;

    while let Some((time, resources)) = queue.pop_front() {
        if resources.ore_robot_count() > ore_spend
            || resources.obsidian_robot_count() > obsidian_spend
            || resources.clay_robot_count() > clay_spend
        {
            continue;
        }

        if visited.contains(&resources) {
            continue;
        }

        let time_left = time_limit - time;
        let geode_count = resources.geode_count();

        max_geodes = max_geodes.max(resources.geode_count());

        if geode_count + time_left * (time_left + 1) / 2 < max_geodes {
            continue;
        }

        visited.insert(resources);

        if time < time_limit {
            let next = resources.tick();
            queue.push_back((time + 1, next));

            let ore_robot = resources.build_ore_robot(blueprint);
            if !(ore_robot == next) && !visited.contains(&ore_robot) {
                queue.push_back((time + 1, ore_robot));
            }

            let clay_robot = resources.build_clay_robot(blueprint);
            if !(clay_robot == next) && !visited.contains(&clay_robot) {
                queue.push_back((time + 1, clay_robot));
            }

            let obsidian_robot = resources.build_obsidian_robot(blueprint);
            if !(obsidian_robot == next) && !visited.contains(&obsidian_robot) {
                queue.push_back((time + 1, obsidian_robot));
            }

            let geode_robot = resources.build_geode_robot(blueprint);
            if !(geode_robot == next) && !visited.contains(&geode_robot) {
                queue.push_back((time + 1, geode_robot));
            }
        }
    }

    println!("{}", visited.len());

    visited.iter().map(Resources::geode_count).max().unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Resources(u64);

impl Resources {
    fn new() -> Self {
        Self(1 << 8)
    }

    fn tick(self) -> Self {
        let tick = (self.0
            & (ORE_ROBOT_MASK | CLAY_ROBOT_MASK | OBSIDIAN_ROBOT_MASK | GEODE_ROBOT_MASK))
            >> 8;

        Self(self.0 + tick)
    }

    fn build_ore_robot(mut self, blueprint: &Blueprint) -> Self {
        if (self.0 & ORE_MASK) >= (blueprint.ore_robot & ORE_MASK) {
            self.0 -= blueprint.ore_robot;
            let mut this = self.tick();
            this.0 += 1 << 8;
            this
        } else {
            self.tick()
        }
    }

    fn build_clay_robot(mut self, blueprint: &Blueprint) -> Self {
        if (self.0 & ORE_MASK) >= (blueprint.clay_robot & ORE_MASK) {
            self.0 -= blueprint.clay_robot;
            let mut this = self.tick();
            this.0 += 1 << 24;
            this
        } else {
            self.tick()
        }
    }

    fn build_obsidian_robot(mut self, blueprint: &Blueprint) -> Self {
        if (self.0 & ORE_MASK) >= (blueprint.obsidian_robot & ORE_MASK)
            && (self.0 & CLAY_MASK) >= (blueprint.obsidian_robot & CLAY_MASK)
        {
            self.0 -= blueprint.obsidian_robot;
            let mut this = self.tick();
            this.0 += 1 << 40;
            this
        } else {
            self.tick()
        }
    }

    fn build_geode_robot(mut self, blueprint: &Blueprint) -> Self {
        if (self.0 & ORE_MASK) >= (blueprint.geode_robot & ORE_MASK)
            && (self.0 & OBSIDIAN_MASK) >= (blueprint.geode_robot & OBSIDIAN_MASK)
        {
            self.0 -= blueprint.geode_robot;
            let mut this = self.tick();
            this.0 += 1 << 56;
            this
        } else {
            self.tick()
        }
    }

    fn ore_count(&self) -> u64 {
        self.0 & ORE_MASK
    }
    fn clay_count(&self) -> u64 {
        (self.0 & CLAY_MASK) >> 16
    }

    fn obsidian_count(&self) -> u64 {
        (self.0 & CLAY_MASK) >> 32
    }
    fn geode_count(&self) -> u64 {
        (self.0 & GEODE_MASK) >> 48
    }

    fn ore_robot_count(&self) -> u64 {
        (self.0 & ORE_ROBOT_MASK) >> 8
    }

    fn clay_robot_count(&self) -> u64 {
        (self.0 & CLAY_ROBOT_MASK) >> 24
    }

    fn obsidian_robot_count(&self) -> u64 {
        (self.0 & OBSIDIAN_ROBOT_MASK) >> 40
    }

    fn geode_robot_count(&self) -> u64 {
        (self.0 & GEODE_ROBOT_MASK) >> 56
    }
}

impl Display for Resources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Ore: {}", self.0 & ORE_MASK)?;
        writeln!(f, "Ore Robot: {}", (self.0 & ORE_ROBOT_MASK) >> 8)?;
        writeln!(f, "Clay: {}", (self.0 & CLAY_MASK) >> 16)?;
        writeln!(f, "Clay Robot: {}", (self.0 & CLAY_ROBOT_MASK) >> 24)?;
        writeln!(f, "Obsidian: {}", (self.0 & OBSIDIAN_MASK) >> 32)?;
        writeln!(
            f,
            "Obsidian Robot: {}",
            (self.0 & OBSIDIAN_ROBOT_MASK) >> 40
        )?;
        writeln!(f, "Geode: {}", (self.0 & GEODE_MASK) >> 48)?;
        writeln!(f, "Geode Robot: {}", (self.0 & GEODE_ROBOT_MASK) >> 56)
    }
}

// Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 9 clay. Each geode robot costs 3 ore and 9 obsidian.
fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    map(
        tuple((
            preceded(tag("Blueprint "), u64),
            delimited(tag(": Each ore robot costs "), u64, tag(" ore.")),
            delimited(tag(" Each clay robot costs "), u64, tag(" ore.")),
            delimited(
                tag(" Each obsidian robot costs "),
                separated_pair(u64, tag(" ore and "), u64),
                tag(" clay."),
            ),
            delimited(
                tag(" Each geode robot costs "),
                separated_pair(u64, tag(" ore and "), u64),
                tag(" obsidian."),
            ),
        )),
        |(id, ore_robot, clay_robot, obsidian_robot, geode_robot)| Blueprint {
            id,
            ore_robot,
            clay_robot,
            obsidian_robot: obsidian_robot.0 | (obsidian_robot.1 << 16),
            geode_robot: geode_robot.0 | (geode_robot.1 << 32),
        },
    )(input)
}

fn parse_blueprints(input: &str) -> anyhow::Result<Vec<Blueprint>> {
    let (_, blueprints) =
        separated_list0(line_ending, parse_blueprint)(input).map_err(|_| anyhow!("Parse error"))?;

    Ok(blueprints)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Blueprint 1:
    //   Each ore robot costs 4 ore.
    //   Each clay robot costs 2 ore.
    //   Each obsidian robot costs 3 ore and 14 clay.
    //   Each geode robot costs 2 ore and 7 obsidian.
    #[test]
    fn max_works() {
        let blueprint = Blueprint {
            id: 1,
            ore_robot: 4,
            clay_robot: 2,
            obsidian_robot: 3 | (14 << 16),
            geode_robot: 2 | (7 << 32),
        };

        assert_eq!(9, max(&blueprint, 24));
    }
}
