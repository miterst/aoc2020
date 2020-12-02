use regex::Regex;

fn main() {
    let re = Regex::new(r"(?P<start>\d+)-(?P<end>\d+) (?P<chr>\w): (?P<password>\w+)").unwrap();

    let password_policies: Vec<((usize, usize), char, String)> = include_str!("input")
        .lines()
        .map(|password_policy| {
            let caps = re.captures(password_policy).unwrap();

            let start: usize = caps["start"].parse().unwrap();
            let end: usize = caps["end"].parse().unwrap();

            let chr = caps["chr"].chars().next().unwrap();
            let passwd = caps["password"].to_owned();

            ((start, end), chr, passwd)
        })
        .collect();

    let count_by_rule1 = password_policies
        .iter()
        .filter(|((start, end), chr, passwd)| {
            let count = passwd.chars().filter(|l| l == chr).count();
            (start..=end).contains(&&count)
        })
        .count();

    let count_by_rule2 = password_policies
        .iter()
        .filter(|((start, end), chr, passwd)| {
            let mut passwd_chars = passwd.chars();

            let start_chr = passwd_chars.nth(*start - 1).unwrap();
            let end_chr = passwd_chars.nth(end - 1 - start).unwrap();

            (&start_chr == chr) ^ (&end_chr == chr)
        })
        .count();

    println!("Part1: {}", count_by_rule1);
    println!("Part2: {}", count_by_rule2);
}
