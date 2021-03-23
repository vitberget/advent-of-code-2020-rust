use std::collections::HashSet;
use std::time::Instant;

static PUZZLE: &'static str = include_str!("../../resources/puzzle3.txt");

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct XY {
    x: u32,
    y: u32,
}

pub(crate) fn day3() {
    println!("Day 3");

    let (trees, size) = mine_resources();

    part1(&trees, &size);
    part2(&trees, &size);
}

fn mine_resources() -> (HashSet<XY>, XY) {
    let start_time = Instant::now();
    let trees = resource_to_trees();
    let size = resource_to_size();

    println!("Resource extraction, {}µs", start_time.elapsed().as_micros());
    (trees, size)
}

fn part1(trees: &HashSet<XY>, size: &XY) {
    let start_time = Instant::now();
    let part_1_result = calc_hits(&trees, &size, &XY { x: 3, y: 1 });

    println!("Day 3, part1: {}, took {}µs",
             part_1_result,
             start_time.elapsed().as_micros());
}

fn part2(trees: &HashSet<XY>, size: &XY) {
    let start_time = Instant::now();
    let part_2_result = [
        XY { x: 1, y: 1 },
        XY { x: 3, y: 1 },
        XY { x: 5, y: 1 },
        XY { x: 7, y: 1 },
        XY { x: 1, y: 2 }
    ].iter()
        .map(|step| calc_hits(&trees, &size, step))
        .into_iter()
        .fold(1, |a, v| a * v);

    println!("Day 3, part2: {}, took {}µs",
             part_2_result,
             start_time.elapsed().as_micros())
}

fn calc_hits(trees: &HashSet<XY>, size: &XY, step: &XY) -> u32 {
    let mut res: u32 = 0;
    for y in (0..=size.y).step_by(step.y as usize) {
        let current = XY { x: (step.x * y) % size.x, y: y };
        if trees.contains(&current) {
            res = res + 1;
        }
    }
    return res;
}

fn resource_to_size() -> XY {
    return XY {
        x: PUZZLE.lines().next().unwrap().len() as u32,
        y: PUZZLE.lines().count() as u32,
    };
}

fn resource_to_trees() -> HashSet<XY> {
    return PUZZLE.lines()
        .enumerate()
        .map(|(y, line)| line_to_trees(y as u32, line))
        .flatten()
        .collect();
}

fn line_to_trees(y: u32, line: &str) -> HashSet<XY> {
    return line.chars()
        .enumerate()
        .map(|(x, c)| maybe_tree(x as u32, y, c))
        .filter(|item| item.is_some())
        .map(|item| item.unwrap())
        .collect();
}

fn maybe_tree(x: u32, y: u32, c: char) -> Option<XY> {
    return if c == '#' {
        Some(XY { x, y })
    } else {
        None
    }
}