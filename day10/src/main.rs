use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

fn main() {
    let jolts: HashSet<i32> = include_str!("input")
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();

    let mut path = vec![];
    let max = *jolts.iter().max().unwrap();

    find_path(&jolts, max, &mut path);
    let ones = path.iter().filter(|c| **c == 1).count();
    let threes = path.iter().filter(|c| **c == 3).count() + 1;
    println!("{}", ones * threes);

    println!("{}", count_all_paths(&jolts, max, &mut HashMap::new()));
}

fn find_path(jolts: &HashSet<i32>, highest_jolt: i32, path: &mut Vec<i32>) -> bool {
    for i in 1..=3 {
        path.push(i);

        let next_highest_jolt = highest_jolt - i;

        if next_highest_jolt == 0 {
            return true;
        }

        if jolts.contains(&next_highest_jolt) && find_path(jolts, next_highest_jolt, path) {
            return true;
        }

        path.pop();
    }

    false
}

fn count_all_paths(
    jolts: &HashSet<i32>,
    highest_jolt: i32,
    cache: &mut HashMap<i32, i128>,
) -> i128 {
    (1..=3)
        .map(|i| {
            let next_highest_jolt = highest_jolt - i;

            match next_highest_jolt.cmp(&0) {
                Ordering::Equal => 1,
                _ if jolts.contains(&next_highest_jolt) => {
                    if cache.contains_key(&next_highest_jolt) {
                        *cache.get(&next_highest_jolt).unwrap()
                    } else {
                        let paths_count = count_all_paths(jolts, next_highest_jolt, cache);
                        cache.insert(next_highest_jolt, paths_count);
                        paths_count
                    }
                }
                _ => 0,
            }
        })
        .sum()
}
