use std::{
    fmt::Debug,
    fs::{self},
};

struct Monkey {
    items: Vec<i64>,
    op: Box<dyn Fn(i64) -> i64>,
    test_coeff: i64,
    true_monkey: usize,
    false_monkey: usize,
    inspect_count: usize,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            /* .field("op", &self.op) */
            .field("test_coeff", &self.test_coeff)
            .field("true_monkey", &self.true_monkey)
            .field("false_monkey", &self.false_monkey)
            .field("inspect_count", &self.inspect_count)
            .finish()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut monkeys = input.split("\n\n").fold(Vec::new(), |mut acc, monkey_str| {
        let mut input = monkey_str.lines();

        let start_items = input
            .nth(1)
            .unwrap()
            .split(':')
            .nth(1)
            .unwrap()
            .split(',')
            .map(|i| i.trim().parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let operation_str = input.next().unwrap();
        let op_coeff_str = operation_str.split_whitespace().nth(5).unwrap().trim();
        let operation: Box<dyn Fn(i64) -> i64> = if let Ok(op_coeff) = op_coeff_str.parse::<i64>() {
            match operation_str.split_whitespace().nth(4).unwrap() {
                "*" => Box::new(move |value: i64| (value * op_coeff)),
                "+" => Box::new(move |value: i64| (value + op_coeff)),
                _ => panic!(),
            }
        } else {
            Box::new(|value| (value * value))
        };

        let test_coeff = input
            .next()
            .unwrap()
            .split_whitespace()
            .nth(3)
            .unwrap()
            .trim()
            .parse::<i64>()
            .unwrap();

        let true_monkey = input
            .next()
            .unwrap()
            .split_whitespace()
            .nth_back(0)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let false_monkey = input
            .next()
            .unwrap()
            .split_whitespace()
            .nth_back(0)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        acc.push(Monkey {
            items: start_items,
            op: operation,
            test_coeff,
            true_monkey,
            false_monkey,
            inspect_count: 0,
        });

        acc
    });

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            // println!("Monkey: {}", i);

            for item_index in 0..monkeys[i].items.len() {
                monkeys[i].inspect_count += 1;

                let item = monkeys[i].items[item_index];
                // println!("\t Inspect item with worry level: {}", item);

                let mut new_worry_level = (monkeys[i].op)(item);
                // println!("\t\t new worry level: {}", new_worry_level);

                new_worry_level /= 3;
                // println!("\t\t Monkey is bored, worry level: {}", new_worry_level);

                if new_worry_level % monkeys[i].test_coeff == 0 {
                    // println!(
                    //     "\t\t Test true, thrown to monkey {}",
                    //     monkeys[i].true_monkey
                    // );
                    let new_monkey = monkeys[i].true_monkey;
                    monkeys[new_monkey].items.push(new_worry_level);
                } else {
                    // println!(
                    //    "\t\t Test false, thrown to monkey {}",
                    //    monkeys[i].false_monkey
                    // );
                    let new_monkey = monkeys[i].false_monkey;
                    monkeys[new_monkey].items.push(new_worry_level);
                }
            }
            monkeys[i].items.clear();
        }
    }

    monkeys.sort_by(|a, b| a.inspect_count.cmp(&b.inspect_count));
    monkeys.reverse();

    println!("Part A: {:#?}", monkeys[0].inspect_count * monkeys[1].inspect_count);

    // part b

    let input = fs::read_to_string("input.txt").unwrap();

    let mut monkeys = input.split("\n\n").fold(Vec::new(), |mut acc, monkey_str| {
        let mut input = monkey_str.lines();

        let start_items = input
            .nth(1)
            .unwrap()
            .split(':')
            .nth(1)
            .unwrap()
            .split(',')
            .map(|i| i.trim().parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let operation_str = input.next().unwrap();
        let op_coeff_str = operation_str.split_whitespace().nth(5).unwrap().trim();
        let operation: Box<dyn Fn(i64) -> i64> = if let Ok(op_coeff) = op_coeff_str.parse::<i64>() {
            match operation_str.split_whitespace().nth(4).unwrap() {
                "*" => Box::new(move |value: i64| (value * op_coeff)),
                "+" => Box::new(move |value: i64| (value + op_coeff)),
                _ => panic!(),
            }
        } else {
            Box::new(|value| (value * value))
        };

        let test_coeff = input
            .next()
            .unwrap()
            .split_whitespace()
            .nth(3)
            .unwrap()
            .trim()
            .parse::<i64>()
            .unwrap();

        let true_monkey = input
            .next()
            .unwrap()
            .split_whitespace()
            .nth_back(0)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let false_monkey = input
            .next()
            .unwrap()
            .split_whitespace()
            .nth_back(0)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        acc.push(Monkey {
            items: start_items,
            op: operation,
            test_coeff,
            true_monkey,
            false_monkey,
            inspect_count: 0,
        });

        acc
    });

    let m: i64 = monkeys.iter().map(|m| m.test_coeff).product();

    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            // println!("Monkey: {}", i);

            for item_index in 0..monkeys[i].items.len() {
                monkeys[i].inspect_count += 1;

                let item = monkeys[i].items[item_index];
                // println!("\t Inspect item with worry level: {}", item);

                let mut new_worry_level = (monkeys[i].op)(item);
                // println!("\t\t new worry level: {}", new_worry_level);

                new_worry_level %= m;
                // println!("\t\t Monkey is bored, worry level: {}", new_worry_level);

                if new_worry_level % monkeys[i].test_coeff == 0 {
                    // println!(
                    //    "\t\t Test true, thrown to monkey {}",
                    //     monkeys[i].true_monkey
                    // );
                    let new_monkey = monkeys[i].true_monkey;
                    monkeys[new_monkey].items.push(new_worry_level);
                } else {
                    // println!(
                    //    "\t\t Test false, thrown to monkey {}",
                    //    monkeys[i].false_monkey
                    // );
                    let new_monkey = monkeys[i].false_monkey;
                    monkeys[new_monkey].items.push(new_worry_level);
                }
            }
            monkeys[i].items.clear();
        }
    }

    monkeys.sort_by(|a, b| a.inspect_count.cmp(&b.inspect_count));
    monkeys.reverse();

    println!("Part B: {:#?}", monkeys[0].inspect_count * monkeys[1].inspect_count);
}
