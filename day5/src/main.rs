use std::collections::HashSet;

fn main() {
    let seat_ids: HashSet<i32> = include_str!("input")
        .lines()
        .map(|line| {
            let row = line[..7]
                .chars()
                .fold((0, 127), |mut acc, next| {
                    let diff = (acc.1 - acc.0 + 1) / 2;
                    match next {
                        'F' => acc.1 = acc.1 - diff,
                        'B' => acc.0 = acc.0 + diff,
                        _ => unreachable!(),
                    }
                    acc
                })
                .0;

            let column = line[7..]
                .chars()
                .fold((0, 7), |mut acc, next| {
                    let diff = (acc.1 - acc.0 + 1) / 2;
                    match next {
                        'L' => acc.1 = acc.1 - diff,
                        'R' => acc.0 = acc.0 + diff,
                        _ => unreachable!(),
                    }
                    acc
                })
                .0;

            row * 8 + column
        })
        .collect();

    let all_possible_seat_ids: HashSet<i32> =
        (0..128).flat_map(|row| {
            (0..8).map(move |column| {
                row*8 + &column
            })
        }).collect();

    let missing_seat: HashSet<_> =
        all_possible_seat_ids
            .difference(&seat_ids)
            .filter(|&&seat_id| seat_ids.contains(&(seat_id + 1)) && seat_ids.contains(&(seat_id - 1)))
            .collect();

    println!("{}", seat_ids.iter().max().unwrap());
    println!("{:?}", missing_seat);
}
