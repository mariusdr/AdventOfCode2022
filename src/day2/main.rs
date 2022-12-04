use std::env;
use std::fs::File;
use std::io::prelude::*;

fn calc_round_score(op: char, me: char) -> u32 {
    let mut outcome = 0u32;
    if op == 'A' && me == 'X' {
       outcome = 3;
    } 
    if op == 'A' && me == 'Y' {
        outcome = 6;
    }
    if op == 'B' && me == 'Y' {
        outcome = 3;
    }
    if op == 'B' && me == 'Z' {
        outcome = 6;
    }
    if op == 'C' && me == 'Z' {
        outcome = 3;
    }
    if op == 'C' && me == 'X' {
        outcome = 6;
    }
    if me == 'X' {
        outcome += 1;
    } else if me == 'Y' {
        outcome += 2;
    } else if me == 'Z' {
        outcome += 3;
    }
    outcome
}

// A = rock B = paper C = scissor
// X = loose Y = draw Z = win
fn select_turn(op: char, outcome: char) -> char {
    if outcome == 'X' {
        if op == 'A' {
            return 'C';
        } else if op == 'B' {
            return 'A';
        } else {
            return 'B';
        }
    } else if outcome == 'Z' {
        if op == 'A' {
            return 'B';
        } else if op == 'B' {
            return 'C';
        } else {
            return 'A';
        }
    }
    op // <- draw
}

fn calc_round_score2(op: char, outcome: char) -> u32 {
    let score = match outcome {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => 0,
    };
    let bonus = match select_turn(op, outcome) {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => 0,
    };
    score + bonus
}

fn game1(file: &File) -> std::io::Result<()> {
    let reader = std::io::BufReader::new(file);    
    let mut running_score = 0u32;
    for line in reader.lines() {
        let lstr = line?;
        let mut cs = lstr.chars();
        let opponent_turn = cs.nth(0).unwrap();
        let your_turn = cs.nth(1).unwrap();
        let score = calc_round_score(opponent_turn, your_turn);
        running_score += score;
    }
    println!("solution 1: {}", running_score);
    Ok(())
}

fn game2(file: &File) -> std::io::Result<()> {
    let reader = std::io::BufReader::new(file);    
    let mut running_score = 0u32;
    for line in reader.lines() {
        let lstr = line?;
        let mut cs = lstr.chars();
        let opponent_turn = cs.nth(0).unwrap();
        let outcome = cs.nth(1).unwrap();
        let score = calc_round_score2(opponent_turn, outcome);
        running_score += score;
    }
    println!("solution 1: {}", running_score);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 2 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Expected filename as argument"));
    }
    let file = File::open(&argv[1])?;
    game1(&file)?;
    let file = File::open(&argv[1])?;
    game2(&file)?;
    Ok(())
}
