fn main() {
    println!("Part1 {}", part1());
    println!("Part2 {}", part2());
}

fn part1() -> u32 {
    let mut notes = include_str!("notes_big").lines();

    let time: u32 = notes.next().unwrap().parse().unwrap();
    let (diff, bus) = notes
        .next()
        .unwrap()
        .split(',')
        .filter(|c| *c != "x")
        .map(|x| x.parse::<u32>().unwrap())
        .map(|x| {
            if time % x == 0 {
                (0, x)
            } else {
                let diff = (((time / x) + 1) * x) - time;
                (diff, x)
            }
        })
        .min_by_key(|(d, _)| *d)
        .unwrap();

    bus * diff
}

fn part2() -> i128 {
    let constraints: Vec<(i128, i128)> = include_str!("notes_big")
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .enumerate()
        .filter(|(_, x)| *x != "x")
        .map(|(i, x)| {
            let divisor: i128 = x.parse().unwrap();
            let reminder = modulus(-(i as i128),  divisor);

            (reminder, divisor)
        })
        .collect();


    println!("{:?}", constraints);
    let (reminders, mods): (Vec<i128>, Vec<i128>) = constraints.into_iter().unzip();
    chinese_remainder_theorem(&reminders, &mods).unwrap()
}

// copy/paste fixes ring_algorithm::chinese_remainder_theorem negative reminder
pub fn chinese_remainder_theorem(reminders: &[i128], mods: &[i128]) -> Option<i128>
{
    if reminders.len() != mods.len() {
        return None;
    }
    let mut v = Vec::with_capacity(reminders.len());
    for (i, (u_i, m_i)) in reminders.iter().zip(mods.iter()).enumerate() {
        let coef_i = ring_algorithm::modulo_inverse(
            mods[0..i].iter().fold(1, |p, v| modulus(p * v, *m_i)),
            m_i.clone(),
        )?;
        let t = v
            .iter()
            .zip(mods.iter())
            .rev()
            .fold(0, |t, (v_j, m_j)| modulus(m_j * t + *v_j, *m_i));

        let c = (u_i - t) * coef_i;

        v.push(modulus(c, *m_i));
    }

    let mut ret = v.pop()?;
    for (v_i, m_i) in v.iter().zip(mods.iter()).rev() {
        ret = ret * m_i + *v_i;
    }

    Some(ret)
}

fn modulus(a: i128, b: i128) -> i128 {
    if a < 0 {
        ((a % b) + b) % b
    } else {
        a % b
    }
}
