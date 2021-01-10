use super::super::*;
use regex::Regex;
use std::ops::RangeInclusive;

pub struct Task;

pub type Rule = Vec<RangeInclusive<usize>>;
pub type Ticket = Vec<usize>;

pub struct Tickets {
    rules: Vec<Rule>,
    my: Ticket,
    nearby: Vec<Ticket>,
}

impl Solution for Task {
    type Input = Tickets;
    type Output = usize;

    fn parse_input(&self, input: String) -> Self::Input {
        let blocks = input.trim().split("\n\n").collect_vec();

        Tickets {
            rules: parse_rules(blocks[0]),
            my: blocks[1].lines().nth(1).map(parse_ticket).unwrap(),
            nearby: blocks[2].lines().skip(1).map(parse_ticket).collect_vec(),
        }
    }

    fn part1(&self, input: &Self::Input) -> Self::Output {
        input
            .nearby
            .iter()
            .flatten()
            .filter(|field| !is_valid(field, &input.rules))
            .sum()
    }

    fn part2(&self, input: &Self::Input) -> Self::Output {
        let valid_tickets = input
            .nearby
            .iter()
            .filter(|ticket| ticket.iter().all(|field| is_valid(field, &input.rules)))
            .collect_vec();

        let cols = (0..input.my.len())
            .map(|i| valid_tickets.iter().map(|ticket| ticket[i]).collect_vec())
            .collect_vec();

        let mut rule_to_cols = input
            .rules
            .iter()
            .map(|rule| {
                (0..input.my.len())
                    .filter(|&i| cols[i].iter().all(|val| matches(val, rule)))
                    .collect_vec()
            })
            .collect_vec();

        let mut rule_to_col = vec![0; input.my.len()];

        while rule_to_cols.iter().any(|rules| !rules.is_empty()) {
            let (rule, col) = rule_to_cols
                .iter()
                .enumerate()
                .find(|(_, cols)| cols.len() == 1)
                .map(|(rule, cols)| (rule, cols[0]))
                .unwrap();

            rule_to_col[rule] = col;

            rule_to_cols = rule_to_cols
                .into_iter()
                .map(|cols| cols.into_iter().filter(|&c| c != col).collect())
                .collect();
        }

        rule_to_col.iter().take(6).map(|&i| input.my[i]).product()
    }
}

fn is_valid(field: &usize, rules: &Vec<Rule>) -> bool {
    rules.iter().any(|rule| matches(field, rule))
}

fn matches(field: &usize, rule: &Rule) -> bool {
    rule.iter().any(|rule| rule.contains(field))
}

fn parse_rules(input: &str) -> Vec<Rule> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)").unwrap();
    }

    input
        .lines()
        .map(|line| {
            RE.captures_iter(line)
                .filter_map(|cap| cap[0].parse().ok())
                .tuples()
                .map(|(from, to)| (from..=to))
                .collect()
        })
        .collect()
}

fn parse_ticket(input: &str) -> Ticket {
    input.split(",").filter_map(|s| s.parse().ok()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            71,
            Task.part1(
                &Task.parse_input(
                    "
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
                    "
                    .to_string()
                )
            )
        );
    }
}
