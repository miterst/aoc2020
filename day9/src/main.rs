fn main() {
    let xmas_numbers: Vec<i128> =
        include_str!("input")
            .lines()
            .map(str::parse)
            .collect::<Result<_, _>>()
            .unwrap();

    let fringe = find_fringe_number(&xmas_numbers, 25);
    println!("Part 1 sol: {}", fringe);
    println!("Part 2 sol: {}", find_encryption_weakness(&xmas_numbers, fringe));

}

fn find_fringe_number(xmas_numbers: &[i128], preamble_length: usize) -> i128 {
    *xmas_numbers.iter()
        .enumerate()
        .skip(preamble_length)
        .find(|(index, num)| {
            !contained_in_any_2sum(&xmas_numbers[index-preamble_length..*index], **num)
        })
        .unwrap()
        .1
}

fn contained_in_any_2sum(nums: &[i128], n: i128) -> bool {
    for i in 0..nums.len() - 1 {
        for j in i+1..nums.len() {
            if nums[i] + nums[j] == n && nums[i] != nums [j] {
                return true;
            }
        }
    }

    false
}

fn find_encryption_weakness(data: &[i128], fringe: i128) -> i128 {
    let mut i = 0;
    let mut j = 0;
    let mut s: i128 = 0;

    // there must be one
    while s != fringe {
        while s < fringe && j < data.len() - 1 {
            j += 1;
            s += data[j];
        }

        while s > fringe && i < j {
            i += 1;
            s -= data[i];
        }
    }

    data[i..j].iter().min().unwrap() + data[i..j].iter().max().unwrap()
}
