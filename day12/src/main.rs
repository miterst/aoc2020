fn main() {
    let input = include_str!("small_input")
        .lines()
        .fold(
            ((0.0, 0.0), 0.0),
            |(mut location, mut angle): (_, f64), instruction| {
                let value: f64 = instruction[1..].parse().unwrap();

                match instruction.chars().next().unwrap() {
                    'F' => {
                        location.0 += value * f64::cos(angle.to_radians());
                        location.1 += value * f64::sin(angle.to_radians());
                    }
                    'N' => location.1 += value,
                    'S' => location.1 -= value,
                    'E' => location.0 += value,
                    'W' => location.0 -= value,
                    'L' => {
                        angle = (angle + value) % 360.0;
                    }
                    'R' => {
                        angle = (angle - value) % 360.0;
                    }
                    _ => unreachable!(),
                }

                (location, angle)
            },
        )
        .0;

    let input2 = include_str!("small_input")
        .lines()
        .fold(
            ((0, 0), (10, 1)),
            |(mut location, mut waypoint), instruction| {
                let value: i32 = instruction[1..].parse().unwrap();

                match instruction.chars().next().unwrap() {
                    'F' => {
                        location.0 += value * waypoint.0;
                        location.1 += value * waypoint.1;
                    }
                    'N' => waypoint.1 += value,
                    'S' => waypoint.1 -= value,
                    'E' => waypoint.0 += value,
                    'W' => waypoint.0 -= value,
                    'L' => match value {
                        90 => waypoint = (-waypoint.1, waypoint.0),
                        180 => waypoint = (-waypoint.0, -waypoint.1),
                        270 => waypoint = (waypoint.1, -waypoint.0),
                        _ => unreachable!(),
                    },
                    'R' => match value {
                        270 => waypoint = (-waypoint.1, waypoint.0),
                        180 => waypoint = (-waypoint.0, -waypoint.1),
                        90 => waypoint = (waypoint.1, -waypoint.0),
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }

                (location, waypoint)
            },
        )
        .0;

    println!("{:?}", input.0.abs() + input.1.abs());
    println!("{:?}", input2.0.abs() + input2.1.abs());
}
