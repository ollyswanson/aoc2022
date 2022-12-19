use anyhow::anyhow;
use nom::bytes::complete::tag;
use nom::character::complete::u16;
use nom::character::streaming::line_ending;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded, separated_pair, tuple};
use nom::IResult;
use rayon::prelude::*;

pub fn run(input: &str) -> anyhow::Result<(u32, u32)> {
    let blueprints = parse_blueprints(input)?;

    let part_1: u32 = blueprints
        .par_iter()
        .enumerate()
        .map(|(i, blueprint)| (i as u32 + 1) * max(blueprint, 24) as u32)
        .sum();

    let part_2: u32 = blueprints[0..3]
        .par_iter()
        .map(|blueprint| max(blueprint, 32) as u32)
        .product();

    Ok((part_1, part_2))
}

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

#[derive(Debug, Clone, Copy)]
struct RobotRecipe([u16; 4]);

// Blueprint 1:
//   Each ore robot costs 4 ore.
//   Each clay robot costs 2 ore.
//   Each obsidian robot costs 3 ore and 14 clay.
//   Each geode robot costs 2 ore and 7 obsidian.
struct Blueprint([RobotRecipe; 4]);

impl Blueprint {
    fn max_material_costs(&self) -> [u16; 4] {
        self.0.iter().fold([0; 4], |acc, recipe| {
            [
                acc[ORE].max(recipe.0[ORE]),
                acc[CLAY].max(recipe.0[CLAY]),
                acc[OBSIDIAN].max(recipe.0[OBSIDIAN]),
                u16::MAX, // Set to sentinel value
            ]
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Resources {
    robots: [u16; 4],
    materials: [u16; 4],
}

impl Resources {
    fn new() -> Self {
        Self {
            robots: [1, 0, 0, 0],
            materials: [0, 0, 0, 0],
        }
    }

    fn tick(mut self, amount: u16) -> Self {
        self.materials[ORE] += self.robots[ORE] * amount;
        self.materials[CLAY] += self.robots[CLAY] * amount;
        self.materials[OBSIDIAN] += self.robots[OBSIDIAN] * amount;
        self.materials[GEODE] += self.robots[GEODE] * amount;
        self
    }

    /// How many turns do we have to wait to build the given robot?
    fn wait(&self, idx: usize, blueprint: &Blueprint) -> Option<u16> {
        let costs = blueprint.0[idx];
        let mut max = 0;

        for cost in costs
            .0
            .iter()
            .enumerate()
            .filter(|(_, &cost)| cost > 0)
            .map(|(i, cost)| match self.robots[i] {
                0 => None,
                _ if self.materials[i] >= *cost => Some(0),
                n => Some((cost - self.materials[i] + n - 1) / n),
            })
        {
            let cost = cost?;
            max = max.max(cost);
        }

        Some(max)
    }

    fn build_robot(mut self, idx: usize, blueprint: &Blueprint) -> Self {
        let costs = blueprint.0[idx].0;
        self.materials[ORE] -= costs[ORE];
        self.materials[CLAY] -= costs[CLAY];
        self.materials[OBSIDIAN] -= costs[OBSIDIAN];
        self.robots[idx] += 1;
        self
    }
}

fn max(blueprint: &Blueprint, time_limit: u16) -> u16 {
    let resources = Resources::new();

    let max_material_costs = blueprint.max_material_costs();

    let mut stack: Vec<(u16, Resources)> = Vec::new();
    stack.push((0, resources));
    let mut max_geodes = 0;

    while let Some((time, resources)) = stack.pop() {
        let time_left = time_limit - time;

        max_geodes = max_geodes.max(resources.materials[GEODE]);

        if time_left > 0 {
            #[allow(clippy::needless_range_loop)]
            for idx in 0..=3 {
                if let Some(wait) = resources.wait(idx, blueprint) {
                    if wait >= time_left {
                        max_geodes = max_geodes
                            .max(resources.materials[GEODE] + resources.robots[GEODE] * time_left);
                    } else {
                        let resources = resources.tick(wait + 1);
                        let resources = resources.build_robot(idx, blueprint);
                        let time_left = time_left - wait - 1;

                        if resources.robots[idx] > max_material_costs[idx] {
                            continue;
                        }

                        if time_left > 0
                            && resources.materials[GEODE]
                                + resources.robots[GEODE] * time_left
                                + time_left * (time_left - 1) / 2
                                + 1
                                < max_geodes
                        {
                            continue;
                        }

                        stack.push((time + wait + 1, resources));
                    }
                }
            }
        }
    }

    max_geodes
}

// Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 9 clay. Each geode robot costs 3 ore and 9 obsidian.
fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    map(
        tuple((
            preceded(
                preceded(tag("Blueprint "), u16),
                delimited(tag(": Each ore robot costs "), u16, tag(" ore.")),
            ),
            delimited(tag(" Each clay robot costs "), u16, tag(" ore.")),
            delimited(
                tag(" Each obsidian robot costs "),
                separated_pair(u16, tag(" ore and "), u16),
                tag(" clay."),
            ),
            delimited(
                tag(" Each geode robot costs "),
                separated_pair(u16, tag(" ore and "), u16),
                tag(" obsidian."),
            ),
        )),
        |(ore_robot, clay_robot, obsidian_robot, geode_robot)| {
            Blueprint([
                RobotRecipe([ore_robot, 0, 0, 0]),
                RobotRecipe([clay_robot, 0, 0, 0]),
                RobotRecipe([obsidian_robot.0, obsidian_robot.1, 0, 0]),
                RobotRecipe([geode_robot.0, 0, geode_robot.1, 0]),
            ])
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

    #[test]
    fn max_works() {
        // Blueprint 1:
        //   Each ore robot costs 4 ore.
        //   Each clay robot costs 2 ore.
        //   Each obsidian robot costs 3 ore and 14 clay.
        //   Each geode robot costs 2 ore and 7 obsidian.
        let blueprint = Blueprint([
            RobotRecipe([4, 0, 0, 0]),
            RobotRecipe([2, 0, 0, 0]),
            RobotRecipe([3, 14, 0, 0]),
            RobotRecipe([2, 0, 7, 0]),
        ]);

        assert_eq!(9, max(&blueprint, 24));

        // Blueprint 2:
        //   Each ore robot costs 2 ore.
        //   Each clay robot costs 3 ore.
        //   Each obsidian robot costs 3 ore and 8 clay.
        //   Each geode robot costs 3 ore and 12 obsidian.
        let blueprint = Blueprint([
            RobotRecipe([2, 0, 0, 0]),
            RobotRecipe([3, 0, 0, 0]),
            RobotRecipe([3, 8, 0, 0]),
            RobotRecipe([3, 0, 12, 0]),
        ]);

        assert_eq!(12, max(&blueprint, 24));
    }
}
