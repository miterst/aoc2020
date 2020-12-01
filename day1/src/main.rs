use std::cmp::Ordering;

// O(n)
fn two_sum(numbers: &[i32], wanted: i32) -> Option<(i32, i32)> {
    let mut i = 0;
    let mut j = numbers.len() - 1;

    while i < j {
        let sum = numbers[i] + numbers[j];

        match (sum).cmp(&wanted) {
            Ordering::Less => {
                i += 1;
            }
            Ordering::Equal => {
                return Some((numbers[i], numbers[j]));
            }
            Ordering::Greater => {
                j -= 1;
            }
        }
    }

    None
}

// O(n^2)
fn three_sum(numbers: &[i32], wanted: i32) -> Option<(i32, i32, i32)> {
    for (index, i) in numbers.iter().enumerate() {
        let n = wanted - i;
        if let Some((j, k)) = two_sum(&numbers[index..], n) {
            return Some((*i, j, k));
        }
    }

    None
}

fn main() {
    let mut numbers: Vec<i32> = include_str!("input")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect();

    numbers.sort();

    let (i, j) = two_sum(&numbers, 2020).unwrap();
    println!("{}+{}=2020 | {0}*{1}={}", i, j, i * j);

    let (i, j, k) = three_sum(&numbers, 2020).unwrap();
    println!("{}+{}+{}=2020 | {0}*{1}*{2}={}", i, j, k, i * j * k);
}
