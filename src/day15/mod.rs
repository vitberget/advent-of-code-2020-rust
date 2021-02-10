use std::iter::Map;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;

static PUZZLE: &'static str = include_str!("../../resources/puzzle15.txt");

pub(crate) fn day15(target_turn: i32) -> i32 {
    let puzzle: Vec<i32> =
        PUZZLE.split(",")
        .map(|w| w.parse().unwrap())
        .collect();

    let mut word_map =
        puzzle[0..puzzle.len() - 1]
            .to_vec()
            .into_iter()
            .enumerate()
            .map(|(i, w)| (w, i as i32))
            .fold(HashMap::new(),
                  |mut map, (k, v)| {
                      map.insert(k, v);
                      map
                  });

    let mut current = puzzle.last().copied().unwrap();

    for index in word_map.len() as i32..target_turn - 1 {
        current =  word_map.insert(current, index)
            .map(|n| index - n)
            .or_else(|| Option::from(0))
            .unwrap();
    }

    return current;
}
