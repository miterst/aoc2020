use regex::Regex;
use std::collections::HashMap;
use std::ops::RangeInclusive;

struct FieldRule {
    name: String,
    rules: Vec<RangeInclusive<usize>>,
}

impl FieldRule {
    fn new(name: String) -> FieldRule {
        FieldRule {
            name,
            rules: vec![],
        }
    }

    fn add_rule(&mut self, rule: RangeInclusive<usize>) {
        self.rules.push(rule);
    }

    fn contains(&self, n: &usize) -> bool {
        self.rules.iter().any(|rule| rule.contains(n))
    }
}

fn main() {
    let input = include_str!("big");

    let field_rules = get_field_rules(input);

    println!("{}", part1(input, &field_rules));
    println!("{:?}", part2(input, &field_rules));
}

fn part1(input: &str, field_rules: &[FieldRule]) -> usize {
    input
        .lines()
        .skip_while(|x| !x.starts_with("nearby tickets"))
        .skip(1)
        .flat_map(|line| {
            line.split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .filter(|n| field_rules.iter().all(|field| !field.contains(n)))
                .collect::<Vec<usize>>()
        })
        .sum()
}

fn part2(input: &str, field_rules: &[FieldRule]) -> u64 {
    let valid_nearby_tickets: Vec<Vec<usize>> = input
        .lines()
        .skip_while(|line| !line.starts_with("nearby tickets"))
        .skip(1)
        .filter_map(|line| {
            let ticket: Vec<usize> = line
                .split(',')
                .map(|value| value.parse::<usize>().unwrap())
                .collect();

            if ticket
                .iter()
                .all(|value| field_rules.iter().any(|x| x.contains(value)))
            {
                Some(ticket)
            } else {
                None
            }
        })
        .collect();

    let count_fields_by_column =
        valid_nearby_tickets
            .iter()
            .fold(HashMap::new(), |mut counter, ticket| {
                for (column, ticket_value) in ticket.iter().enumerate() {
                    for field_rule in field_rules.iter() {
                        if field_rule.contains(ticket_value) {
                            *counter
                                .entry(column)
                                .or_insert_with(HashMap::new)
                                .entry(field_rule.name.as_str())
                                .or_insert(0) += 1;
                        }
                    }
                }

                counter
            });

    let mut valid_counts: Vec<(usize, Vec<_>)> = count_fields_by_column
        .into_iter()
        .map(|(col, vals)| {
            (
                col,
                vals.into_iter()
                    .filter(|(_, count)| *count == valid_nearby_tickets.len())
                    .collect(),
            )
        })
        .collect();

    let own_ticket = parse_own_ticket(input);

    // start with the most constrained
    valid_counts.sort_by_key(|(_, field_counts)| field_counts.len());

    valid_counts
        .iter()
        .fold(
            HashMap::new(),
            |mut assignment, (column, available_fields)| {
                let (field_name, _) = available_fields
                    .iter()
                    .find(|(field_name, _)| !assignment.contains_key(field_name))
                    .unwrap();

                assignment.insert(field_name, column);

                assignment
            },
        )
        .into_iter()
        .filter(|(field_name, _)| field_name.starts_with("departure"))
        .map(|(_, column)| own_ticket[*column] as u64)
        .product()
}

fn get_field_rules(input: &str) -> Vec<FieldRule> {
    let ranges_re = Regex::new(
        r"(?P<field>.+): (?P<start1>\d+)-(?P<end1>\d+) or (?P<start2>\d+)-(?P<end2>\d+)",
    )
    .unwrap();

    input
        .lines()
        .take_while(|line| !line.is_empty())
        .fold(vec![], |mut field_rules, line| {
            let caps = ranges_re.captures(line).unwrap();

            let mut field_rule = FieldRule::new(caps["field"].to_string());

            let rule =
                caps["start1"].parse::<usize>().unwrap()..=caps["end1"].parse::<usize>().unwrap();
            field_rule.add_rule(rule);

            let rule =
                caps["start2"].parse::<usize>().unwrap()..=caps["end2"].parse::<usize>().unwrap();
            field_rule.add_rule(rule);

            field_rules.push(field_rule);
            field_rules
        })
}

fn parse_own_ticket(input: &str) -> Vec<u32> {
    input
        .lines()
        .skip_while(|line| !line.starts_with("your ticket"))
        .skip(1)
        .take(1)
        .flat_map(|line| {
            line.split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}
