use std::env;
use std::fs::File;
use std::io::prelude::*;

type Cell = Option<char>;

struct ReadDrawing {
    cells: Vec<Cell>,
    nrows: usize,
    ncols: usize,
}

impl ReadDrawing {
    fn open(file: &File) -> std::io::Result<Self> {
        let mut cells: Vec<Cell> = Vec::new();
        let mut nl = 0;
        let mut nr = 0;
        let reader = std::io::BufReader::new(file);
        for line in reader.lines() {
            let lstr = line?;
            if lstr.trim_start().starts_with("1") {
                break;
            }
            for ck in lstr.as_bytes().chunks(4) {
                if ck[1] != b' ' {
                    cells.push(Some(ck[1] as char))
                } else {
                    cells.push(None);
                }
                nr += 1;
            }
            nl += 1;
        }        
        Ok(Self { cells: cells, nrows: nl, ncols: nr / nl })
    }

    fn stack_col(&self, col: usize) -> Vec<Cell> {
        let mut stack: Vec<Cell> = Vec::with_capacity(self.nrows);
        for row in (0..self.nrows).rev() {
            let c = self.cells[row * self.ncols + col];
            if c.is_some() {
                stack.push(c);
            }
        }
        stack
    }

    fn stacks(&self) -> Vec<Vec<Cell>> {
        let mut stacks: Vec<Vec<Cell>> = Vec::with_capacity(self.ncols);
        for col in 0..self.ncols {
            stacks.push(self.stack_col(col));
        }
        stacks
    }
}

#[derive(Debug, Clone)]
struct Command {
    mov: usize,
    from: usize,
    to: usize,
}

struct ReadCommands {
    commands: Vec<Command>,
}

impl ReadCommands {
    fn open(file: &File) -> std::io::Result<Self> {
        let mut comms: Vec<Command> = Vec::new();
        let reader = std::io::BufReader::new(file);
        for line in reader.lines() {
            let lstr = line?;
            if !lstr.starts_with("move") {
                continue;
            }
            let mut it = lstr.split(' ');
            let c = Command { 
                mov: it.nth(1).unwrap().parse::<usize>().unwrap(),
                from: it.nth(1).unwrap().parse::<usize>().unwrap() - 1,
                to: it.nth(1).unwrap().parse::<usize>().unwrap() - 1,
            };
            comms.push(c);
        }
        Ok(Self { commands: comms })
    }

    fn commands(&self) -> Vec<Command> {
        self.commands.clone()
    }
}

#[inline]
fn move_single(stacks: &mut Vec<Vec<Cell>>, from_idx: usize, to_idx: usize) {
    let c = stacks[from_idx].pop().unwrap();
    // println!("move {} from {} to {}", c.unwrap(), from_idx, to_idx);
    stacks[to_idx].push(c);
}

fn challenge1(filepath: &str) -> std::io::Result<()> {
    let mut stacks = ReadDrawing::open(&File::open(filepath)?)?.stacks();
    let commands = ReadCommands::open(&File::open(filepath)?)?.commands();

    for command in commands {
        for i in 0..command.mov {
            move_single(&mut stacks, command.from, command.to);
        }
    }

    for s in &stacks {
        print!("{:?}", s.last().unwrap().unwrap());
    }
    print!("\n");
    Ok(())
}

#[inline]
fn move_multiple(stacks: &mut Vec<Vec<Cell>>, cnt: usize, from_idx: usize, to_idx: usize) {
    let mut tmp: Vec<Cell> = Vec::with_capacity(cnt);
    for _ in 0..cnt {
        let c = stacks[from_idx].pop().unwrap();
        tmp.push(c);
    }
    for _ in 0..cnt {
        let c = tmp.pop().unwrap();
        stacks[to_idx].push(c);
    }
}

fn challenge2(filepath: &str) -> std::io::Result<()> {
    let mut stacks = ReadDrawing::open(&File::open(filepath)?)?.stacks();
    let commands = ReadCommands::open(&File::open(filepath)?)?.commands();

    for command in commands {
        move_multiple(&mut stacks, command.mov, command.from, command.to);
    }

    for s in &stacks {
        print!("{:?}", s.last().unwrap().unwrap());
    }
    print!("\n");
    Ok(())
}


fn main() -> std::io::Result<()> {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 2 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Expected filename as argument"));
    }
    challenge1(&argv[1])?;
    challenge2(&argv[1])?;
    Ok(())
}
