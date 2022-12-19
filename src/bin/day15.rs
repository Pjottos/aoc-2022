use aoc_2022::*;

use std::{iter, ops::Range};

fn main() {
    Harness::begin()
        .day(15)
        //        .input_override(
        //            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        //Sensor at x=9, y=16: closest beacon is at x=10, y=16
        //Sensor at x=13, y=2: closest beacon is at x=15, y=3
        //Sensor at x=12, y=14: closest beacon is at x=10, y=16
        //Sensor at x=10, y=20: closest beacon is at x=10, y=16
        //Sensor at x=14, y=17: closest beacon is at x=10, y=16
        //Sensor at x=8, y=7: closest beacon is at x=2, y=10
        //Sensor at x=2, y=0: closest beacon is at x=2, y=10
        //Sensor at x=0, y=11: closest beacon is at x=2, y=10
        //Sensor at x=20, y=14: closest beacon is at x=25, y=17
        //Sensor at x=17, y=20: closest beacon is at x=21, y=22
        //Sensor at x=16, y=7: closest beacon is at x=15, y=3
        //Sensor at x=14, y=3: closest beacon is at x=15, y=3
        //Sensor at x=20, y=1: closest beacon is at x=15, y=3
        //",
        //        )
        .extract(|text| {
            let mut bytes = text.as_bytes().iter().copied().rev();
            bytes.next();
            iter::from_fn(move || {
                let mut sensor = Sensor::default();
                #[inline]
                fn parse_num(bytes: &mut impl Iterator<Item = u8>) -> Option<i32> {
                    let mut b = bytes.next()?;
                    let mut num = 0;
                    for multiplier in [1, 10, 100, 1000, 10000, 100000, 1000000] {
                        let digit = b.wrapping_sub(b'0');
                        if digit > 9 {
                            break;
                        }
                        num += digit as i32 * multiplier;
                        b = bytes.next().unwrap();
                    }
                    if b == b'-' {
                        num = -num;
                        bytes.next();
                    }
                    Some(num)
                }

                sensor.beacon_y = parse_num(&mut bytes)?;
                bytes.nth(2);
                sensor.beacon_x = parse_num(&mut bytes)?;
                bytes.nth(23);
                sensor.y = parse_num(&mut bytes)?;
                bytes.nth(2);
                sensor.x = parse_num(&mut bytes)?;
                bytes.nth(11);

                Some(sensor)
            })
        })
        .run_part(1, |sensors| {
            let target_y = 2_000_000;

            let mut beacons_on_target = vec![];
            let mut ranges = sensors
                .filter_map(|sensor| {
                    let row_extent = sensor.range() - (target_y - sensor.y).abs();
                    (row_extent >= 0).then(|| {
                        if sensor.beacon_y == target_y {
                            beacons_on_target.push(sensor.beacon_x);
                        }
                        let start = sensor.x - row_extent;
                        let end = sensor.x + row_extent + 1;
                        start..end
                    })
                })
                .collect::<Vec<_>>();

            beacons_on_target.sort_unstable();
            beacons_on_target.dedup();

            ranges.sort_unstable_by(|a, b| a.start.cmp(&b.start).then(a.end.cmp(&b.end)));
            let mut ranges = ranges.into_iter();

            let mut last = ranges.next().unwrap();
            let mut sum = -(beacons_on_target.len() as i32);
            for range in ranges {
                if last.end >= range.start {
                    last.end = last.end.max(range.end);
                } else {
                    sum += last.end - last.start;
                    last = range;
                }
            }
            sum += last.end - last.start;

            sum
        })
        .run_part(2, |sensors| {
            let x_bounds = 0..4_000_000;
            let y_bounds = 0..4_000_000;
            let sensors = sensors.map(|s| (s.x, s.y, s.range())).collect::<Vec<_>>();

            let mut positive_lines = vec![];
            let mut negative_lines = vec![];
            for &(x, y, range) in &sensors {
                positive_lines.push(LineSegment::new(x - (range + 1), y, x, y + range + 1));
                positive_lines.push(LineSegment::new(x, y - (range + 1), x + range + 1, y));
                negative_lines.push(LineSegment::new(x - (range + 1), y, x, y - (range + 1)));
                negative_lines.push(LineSegment::new(x, y + range + 1, x + range + 1, y));
            }

            for (j, positive) in positive_lines.iter().enumerate() {
                // Skip negative lines belonging to the same diamond as the current positive lines, since
                // they will always intersect on the corners of the diamond
                for i in (0..negative_lines.len()).filter(|&i| i != j && i != j + 1) {
                    let negative = &negative_lines[i];

                    let intersect_x = (negative.b - positive.b) / 2;
                    if !x_bounds.contains(&intersect_x) {
                        continue;
                    }
                    if positive.domain.contains(&intersect_x)
                        && negative.domain.contains(&intersect_x)
                    {
                        let intersect_y = intersect_x + positive.b;
                        if !y_bounds.contains(&intersect_y) {
                            continue;
                        }
                        if !sensors
                            .iter()
                            .any(|&(x, y, range)| distance(x, y, intersect_x, intersect_y) <= range)
                        {
                            //println!("{intersect_x}, {intersect_y}");
                            return i64::from(intersect_x) * 4_000_000 + i64::from(intersect_y);
                        }
                    }
                }
            }

            panic!("no distress beacon found")
        });
}

#[derive(Debug, Default)]
struct Sensor {
    x: i32,
    y: i32,
    beacon_x: i32,
    beacon_y: i32,
}

impl Sensor {
    #[inline]
    fn range(&self) -> i32 {
        distance(self.x, self.y, self.beacon_x, self.beacon_y)
    }
}

#[inline]
fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x2 - x1).abs() + (y2 - y1).abs()
}

struct LineSegment {
    domain: Range<i32>,
    b: i32,
}

impl LineSegment {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        let a = if y1 > y2 { -1 } else { 1 };
        let b = y1 - a * x1;

        Self {
            domain: x1..x2 + 1,
            b,
        }
    }
}
