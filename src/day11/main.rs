#![feature(iter_array_chunks)]
#![feature(iterator_try_collect)]
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;

use std::io::ErrorKind;
type IoError = std::io::Error;

/// Couldn't be bothered to modify my parser for task 2... so the modulus is 
/// a magic number here.
const MODULUS: usize = 7 * 19 * 17 * 11 * 13 * 2 * 5 * 3;

fn parse_starting_items(line: &str) -> std::io::Result<VecDeque<usize>> {
    let errf = || {IoError::new(ErrorKind::InvalidInput, "Wrong prefix on starting items line")};
    let numbers = line.trim().strip_prefix("Starting items:").ok_or_else(errf)?;
    
    numbers.trim().split(", ").map(|num| -> std::io::Result<_> {
        let n = num.parse::<usize>().map_err(|err| IoError::new(ErrorKind::InvalidInput, err.to_string()))?;
        Ok(n)
    }).try_collect::<VecDeque<usize>>()
}

fn parse_operation(line: &str) -> std::io::Result<Box<dyn Fn(usize) -> usize>> {
    let errf = || {IoError::new(ErrorKind::InvalidInput, "Wrong prefix on operation line")};
    let eq = line.trim().strip_prefix("Operation: ").ok_or_else(errf)?;

    let errf = || IoError::new(ErrorKind::InvalidInput, "Could not parse equation");
    let mut parts = eq.split(' ');
    let is = parts.nth(0).ok_or_else(errf)?;
    if is != "new" {
        return Err(IoError::new(ErrorKind::InvalidInput, "left of = must be 'new'"));
    }

    let lhs = parts.nth(1).ok_or_else(errf)?;
    if lhs != "old" {
        return Err(IoError::new(ErrorKind::InvalidInput, "left operand must be 'old'"));
    }

    let op = parts.nth(0).ok_or_else(errf)?;
    let rhs = parts.nth(0).ok_or_else(errf)?;

    if op == "*" {
        if rhs == "old" {
            let closure = |old: usize | { old.clone() * old };
            return Ok(Box::new(closure));
        } else {
            let c = rhs.parse::<usize>().map_err(|e| IoError::new(ErrorKind::InvalidInput, e.to_string()))?;
            let closure = move |old: usize| {old * c};
            return Ok(Box::new(closure));
        }
    } else if op == "+" {
        if rhs == "old" {
            let closure = |old: usize| { old.clone() + old };
            return Ok(Box::new(closure));
        } else {
            let c = rhs.parse::<usize>().map_err(|e| IoError::new(ErrorKind::InvalidInput, e.to_string()))?;
            let closure = move |old: usize| {old + c};
            return Ok(Box::new(closure));
        }
    }
    Err(IoError::new(ErrorKind::InvalidInput, "unknown operation"))
}

fn parse_test(line: &str) -> std::io::Result<Box<dyn Fn(usize) -> bool>> {
    let errf = || {IoError::new(ErrorKind::InvalidInput, "Wrong prefix on test line")};
    let cond = line.trim().strip_prefix("Test: ").ok_or_else(errf)?;

    let errf = || {IoError::new(ErrorKind::InvalidInput, "Could not parse condition")};
    let val = cond.split(' ').nth(2).ok_or_else(errf)?.parse::<usize>().map_err(|err| IoError::new(ErrorKind::InvalidInput, err.to_string()))?;
    let closure = move |test: usize| -> bool { test % val == 0 };
    Ok(Box::new(closure))
}

fn parse_true_target(line: &str) -> std::io::Result<usize> {
    let errf = || {IoError::new(ErrorKind::InvalidInput, "Wrong prefix on true target line")};
    let tar = line.trim().strip_prefix("If true: throw to monkey ").ok_or_else(errf)?;
    tar.parse::<usize>().map_err(|err| IoError::new(ErrorKind::InvalidInput, err.to_string()))
}

fn parse_false_target(line: &str) -> std::io::Result<usize> {
    let errf = || {IoError::new(ErrorKind::InvalidInput, "Wrong prefix on false target line")};
    let tar = line.trim().strip_prefix("If false: throw to monkey ").ok_or_else(errf)?;
    tar.parse::<usize>().map_err(|err| IoError::new(ErrorKind::InvalidInput, err.to_string()))
}

/// collective of monkeys == a troop of monkeys!
struct MonkeyTroop {
    items: Vec<VecDeque<usize>>,
    op: Vec<Box<dyn Fn(usize) -> usize>>,
    test_op: Vec<Box<dyn Fn(usize) -> bool>>,
    true_target: Vec<usize>,
    false_target: Vec<usize>,
    inspected_items: Vec<usize>,
}

