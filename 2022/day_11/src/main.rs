#[macro_use]
extern crate lazy_static;

use regex::Regex;

const _INPUT_EXAMPLE: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
const _INPUT_PART1: &str = "Monkey 0:
Starting items: 54, 53
Operation: new = old * 3
Test: divisible by 2
  If true: throw to monkey 2
  If false: throw to monkey 6

Monkey 1:
Starting items: 95, 88, 75, 81, 91, 67, 65, 84
Operation: new = old * 11
Test: divisible by 7
  If true: throw to monkey 3
  If false: throw to monkey 4

Monkey 2:
Starting items: 76, 81, 50, 93, 96, 81, 83
Operation: new = old + 6
Test: divisible by 3
  If true: throw to monkey 5
  If false: throw to monkey 1

Monkey 3:
Starting items: 83, 85, 85, 63
Operation: new = old + 4
Test: divisible by 11
  If true: throw to monkey 7
  If false: throw to monkey 4

Monkey 4:
Starting items: 85, 52, 64
Operation: new = old + 8
Test: divisible by 17
  If true: throw to monkey 0
  If false: throw to monkey 7

Monkey 5:
Starting items: 57
Operation: new = old + 2
Test: divisible by 5
  If true: throw to monkey 1
  If false: throw to monkey 3

Monkey 6:
Starting items: 60, 95, 76, 66, 91
Operation: new = old * old
Test: divisible by 13
  If true: throw to monkey 2
  If false: throw to monkey 5

Monkey 7:
Starting items: 65, 84, 76, 72, 79, 65
Operation: new = old + 5
Test: divisible by 19
  If true: throw to monkey 6
  If false: throw to monkey 0
";

struct Monkey {
    id: i64,
    items: Vec<i64>,
    operation: (String, char, String),
    test: i64,
    test_true: i64,
    test_false: i64,

    inspections: usize,
}

fn monkey_print(monkey: &Monkey, debug: bool) {
    println!(
        "Monkey {}, items: {}",
        monkey.id,
        monkey
            .items
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );
    if debug {
        println!(
            "  op: {} {} {}\n  test: div by {} (true -> {}, false -> {})",
            monkey.operation.0,
            monkey.operation.1,
            monkey.operation.2,
            monkey.test,
            monkey.test_true,
            monkey.test_false,
        );
    }
}
fn print_monkeys(monkeys: &Vec<Monkey>) {
    for monkey in monkeys {
        monkey_print(monkey, false);
    }
}
fn print_monkeys_debug(monkeys: &Vec<Monkey>) {
    for monkey in monkeys {
        monkey_print(monkey, true);
        println!("");
    }
}

lazy_static! {
    static ref RE_MONKEY: Regex = Regex::new(r"^Monkey ([0-9]+):$").unwrap();
    static ref RE_ITEMS: Regex = Regex::new(r"Starting items: ([0-9, ]+)").unwrap();
    static ref RE_OPERATION: Regex =
        Regex::new(r"Operation: new = ([a-z0-9]+) ([+*/-]) ([a-z0-9]+)").unwrap();
    static ref RE_TEST: Regex = Regex::new(r"Test: divisible by ([0-9]+)").unwrap();
    static ref RE_TEST_TRUE: Regex = Regex::new(r"If true: throw to monkey ([0-9]+)").unwrap();
    static ref RE_TEST_FALSE: Regex = Regex::new(r"If false: throw to monkey ([0-9]+)").unwrap();
}

