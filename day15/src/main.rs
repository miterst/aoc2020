use std::collections::HashMap;

struct Recitation {
    current_location: usize,
    last_spoken: usize,
    spoken: HashMap<usize, usize>,
}

impl Recitation {
    fn new(spoken: &[usize]) -> Self {
        let mut spoken_map: HashMap<usize, usize> = HashMap::new();
        spoken[..spoken.len() - 1]
            .iter()
            .enumerate()
            .for_each(|(i, x)| {
                spoken_map.insert(*x, i + 1);
            });

        Self {
            current_location: spoken.len(),
            last_spoken: *spoken.last().unwrap(),
            spoken: spoken_map,
        }
    }
}

impl Iterator for Recitation {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let found_at = *self
            .spoken
            .get(&self.last_spoken)
            .unwrap_or(&self.current_location);

        self.spoken.insert(self.last_spoken, self.current_location);

        self.last_spoken = self.current_location - found_at;
        self.current_location += 1;

        Some(self.last_spoken)
    }
}

fn main() {
    let spoken = vec![0, 5, 4, 1, 10, 14, 7];
    let last_spoken = spoken.len() + 1;

    println!(
        "Part 1: {:?}",
        Recitation::new(&spoken).nth(2020 - last_spoken).unwrap()
    );
    println!(
        "Part 2: {:?}",
        Recitation::new(&spoken)
            .nth(30000000 - last_spoken)
            .unwrap()
    );
}
