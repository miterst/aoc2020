use std::collections::{HashMap, HashSet};

const ONES: u64 = u64::max_value();

#[derive(Debug)]
struct Mask {
    mask0: u64,
    mask1: u64,
    floating: Vec<u64>,
}

impl Mask {
    fn new() -> Self {
        Self {
            mask0: ONES,
            mask1: 0,
            floating: vec![],
        }
    }

    fn add_floating(&mut self, index: u64) {
        self.floating.push(index)
    }

    fn set_m0(&mut self, index: u64) {
        self.mask0 ^= 1 << index;
    }

    fn set_m1(&mut self, index: u64) {
        self.mask1 |= 1 << index;
    }

    fn apply_mask_on_value(&self, mut n: u64) -> u64 {
        n &= self.mask0;
        n |= self.mask1;
        n
    }

    fn apply_mask_on_index(&self, mut n: u64) -> u64 {
        n |= self.mask1;
        n
    }

    fn generate_addresses(&self, n: u64) -> HashSet<u64> {
        let mut addresses = HashSet::new();
        addresses.insert(self.apply_mask_on_index(n));

        let mut floating_indices = self.floating.to_vec();

        while !floating_indices.is_empty() {
            let index = floating_indices.pop().unwrap();

            let mut v = HashSet::new();
            for address in addresses.iter() {
                let address0 = *address & ((1 << index) ^ ONES);
                let address1 = *address | (1 << index);
                v.insert(address1);
                v.insert(address0);
            }

            addresses.extend(v);
        }

        addresses
    }
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn part1() -> u64 {
    include_str!("big")
        .lines()
        .fold(
            (HashMap::<u64, u64>::new(), Mask::new()),
            |(mut memory, mut mask), line| {
                if line.starts_with("mask") {
                    mask = parse_mask(line);
                } else {
                    let (index, num) = parse_mem(line);

                    let m = mask.apply_mask_on_value(num);
                    memory.insert(index, m);
                }

                (memory, mask)
            },
        )
        .0
        .values()
        .sum()
}

fn part2() -> u64 {
    include_str!("big")
        .lines()
        .fold(
            (HashMap::<u64, u64>::new(), Mask::new()),
            |(mut memory, mut mask), line| {
                if line.starts_with("mask") {
                    mask = parse_mask(line);
                } else {
                    let (index, num) = parse_mem(line);

                    for ix in mask.generate_addresses(index) {
                        memory.insert(ix, num);
                    }
                }

                (memory, mask)
            },
        )
        .0
        .values()
        .sum()
}

fn parse_mem(line: &str) -> (u64, u64) {
    let index: u64 = line
        .chars()
        .skip(4)
        .take_while(|c| c.is_numeric())
        .collect::<String>()
        .parse()
        .unwrap();
    let num: u64 = line
        .chars()
        .skip_while(|x| *x != '=')
        .skip(2)
        .collect::<String>()
        .parse()
        .unwrap();

    (index, num)
}

fn parse_mask(line: &str) -> Mask {
    let mask_string: Vec<char> = line
        .strip_prefix("mask = ")
        .unwrap()
        .chars()
        .rev()
        .collect();

    mask_string
        .iter()
        .enumerate()
        .fold(Mask::new(), |mut mask, (index, c)| {
            if *c == '1' {
                mask.set_m1(index as u64);
            } else if *c == '0' {
                mask.set_m0(index as u64);
            } else {
                mask.add_floating(index as u64);
            }

            mask
        })
}

#[test]
fn parse_mask_works() {
    let mask = parse_mask("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
    assert_eq!(73, mask.apply_mask_on_value(11))
}

#[test]
fn parse_mem_works() {
    let (index, mem) = parse_mem("mem[8] = 111100");
    assert_eq!(8, index);
    assert_eq!(111100, mem);
}

#[test]
fn floating_addresses_generator() {
    let mask = parse_mask("mask = 00000000000000000000000000000000X0XX");
    assert_eq!(8, mask.generate_addresses(26).len());
}
