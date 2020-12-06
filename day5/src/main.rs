use std::collections::HashSet;

fn main() {
    let seat_ids: HashSet<i32> = include_str!("input")
        .lines()
        .map(|line| {
            let seat_id = line.chars().fold(0, |mut acc, next| {
                acc <<= 1;

                if next == 'B' || next == 'R' {
                    acc += 1;
                }

                acc
            });

            seat_id
        })
        .collect();

    let missing_seat: i32 = (0..1024) // all possible seats
        .filter(|seat| !seat_ids.contains(seat))
        .find(|seat_id| seat_ids.contains(&(seat_id + 1)) && seat_ids.contains(&(seat_id - 1)))
        .unwrap();

    println!("{}", seat_ids.iter().max().unwrap());
    println!("{}", missing_seat);
}
