use std::collections::HashSet;

fn main() {
    let answers = include_str!("input")
        .lines()
        .fold(
            (vec![HashSet::new()], 0),
            |(mut answers, mut group), next| {
                if next.is_empty() {
                    group += 1;
                    answers.push(HashSet::new());
                } else {
                    answers[group].extend(next.chars())
                }

                (answers, group)
            },
        )
        .0;

    let answers2 = include_str!("input")
        .lines()
        .fold(
            (vec![HashSet::new()], 0, true),
            |(mut answers, mut group, mut first_person), line| {
                if line.is_empty() {
                    group += 1;
                    first_person = true;
                    answers.push(HashSet::new());
                } else {
                    if first_person {
                        answers[group].extend(line.chars());
                        first_person = false;
                    } else {
                        answers[group] = line
                            .chars()
                            .filter(|c| answers[group].contains(c))
                            .collect();
                    }
                }

                (answers, group, first_person)
            },
        )
        .0;

    println!(
        "Part 1: {}",
        answers.iter().map(HashSet::len).sum::<usize>()
    );

    println!(
        "Part 2: {}",
        answers2.iter().map(HashSet::len).sum::<usize>()
    );
}
