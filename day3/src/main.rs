fn main() {
    let trees_count = include_str!("input")
        .lines()
        .enumerate()
        .skip(1)
        .fold(
            (vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0]),
            |(mut columns, mut tree_count), (row, line)| {
                for (col, step) in columns.iter_mut().zip((1..10).step_by(2)) {
                    if step == 9 {
                        // if we are two down
                        if row % 2 == 0 {
                            *col = (*col + 1) % line.len();
                        } else {
                            continue;
                        }
                    } else {
                        *col = (*col + step) % line.len();
                    }

                    if line.chars().nth(*col).unwrap() == '#' {
                        tree_count[step / 2] += 1;
                    }
                }

                (columns, tree_count)
            },
        )
        .1;

    println!("Part1 tree count: {}", trees_count[1]);
    println!(
        "Part2 tree count: {}",
        trees_count.iter().product::<usize>()
    );
}