impl MonkeyTroop {
    fn empty() -> Self {
        Self {
            items: Vec::new(),
            op: Vec::new(),
            test_op: Vec::new(),
            true_target: Vec::new(),
            false_target: Vec::new(),
            inspected_items: Vec::new(),
        }
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

fn solve1(monkeys: &mut MonkeyTroop) -> std::io::Result<()> {
    for _round in 0..20 {
        for i in 0..monkeys.len() {
            let mut qi = monkeys.items[i].clone();
            monkeys.inspected_items[i] += qi.len();
            while !qi.is_empty() {
                let olvl = qi.pop_front().expect("queue qi should be non-empty");
                let nlvl = (monkeys.op[i])(olvl) / 3;
                if (monkeys.test_op[i])(nlvl) {
                    let ti = monkeys.true_target[i];
                    monkeys.items[ti].push_back(nlvl);
                } else {
                    let fi = monkeys.false_target[i];
                    monkeys.items[fi].push_back(nlvl);
                }
            }
            monkeys.items[i] = qi;
        }
    }
    let mut inspected = monkeys.inspected_items.clone();
    inspected.sort();
    inspected.reverse();
    println!("solution 1: {} x {} = {}", inspected[0], inspected[1], inspected[0] * inspected[1]);
    Ok(())
}

fn solve2(monkeys: &mut MonkeyTroop) -> std::io::Result<()> {
    for _round in 0..10000 {
        for i in 0..monkeys.len() {
            let mut qi = monkeys.items[i].clone();
            monkeys.inspected_items[i] += qi.len();
            while !qi.is_empty() {
                let olvl = qi.pop_front().expect("queue qi should be non-empty");
                let nlvl = (monkeys.op[i])(olvl);
                if (monkeys.test_op[i])(nlvl) {
                    let ti = monkeys.true_target[i];
                    monkeys.items[ti].push_back(nlvl);
                } else {
                    let fi = monkeys.false_target[i];
                    monkeys.items[fi].push_back(nlvl);
                }
            }
            monkeys.items[i] = qi;
        }
        // normalization step
        for i in 0..monkeys.len() {
            for j in 0..monkeys.items[i].len() {
                let worry = monkeys.items[i][j];
                monkeys.items[i][j] = worry % MODULUS;
            }
        }
    }
    let mut inspected = monkeys.inspected_items.clone();
    inspected.sort();
    inspected.reverse();
    println!("solution 2: {} x {} = {}", inspected[0], inspected[1], inspected[0] * inspected[1]);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 2 {
        return Err(IoError::new(ErrorKind::InvalidInput, "Expected filename as argument"));
    }

    let file = File::open(&argv[1])?;
    let reader = std::io::BufReader::new(file);
    let mut monkeys = reader.lines().array_chunks::<7>().map(|chunk| -> std::io::Result<_> {
        let starting_items_str = chunk[1].as_ref().map_err(|e| IoError::new(ErrorKind::InvalidInput, e.to_string()))?;
        let items = parse_starting_items(&starting_items_str)?;
        let operation_str = chunk[2].as_ref().map_err(|e| IoError::new(ErrorKind::InvalidInput, e.to_string()))?;
        let op = parse_operation(&operation_str)?;
        let test_str = chunk[3].as_ref().map_err(|e| IoError::new(ErrorKind::InvalidInput, e.to_string()))?;
        let testop = parse_test(&test_str)?;
        let true_tar_str = chunk[4].as_ref().map_err(|e| IoError::new(ErrorKind::InvalidInput, e.to_string()))?;
        let true_tar = parse_true_target(true_tar_str)?;
        let false_tar_str = chunk[5].as_ref().map_err(|e| IoError::new(ErrorKind::InvalidInput, e.to_string()))?;
        let false_tar = parse_false_target(false_tar_str)?;
        Ok((items, op, testop, true_tar, false_tar))
    }).fold(MonkeyTroop::empty(), |mut acc, io_result| {
        if let Ok((items, op, testop, true_tar, false_tar)) = io_result {
            acc.items.push(items);
            acc.op.push(op);
            acc.test_op.push(testop);
            acc.true_target.push(true_tar);
            acc.false_target.push(false_tar);
            acc.inspected_items.push(0);
        } else if let Err(e) = io_result {
            panic!("error during parsing = {}", e.to_string());
        }
        acc
    });

    println!("read {} monkeys", monkeys.len());
    println!("MODOLUS {}", MODULUS);
    // solve1(monkeys)?;
    solve2(&mut monkeys)?;
    Ok(())
}