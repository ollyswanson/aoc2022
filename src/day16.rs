use std::ops::{Index, IndexMut};
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, i32, line_ending};
use nom::multi::{many0, separated_list0};
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;

pub fn run(input: &str) -> anyhow::Result<(i32, i32)> {
    let cave: Cave<5> = input.parse()?;

    let part_1 = part_1(&cave);

    Ok((part_1, part_2(&cave)))
}

fn cave_run<const E: usize>(cave: &Cave<E>, time_limit: i32, starting_states: States) -> States {
    let state_space = starting_states.state_space;
    let valve_count = cave.valve_count();
    let mut states = starting_states;

    for minute in 0..time_limit {
        let mut next_states = States::new(valve_count, state_space);

        for idx in 0..valve_count {
            // Total flow from the valve from now until the time limit. Accounting for the fact
            // that it takes 1 minute to open the valve
            let valve = cave.get(idx);
            let total_flow = (time_limit - 1 - minute) * valve.flow;

            // If the state >= 0, i.e. != -1, then the runner is at this valve
            for state in 0..state_space {
                if states[idx][state] >= 0 {
                    // We want to set the state if the total flow is greater than 0 and the valve has
                    // not already been turned on
                    if total_flow > 0 && (valve.mask & state == 0) {
                        let next_state = valve.mask | state;
                        // Set the state to the current flow if it's bigger than the current flow
                        next_states[idx][next_state] =
                            next_states[idx][next_state].max(states[idx][state] + total_flow);
                    }

                    for &edge_idx in valve.get_edges() {
                        next_states[edge_idx][state] =
                            next_states[edge_idx][state].max(states[idx][state])
                    }
                }
            }
        }

        states = next_states;
    }

    states
}

fn part_1<const E: usize>(cave: &Cave<E>) -> i32 {
    let non_zero_flow_count = cave.non_zero_flow_count();
    let state_space = 1 << non_zero_flow_count;
    let valve_count = cave.valve_count();

    let mut states = States::new(valve_count, state_space);
    let aa = cave.index_of("AA");

    states[aa][0] = 0;

    let end_states = cave_run(cave, 30, states);

    end_states.states.into_iter().max().unwrap_or(0)
}

fn part_2<const E: usize>(cave: &Cave<E>) -> i32 {
    let non_zero_flow_count = cave.non_zero_flow_count();
    let state_space = 1 << non_zero_flow_count;
    let valve_count = cave.valve_count();

    let mut states = States::new(valve_count, state_space);
    let aa = cave.index_of("AA");
    // Time to teach elephant is 4 mins
    let time_limit = 26;

    states[aa][0] = 0;
    let intermediate_states = cave_run(cave, time_limit, states);

    let mut elephant_starting_states = States::new(valve_count, state_space);

    for state in 0..state_space {
        elephant_starting_states[aa][state] = intermediate_states
            .states
            .iter()
            .skip(state)
            .step_by(state_space)
            .max()
            .copied()
            .unwrap_or(0);
    }

    let end_states = cave_run(cave, time_limit, elephant_starting_states);

    end_states.states.into_iter().max().unwrap_or(0)
}

/// E is the max number of edges a valve might have
struct Cave<const E: usize> {
    names: Vec<String>,
    valves: Vec<Valve<E>>,
}

impl<const E: usize> Cave<E> {
    fn non_zero_flow_count(&self) -> usize {
        self.valves.iter().filter(|v| v.flow > 0).count()
    }

    #[inline]
    fn get(&self, idx: ValveIdx) -> &Valve<E> {
        &self.valves[idx]
    }

    fn valve_count(&self) -> usize {
        self.valves.len()
    }

    fn index_of<N: AsRef<str>>(&self, name: N) -> usize {
        self.names
            .iter()
            .position(|n| n == name.as_ref())
            .expect("vertex not found")
    }
}

#[derive(Debug)]
struct States {
    states: Vec<Flow>,
    /// The size of the state space for each valve
    state_space: usize,
}

impl States {
    fn new(valves: usize, state_space: usize) -> Self {
        Self {
            states: vec![-1; valves * state_space],
            state_space,
        }
    }
}

impl Index<usize> for States {
    type Output = [Flow];

    fn index(&self, index: usize) -> &Self::Output {
        let offset = index * self.state_space;
        &self.states[offset..offset + self.state_space]
    }
}

impl IndexMut<usize> for States {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let offset = index * self.state_space;
        &mut self.states[offset..offset + self.state_space]
    }
}

impl<const E: usize> FromStr for Cave<E> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        type Line<'a> = (&'a str, i32, Vec<&'a str>);

        fn parse_lines(input: &str) -> anyhow::Result<Vec<Line<'_>>> {
            let parse_result: IResult<&str, Vec<Line<'_>>> = many0(terminated(
                // Valve GS has flow rate=0; tunnels lead to valves KB, GW
                tuple((
                    preceded(tag("Valve "), alpha1),
                    preceded(tag(" has flow rate="), i32),
                    alt((
                        preceded(
                            tag("; tunnels lead to valves "),
                            separated_list0(tag(", "), alpha1),
                        ),
                        preceded(
                            tag("; tunnel leads to valve "),
                            separated_list0(tag(", "), alpha1),
                        ),
                    )),
                )),
                line_ending,
            ))(input);

            let (_, lines) = parse_result.map_err(|_| anyhow::anyhow!("Parse error"))?;

            Ok(lines)
        }

        let lines = parse_lines(s)?;
        let names: Vec<String> = lines.iter().map(|&(name, _, _)| name.to_owned()).collect();
        let mut valves: Vec<Valve<E>> = Vec::with_capacity(names.len());

        for (_, flow, edges) in lines {
            let mut valve = Valve::<E>::new(flow);
            for edge in edges {
                // This is inefficient, but because there are not many vertices in the graph it
                // should be ok.
                valve.edges.add_edge(
                    names
                        .iter()
                        .position(|e| e == edge)
                        .ok_or_else(|| anyhow::anyhow!("{} not found", edge))?,
                )
            }

            valves.push(valve);
        }

        for (i, v) in valves.iter_mut().filter(|v| v.flow > 0).enumerate() {
            v.mask = 1 << i;
        }

        Ok(Self { names, valves })
    }
}

type Flow = i32;

type ValveIdx = usize;

struct Valve<const E: usize> {
    edges: Edges<E>,
    flow: Flow,
    mask: usize,
}

impl<const E: usize> Valve<E> {
    fn new(flow: i32) -> Self {
        Self {
            edges: Edges::new(),
            flow,
            mask: 0,
        }
    }

    fn get_edges(&self) -> &[ValveIdx] {
        self.edges.get_edges()
    }
}

struct Edges<const E: usize> {
    count: usize,
    edges: [ValveIdx; E],
}

impl<const E: usize> Edges<E> {
    fn new() -> Self {
        Self {
            count: 0,
            edges: [0; E],
        }
    }

    fn add_edge(&mut self, idx: ValveIdx) {
        // Panic if we try to add more edges than we can hold
        assert!(self.count < E);
        self.edges[self.count] = idx;
        self.count += 1;
    }

    fn get_edges(&self) -> &[ValveIdx] {
        &self.edges[0..self.count]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = include_str!("../inputs/day16_test.txt");

    #[test]
    fn part_1_works() {
        let cave: Cave<5> = TEST_INPUT.parse().unwrap();

        assert_eq!(1651, part_1(&cave));
    }

    #[test]
    fn part_2_works() {
        let cave: Cave<5> = TEST_INPUT.parse().unwrap();

        assert_eq!(1707, part_2(&cave));
    }
}
