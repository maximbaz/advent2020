use super::super::*;
use std::collections::HashMap;
use Rule::*;

pub struct Task;

pub enum Rule {
    Char(char),
    Ref(Vec<Vec<usize>>),
}

type Rules = HashMap<usize, Rule>;
type Values = Vec<String>;

pub struct Messages {
    rules: Rules,
    values: Values,
}

impl Solution for Task {
    type Input = Messages;
    type Output = usize;

    fn parse_input(&self, input: String) -> Self::Input {
        let blocks = input.trim().split("\n\n").collect_vec();
        Messages {
            rules: parse_rules(blocks[0]),
            values: blocks[1].lines().map(str::to_string).collect(),
        }
    }

    fn part1(&self, input: Self::Input) -> Self::Output {
        count_matching(input.rules, input.values)
    }

    fn part2(&self, input: Self::Input) -> Self::Output {
        let mut rules = input.rules;
        rules.insert(8, Ref(vec![vec![42], vec![42, 8]]));
        rules.insert(11, Ref(vec![vec![42, 31], vec![42, 11, 31]]));
        count_matching(rules, input.values)
    }
}

fn parse_rules(string: &str) -> Rules {
    string
        .lines()
        .map(|line| {
            let (id, rules) = line.split(": ").next_tuple().unwrap();
            let rules = match rules.chars().next() {
                Some('"') => Char(rules.chars().nth(1).unwrap()),
                _ => Ref(rules
                    .split(" | ")
                    .map(|b| b.split(' ').filter_map(|s| s.parse().ok()).collect())
                    .collect()),
            };

            (id.parse().unwrap(), rules)
        })
        .collect()
}

fn count_matching(rules: Rules, values: Values) -> usize {
    values
        .iter()
        .filter(|value| {
            strip_prefix(&rules, rules.get(&0).unwrap(), value)
                .iter()
                .any(|x| x.len() == 0)
        })
        .count()
}

fn strip_prefix(rules: &Rules, rule: &Rule, value: &str) -> Values {
    match (rule, value.chars().next()) {
        (Char(r), Some(c)) if *r == c => vec![value[1..].to_string()],
        (Ref(blocks), Some(_)) => blocks
            .iter()
            .flat_map(|block| {
                block
                    .iter()
                    .map(|r| rules.get(r).unwrap())
                    .fold(vec![value.to_string()], |acc, x| {
                        acc.iter().flat_map(|v| strip_prefix(rules, x, v)).collect()
                    })
            })
            .collect(),
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            2,
            Task.part1(
                Task.parse_input(
                    "
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb
                    "
                    .to_string()
                )
            )
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            12,
            Task.part2(
                Task.parse_input(
                    "
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
                    "
                    .to_string()
                )
            )
        );
    }
}
