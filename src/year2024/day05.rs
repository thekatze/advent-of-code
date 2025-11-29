#[derive(Debug)]
pub struct PageOrderRule(u64, u64);
#[derive(Clone, Debug)]
pub struct PageUpdate(Vec<u64>);

#[derive(Debug)]
pub struct Parsed {
    page_orders: Vec<PageOrderRule>,
    page_updates: Vec<PageUpdate>,
}

pub fn parse(input: &str) -> Parsed {
    let (page_orders, page_updates) = input.split_once("\n\n").expect("invalid input format");

    let page_orders = page_orders
        .lines()
        .map(|line| {
            let (first, second) = line.split_once("|").expect("rule should be separated by |");
            PageOrderRule(
                first.parse().expect("page order must be a number"),
                second.parse().expect("page order must be a number"),
            )
        })
        .collect();

    let page_updates = page_updates
        .lines()
        .map(|line| {
            PageUpdate(
                line.split(',')
                    .map(str::parse)
                    .collect::<Result<_, _>>()
                    .expect("page update must be number"),
            )
        })
        .collect();

    Parsed {
        page_orders,
        page_updates,
    }
}

impl PageOrderRule {
    fn is_valid_list(&self, list: &[u64]) -> bool {
        let (first, second) = (self.0, self.1);

        let first_index = list.iter().position(|num| *num == first);
        let second_index = list.iter().position(|num| *num == second);

        first_index
            .zip(second_index)
            .map(|(f, s)| f < s)
            .unwrap_or(true)
    }

    // TODO: refactor part 1 to use is_valid_list
    fn is_valid(&self, previous_numbers: &[u64], next: u64) -> bool {
        let (first, second) = (self.0, self.1);

        previous_numbers
            .iter()
            .all(|previous| !(next == first && *previous == second))
    }
}

impl PageUpdate {
    fn valid_order(&self, rules: &[PageOrderRule]) -> bool {
        let mut numbers_inserted: &[u64] = &self.0[..0];

        self.0.iter().all(|num| {
            let valid_insert = rules
                .iter()
                .all(|rule| rule.is_valid(numbers_inserted, *num));

            if valid_insert {
                numbers_inserted = &self.0[..numbers_inserted.len() + 1];
            }

            valid_insert
        })
    }

    fn to_fixed(&self, rules: &[PageOrderRule]) -> Self {
        let mut fixed: Vec<u64> = Vec::with_capacity(self.0.len());

        while fixed.len() != self.0.len() {
            let next = self.0[fixed.len()];
            fixed.push(next);

            while let Some(failing_rule) = rules.iter().find(|rule| !rule.is_valid_list(&fixed)) {
                let left = fixed
                    .iter()
                    .position(|n| *n == failing_rule.0)
                    .expect("failing rule implies both numbers exist in list");

                let right = fixed
                    .iter()
                    .position(|n| *n == failing_rule.1)
                    .expect("failing rule implies both numbers exist in list");

                fixed.swap(left, right);
            }
        }

        PageUpdate(fixed)
    }

    fn get_middle(&self) -> u64 {
        let middle_index = self.0.len() / 2;
        self.0[middle_index]
    }
}

pub fn part1(input: &Parsed) -> u64 {
    input
        .page_updates
        .iter()
        .filter(|&update| update.valid_order(&input.page_orders))
        .map(|update| update.get_middle())
        .sum()
}

pub fn part2(input: &Parsed) -> u64 {
    input
        .page_updates
        .iter()
        .filter(|update| !update.valid_order(&input.page_orders))
        .map(|update| update.to_fixed(&input.page_orders).get_middle())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part1() {
        let result = super::part1(&parse(SAMPLE_INPUT));
        assert_eq!(result, 143)
    }

    #[test]
    fn part2() {
        let result = super::part2(&parse(SAMPLE_INPUT));
        assert_eq!(result, 123)
    }
}
