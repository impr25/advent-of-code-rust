use crate::util::hash::*;
use crate::util::iter::*;
use crate::util::parse::*;
use crate::util::point::*;
use std::ops::Range;

pub struct Input {
    sensor: Point,
    beacon: Point,
    manhattan: i32,
}

pub fn parse(input: &str) -> Vec<Input> {
    fn helper([x1, y1, x2, y2]: [i32; 4]) -> Input {
        let sensor = Point { x: x1, y: y1 };
        let beacon = Point { x: x2, y: y2 };
        let manhattan = sensor.manhattan(beacon);
        Input { sensor, beacon, manhattan }
    }
    input.iter_signed().chunk::<4>().map(helper).collect()
}

pub fn part1(input: &[Input]) -> i32 {
    part1_testable(input, 2_000_000)
}

pub fn part1_testable(input: &[Input], row: i32) -> i32 {
    fn build_range(input: &Input, row: i32) -> Option<Range<i32>> {
        let Input { sensor, manhattan, .. } = input;
        let extra = manhattan - (sensor.y - row).abs();
        (extra >= 0).then(|| (sensor.x - extra)..(sensor.x + extra))
    }

    fn build_beacons(input: &Input, row: i32) -> Option<i32> {
        let Input { beacon, .. } = input;
        (beacon.y == row).then_some(beacon.x)
    }

    let mut ranges: Vec<_> = input.iter().filter_map(|i| build_range(i, row)).collect();
    ranges.sort_unstable_by(|a, b| a.start.cmp(&b.start));

    let mut total = 0;
    let mut max = i32::MIN;

    for Range { start, end } in ranges {
        if start > max {
            total += end - start + 1;
            max = end;
        } else {
            total += (end - max).max(0);
            max = max.max(end);
        }
    }

    let beacons: FastSet<_> = input.iter().filter_map(|i| build_beacons(i, row)).collect();
    total - (beacons.len() as i32)
}

pub fn part2(input: &[Input]) -> u64 {
    part2_testable(input, 4_000_000)
}

pub fn part2_testable(input: &[Input], size: i32) -> u64 {
    let mut top = FastSet::new();
    let mut left = FastSet::new();
    let mut bottom = FastSet::new();
    let mut right = FastSet::new();

    // Rotate points clockwise by 45 degrees, scale by √2 and extend edge by 1.
    // This transform each sensor into an axis aligned bounding box.
    // The distress beacon is located where the top, left, bottom and right
    // edges of 4 separate bounding boxes intersect.
    for Input { sensor, manhattan, .. } in input {
        top.insert(sensor.x + sensor.y - manhattan - 1);
        left.insert(sensor.x - sensor.y - manhattan - 1);
        bottom.insert(sensor.x + sensor.y + manhattan + 1);
        right.insert(sensor.x - sensor.y + manhattan + 1);
    }

    let horizontal: Vec<_> = top.intersection(&bottom).collect();
    let vertical: Vec<_> = left.intersection(&right).collect();
    let range = 0..(size + 1);

    for x in &vertical {
        for y in &horizontal {
            // Rotate intersection point counter clockwise and scale by 1 / √2
            // to return to original coordinates.
            let point = Point { x: (**x + **y) / 2, y: (**y - **x) / 2 };
            if range.contains(&point.x)
                && range.contains(&point.y)
                && input.iter().all(|i| i.sensor.manhattan(point) > i.manhattan)
            {
                return 4_000_000 * (point.x as u64) + (point.y as u64);
            }
        }
    }

    unreachable!()
}
