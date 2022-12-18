use anyhow::Context;
use nom::bytes::complete::tag;
use nom::character::complete::{i64, line_ending};
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;

pub fn run(input: &str) -> anyhow::Result<(i64, i64)> {
    let sensors = parse_sensors(input)?;

    let mut segments = Vec::new();
    covered_segments(&sensors, 2000000, &mut segments);
    let part_1 = total_extent(&segments);

    let beacon = find_beacon(
        &sensors,
        Coordinate {
            x: 4_000_000,
            y: 4_000_000,
        },
        &mut segments,
    )
    .context("No beacon found")?;
    let frequency = 4_000_000 * beacon.x + beacon.y;

    Ok((part_1, frequency))
}

fn total_extent(segments: &[(i64, i64)]) -> i64 {
    let mut extent = 0;
    let mut x = i64::MIN;

    for &(start, end) in segments {
        let start = x.max(start);
        extent += (end - start).max(0);
        x = x.max(end);
    }

    extent
}

fn find_beacon(
    sensors: &[(Coordinate, Coordinate)],
    upper_bound: Coordinate,
    segments_buffer: &mut Vec<(i64, i64)>,
) -> Option<Coordinate> {
    for y in 0..=upper_bound.y {
        let mut x = 0; // Lower bound for x
        covered_segments(sensors, y, segments_buffer);
        for &(start, end) in segments_buffer.iter() {
            if (start..=end).contains(&x) {
                x = end + 1;
            }
        }

        if x <= upper_bound.x {
            return Some(Coordinate { x, y });
        }
    }

    None
}

fn covered_segments(sensors: &[(Coordinate, Coordinate)], y: i64, segments: &mut Vec<(i64, i64)>) {
    segments.clear();

    for (sensor, beacon) in sensors {
        let d_beacon = sensor.manhattan(beacon);
        let d_line = sensor.manhattan(&Coordinate { x: sensor.x, y });

        if d_line <= d_beacon {
            let d = d_beacon - d_line;
            segments.push((sensor.x - d, sensor.x + d));
        }
    }

    segments.sort_unstable();
}

#[derive(Debug, PartialEq, Eq)]
struct Coordinate {
    x: i64,
    y: i64,
}

impl Coordinate {
    fn manhattan(&self, rhs: &Coordinate) -> i64 {
        (rhs.x - self.x).abs() + (rhs.y - self.y).abs()
    }
}

fn parse_sensors(input: &str) -> anyhow::Result<Vec<(Coordinate, Coordinate)>> {
    let (_, sensors) = many0(terminated(parse_sensor, opt(line_ending)))(input)
        .map_err(|_| anyhow::anyhow!("Parse error"))?;

    Ok(sensors)
}

// Sensor at x=2327144, y=3342616: closest beacon is at x=2445544, y=3467698
fn parse_sensor(input: &str) -> IResult<&str, (Coordinate, Coordinate)> {
    separated_pair(
        map(
            separated_pair(preceded(tag("Sensor at x="), i64), tag(", y="), i64),
            |(x, y)| Coordinate { x, y },
        ),
        tag(": closest beacon is at x="),
        map(separated_pair(i64, tag(", y="), i64), |(x, y)| Coordinate {
            x,
            y,
        }),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_INPUT: &str = include_str!("../inputs/day15_test.txt");

    #[test]
    fn parse_sensor_works() {
        let input = "Sensor at x=2327144, y=3342616: closest beacon is at x=2445544, y=3467698";
        let expected = (
            Coordinate {
                x: 2327144,
                y: 3342616,
            },
            Coordinate {
                x: 2445544,
                y: 3467698,
            },
        );

        assert_eq!(expected, parse_sensor(input).unwrap().1);
    }

    #[test]
    fn part_1_works() {
        let sensors = parse_sensors(TEST_INPUT).unwrap();

        let mut segments = Vec::new();
        covered_segments(&sensors, 10, &mut segments);

        assert_eq!(26, total_extent(&segments));
    }

    #[test]
    fn part_2_works() {
        let sensors = parse_sensors(TEST_INPUT).unwrap();

        let mut segments = Vec::new();
        let beacon = find_beacon(&sensors, Coordinate { x: 20, y: 20 }, &mut segments).unwrap();

        let frequency = 4_000_000 * beacon.x + beacon.y;

        assert_eq!(56000011, frequency);
    }
}
