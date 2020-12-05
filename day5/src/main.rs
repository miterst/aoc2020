use std::collections::HashSet;

fn main() {
    let seat_ids: HashSet<i32> = include_str!("input")
        .lines()
        .map(|line| {
            let bit_string: Vec<&str> = line
                .chars()
                .map(|c| if c == 'F' || c == 'L' { "0" } else { "1" })
                .collect();

            i32::from_str_radix(&bit_string.join(""), 2).unwrap()
        })
        .collect();

    let all_possible_seat_ids: HashSet<i32> = (0..1024).collect();

    let missing_seat: HashSet<_> = all_possible_seat_ids
        .difference(&seat_ids)
        .filter(|&&seat_id| seat_ids.contains(&(seat_id + 1)) && seat_ids.contains(&(seat_id - 1)))
        .collect();

    println!("{}", seat_ids.iter().max().unwrap());
    println!("{:?}", missing_seat);
}