fn monkey_parse(lines_iter: &mut core::str::Split<&str>, id: i64) -> Monkey {
    let items = RE_ITEMS
        .captures(lines_iter.next().unwrap())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .split(", ")
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let caps_operation = RE_OPERATION.captures(lines_iter.next().unwrap()).unwrap();
    let operation = (
        String::from(caps_operation.get(1).unwrap().as_str()),
        caps_operation
            .get(2)
            .unwrap()
            .as_str()
            .chars()
            .into_iter()
            .next()
            .unwrap(),
        String::from(caps_operation.get(3).unwrap().as_str()),
    );
    let test = RE_TEST
        .captures(lines_iter.next().unwrap())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<i64>()
        .unwrap();
    let test_true = RE_TEST_TRUE
        .captures(lines_iter.next().unwrap())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<i64>()
        .unwrap();
    let test_false = RE_TEST_FALSE
        .captures(lines_iter.next().unwrap())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<i64>()
        .unwrap();
    lines_iter.next(); // skip empty line

    Monkey {
        id,
        items,
        operation,
        test,
        test_true,
        test_false,

        inspections: 0,
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let lines = input.split("\n");
    let mut lines_iter = lines.into_iter();

    let mut monkeys = Vec::new();
    while let Some(line) = lines_iter.next() {
        let id = RE_MONKEY
            .captures(line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i64>()
            .unwrap();
        let monkey = monkey_parse(&mut lines_iter, id);
        monkeys.push(monkey);
    }
    monkeys
}

fn exec_item_op((lhs_str, op, rhs_str): &(String, char, String), item: i64) -> i64 {
    let lhs = if lhs_str == "old" {
        item
    } else {
        lhs_str.parse().unwrap()
    };
    let rhs = if rhs_str == "old" {
        item
    } else {
        rhs_str.parse().unwrap()
    };
    match op {
        '+' => lhs + rhs,
        '*' => lhs * rhs,
        '-' => lhs - rhs,
        '/' => lhs / rhs,
        _ => panic!("unknown operation '{}'", op),
    }
}
fn get_monkey_id(monkey: &Monkey, item: i64) -> i64 {
    if item % monkey.test == 0 {
        monkey.test_true
    } else {
        monkey.test_false
    }
}
fn monkey_exec(monkey: &mut Monkey, modulo: i64) -> Vec<(i64, i64)> {
    // returns Vec<(item_id: i64, monkey_id: i64)>
    let items: Vec<(i64, i64)> = monkey
        .items
        .iter()
        .map(|old| {
            let mut item = exec_item_op(&monkey.operation, *old);
            item = if modulo == 0 { item / 3 } else { item % modulo };
            (item, get_monkey_id(monkey, item))
        })
        .collect();
    monkey.inspections += items.len();
    monkey.items = Vec::new();

    items
}
fn run_one_round(monkeys: &mut Vec<Monkey>, modulo: i64) {
    let mut items: Vec<(i64, i64)> = Vec::new();
    for monkey in monkeys.iter_mut() {
        let mut new_items: Vec<i64> = items
            .iter()
            .filter(|(_, monkey_id)| *monkey_id == monkey.id)
            .map(|(item, _)| *item)
            .collect();
        items = items
            .into_iter()
            .filter(|(_, id)| *id != monkey.id)
            .collect();
        monkey.items.append(&mut new_items);
        items.append(&mut monkey_exec(monkey, modulo));
    }

    // drain all moved items
    for monkey in monkeys.iter_mut() {
        let mut new_items: Vec<i64> = items
            .iter()
            .filter(|(_, monkey_id)| *monkey_id == monkey.id)
            .map(|(item, _)| *item)
            .collect();
        items = items
            .into_iter()
            .filter(|(_, id)| *id != monkey.id)
            .collect();
        monkey.items.append(&mut new_items);
    }
}

fn part1() {
    let mut monkeys = parse_input(_INPUT_PART1);
    print_monkeys_debug(&monkeys);

    for round in 1..21 {
        run_one_round(&mut monkeys, 0);
        println!("\n\n === Round {} ===", round);
        print_monkeys(&monkeys);
    }

    for monkey in monkeys.iter() {
        println!("Monkey {}: {} inspections", monkey.id, monkey.inspections);
    }
    let mut inspections = monkeys.iter().map(|m| m.inspections).collect::<Vec<usize>>();
    inspections.sort();
    println!("answer={}", inspections[inspections.len() - 1] * inspections[inspections.len() - 2]);
}

fn part2() {
    let mut monkeys = parse_input(_INPUT_PART1);
    print_monkeys_debug(&monkeys);

    let modulo = monkeys.iter().map(|m| m.test).reduce(|a, b| a * b).unwrap();
    println!("mod={}", modulo);

    for _round in 1..10001 {
        run_one_round(&mut monkeys, modulo);
    }
    println!("\n === Round 10000 ===");
    print_monkeys(&monkeys);

    for monkey in monkeys.iter() {
        println!("Monkey {}: {} inspections", monkey.id, monkey.inspections);
    }
    let mut inspections = monkeys.iter().map(|m| m.inspections).collect::<Vec<usize>>();
    inspections.sort();
    println!("answer={}", inspections[inspections.len() - 1] * inspections[inspections.len() - 2]);
}

fn main() {
    println!("part1");
    part1();
    println!("part2");
    part2();
}
