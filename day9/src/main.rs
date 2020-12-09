fn main() {
    let xmas_numbers: Vec<u128> =
        include_str!("input")
            .lines()
            .map(str::parse)
            .collect::<Result<_, _>>()
            .unwrap();

    let fringe = find_fringe_number(&xmas_numbers, 25);
    println!("Part 1 sol: {}", fringe);
    println!("Part 2 sol: {}", find_encryption_weakness(&xmas_numbers, fringe));

}

fn find_fringe_number(xmas_numbers: &[u128], preamble_length: usize) -> u128 {
    *xmas_numbers.iter()
        .enumerate()
        .skip(preamble_length)
        .find(|(index, num)| {
            !contained_in_any_2sum(&xmas_numbers[index-preamble_length..*index], **num)
        })
        .unwrap()
        .1
}

fn contained_in_any_2sum(nums: &[u128], n: u128) -> bool {
    for i in 0..nums.len() - 1 {
        for j in i+1..nums.len() {
            if nums[i] + nums[j] == n && nums[i] != nums [j] {
                return true;
            }
        }
    }

    false
}

fn find_encryption_weakness(data: &[u128], fringe: u128) -> u128 {
    let range = (5..data.len()/2)
        .find_map(|window_size| contained_in_window(data, window_size, fringe))
        .unwrap();

    range.iter().min().unwrap() + range.iter().max().unwrap()
}

fn contained_in_window(nums: &[u128], window_size: usize, n: u128) -> Option<&[u128]> {
    nums.windows(window_size)
        .find(|window| window.iter().sum::<u128>() == n)
}
