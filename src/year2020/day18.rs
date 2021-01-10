use super::super::*;
use Expression::*;
use Operator::*;

pub struct Task;

impl Solution for Task {
    type Input = Vec<Vec<char>>;
    type Output = i64;

    fn parse_input(&self, input: String) -> Self::Input {
        input
            .trim()
            .lines()
            .map(|line| line.chars().filter(|c| !c.is_whitespace()).collect())
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> Self::Output {
        solve(input, false)
    }

    fn part2(&self, input: &Self::Input) -> Self::Output {
        solve(input, true)
    }
}

fn solve(homework: &Vec<Vec<char>>, is_advanced: bool) -> i64 {
    homework
        .iter()
        .filter_map(|chars| Parser { chars, is_advanced }.run())
        .map(|expr| expr.eval())
        .sum()
}

#[derive(Copy, Clone, PartialEq)]
enum Operator {
    Sum,
    Product,
}

enum Expression {
    Value(i64),
    Parenthesis(Box<Expression>),
    Binary(Operator, Box<Expression>, Box<Expression>),
}

impl Expression {
    fn eval(&self) -> i64 {
        match self {
            Value(n) => *n,
            Parenthesis(a) => a.eval(),
            Binary(Product, a, b) => a.eval() * b.eval(),
            Binary(Sum, a, b) => a.eval() + b.eval(),
        }
    }
}

struct Parser<'a> {
    chars: &'a [char],
    is_advanced: bool,
}

impl<'a> Parser<'a> {
    fn run(&self) -> Option<Expression> {
        self.parse_expression(0).map(|(_, expr)| expr)
    }

    fn parse_expression(&self, pos: usize) -> Option<(usize, Expression)> {
        self.parse_start(pos)
            .and_then(|(pos, left)| match self.parse_operator(pos) {
                Some((pos, op)) => self
                    .parse_expression(pos)
                    .and_then(|(pos, right)| Some((pos, self.combine(op, left, right)))),
                None => Some((pos, left)),
            })
    }

    fn parse_start(&self, pos: usize) -> Option<(usize, Expression)> {
        match self.parse_open_parenthesis(pos) {
            Some(pos) => self
                .parse_expression(pos)
                .and_then(|(pos, expr)| self.parse_close_parenthesis(expr, pos)),
            None => self.parse_value(pos),
        }
    }

    fn parse_open_parenthesis(&self, pos: usize) -> Option<usize> {
        self.chars.get(pos).filter(|&&v| v == '(').map(|_| pos + 1)
    }

    fn parse_close_parenthesis(&self, expr: Expression, pos: usize) -> Option<(usize, Expression)> {
        self.chars
            .get(pos)
            .filter(|&&v| v == ')')
            .map(|_| (pos + 1, Parenthesis(Box::new(expr))))
    }

    fn parse_value(&self, pos: usize) -> Option<(usize, Expression)> {
        let string = self
            .chars
            .iter()
            .skip(pos)
            .take_while(|&&c| c.is_digit(10))
            .collect::<String>();

        string.parse().ok().map(|v| (pos + string.len(), Value(v)))
    }

    fn parse_operator(&self, pos: usize) -> Option<(usize, Operator)> {
        self.chars
            .get(pos)
            .filter(|&&v| v == '+' || v == '*')
            .map(|&op| (pos + 1, if op == '+' { Sum } else { Product }))
    }

    fn combine(&self, op: Operator, left: Expression, right: Expression) -> Expression {
        match right {
            Binary(op2, left2, right2) => {
                if self.has_lower_precedence(op, op2) {
                    Binary(op, Box::new(left), Box::new(Binary(op2, left2, right2)))
                } else {
                    Binary(op2, Box::new(self.combine(op, left, *left2)), right2)
                }
            }
            _ => Binary(op, Box::new(left), Box::new(right)),
        }
    }

    fn has_lower_precedence(&self, op1: Operator, op2: Operator) -> bool {
        self.is_advanced && op1 == Product && op2 == Sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            71,
            Task.part1(&Task.parse_input("1 + 2 * 3 + 4 * 5 + 6".to_string()))
        );
        assert_eq!(
            51,
            Task.part1(&Task.parse_input("1 + (2 * 3) + (4 * (5 + 6))".to_string()))
        );
        assert_eq!(
            26,
            Task.part1(&Task.parse_input("2 * 3 + (4 * 5)".to_string()))
        );
        assert_eq!(
            437,
            Task.part1(&Task.parse_input("5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string()))
        );
        assert_eq!(
            12240,
            Task.part1(&Task.parse_input("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string()))
        );
        assert_eq!(
            13632,
            Task.part1(
                &Task.parse_input("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string())
            )
        );

        assert_eq!(
            71 + 51 + 26 + 437 + 12240 + 13632,
            Task.part1(
                &Task.parse_input(
                    "
        1 + 2 * 3 + 4 * 5 + 6
        1 + (2 * 3) + (4 * (5 + 6))
        2 * 3 + (4 * 5)
        5 + (8 * 3 + 9 + 3 * 4 * 3)
        5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
        ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2
                        "
                    .to_string()
                )
            )
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            231,
            Task.part2(&Task.parse_input("1 + 2 * 3 + 4 * 5 + 6".to_string()))
        );
        assert_eq!(
            51,
            Task.part2(&Task.parse_input("1 + (2 * 3) + (4 * (5 + 6))".to_string()))
        );
        assert_eq!(
            46,
            Task.part2(&Task.parse_input("2 * 3 + (4 * 5)".to_string()))
        );
        assert_eq!(
            1445,
            Task.part2(&Task.parse_input("5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string()))
        );
        assert_eq!(
            669060,
            Task.part2(&Task.parse_input("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string()))
        );
        assert_eq!(
            23340,
            Task.part2(
                &Task.parse_input("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string())
            )
        );

        assert_eq!(
            231 + 51 + 46 + 1445 + 669060 + 23340,
            Task.part2(
                &Task.parse_input(
                    "
        1 + 2 * 3 + 4 * 5 + 6
        1 + (2 * 3) + (4 * (5 + 6))
        2 * 3 + (4 * 5)
        5 + (8 * 3 + 9 + 3 * 4 * 3)
        5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
        ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2
                        "
                    .to_string()
                )
            )
        );
    }
}
