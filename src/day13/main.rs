#![feature(iter_array_chunks)]
#![feature(iterator_try_collect)]
#![feature(let_chains)]
use std::env;
use std::fs::File;
use std::io::prelude::*;

use std::io::ErrorKind;
type IoError = std::io::Error;

const LIST_BEGIN: u8 = '[' as u8;
const LIST_END: u8 = ']' as u8;
const SEP: u8 = ',' as u8;

#[derive(Debug, PartialEq, Clone)]
enum Expr {
    Nil,
    Atom(u32),
    List(Vec<Expr>)
}

fn parse_atom(xs: &[u8]) -> Option<(Expr, &[u8])> {
    let mut offset = 0;
    for i in 0..xs.len() {
        let x = xs[i];
        if x == SEP || x == LIST_END || x == LIST_BEGIN {
            break;
        }
        offset += 1;
    }
    if offset == 0 {
        return Some((Expr::Nil, xs));
    }
    let n = String::from_utf8_lossy(&xs[0..offset]).parse::<u32>().unwrap();
    if offset < xs.len() && xs[offset] == SEP {
        offset += 1;
    }
    Some((Expr::Atom(n), &xs[offset..]))
}

fn parse_atoms(xs: &[u8]) -> Option<(Vec<Expr>, &[u8])> {
    let mut atoms: Vec<Expr> = Vec::new();
    let mut cursor = xs;
    loop {
        // println!("parser at {}", String::from_utf8_lossy(cursor));
        if let Some((atom, ys)) = parse_atom(cursor) {
            if atom == Expr::Nil {
                break;
            }
            atoms.push(atom);
            cursor = ys;
        } else {
            return None;
        }
    }
    Some((atoms, cursor))
}

fn parse_list(xs: &[u8]) -> Option<(Expr, &[u8])> {
    let mut exprs: Vec<Expr> = Vec::new();
    let mut cursor = xs;

    if cursor[0] != LIST_BEGIN {
        println!("error: expected list begin at cursor = {}", String::from_utf8_lossy(cursor));
    }
    cursor = &cursor[1..];

    loop {
        if cursor.len() == 0 {
            break;
        }
        // println!("cursor at {}", String::from_utf8_lossy(cursor));

        if cursor[0] == LIST_END {
            cursor = &cursor[1..];
            if cursor.len() > 1 && cursor[0] == SEP {
                cursor = &cursor[1..];
            }
            break;
        }
        if cursor[0] == LIST_BEGIN {
            if let Some((ls, xs)) = parse_list(cursor) {
                if ls == Expr::Nil {
                    break;
                }
                exprs.push(ls);
                cursor = xs;
            } else {
                return None;
            }
        } else {
            if let Some((mut atoms, xs)) = parse_atoms(cursor) {
                exprs.append(&mut atoms);
                cursor = xs;
            } else {
                return None;
            }
        }
    }
    Some((Expr::List(exprs), cursor))
}

impl PartialOrd for Expr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let Expr::Atom(x) = self && let Expr::Atom(y) = other {
            return x.partial_cmp(y);
        }
        if let Expr::Atom(x) = self && let Expr::List(_) = other {
            let lhs = Expr::List(vec![Expr::Atom(*x)]);
            return lhs.partial_cmp(other);
        }
        if let Expr::List(_) = self && let Expr::Atom(y) = other {
            let rhs = Expr::List(vec![Expr::Atom(*y)]);
            return self.partial_cmp(&rhs);
        }
        if let Expr::List(xs) = self && let Expr::List(ys) = other {
            let slen = std::cmp::min(xs.len(), ys.len());
            for i in 0..slen {
                let r = xs[i].partial_cmp(&ys[i]).unwrap();
                if r != std::cmp::Ordering::Equal {
                    return Some(r);
                }
            }
            if xs.len() == ys.len() {
                return Some(std::cmp::Ordering::Equal);
            } else if xs.len() < ys.len() {
                return Some(std::cmp::Ordering::Less);
            } else {
                return Some(std::cmp::Ordering::Greater);
            }
        }
        None
    }
}

fn solve2(file: &File) -> std::io::Result<()> {
    let reader = std::io::BufReader::new(file); 
    let mut exprs: Vec<Expr> = reader.lines().array_chunks::<3>().flat_map(|chunk| {
        let c0 = chunk[0].as_ref().unwrap();
        let c1 = chunk[1].as_ref().unwrap();
        let inp = c0.as_bytes();
        let (expr1, _) = parse_list(inp).unwrap();
        let inp = c1.as_bytes();
        let (expr2, _) = parse_list(inp).unwrap();
        
        vec![expr1, expr2].into_iter()
    }).collect();

    let (e1, _) = parse_list(b"[[2]]").unwrap();
    let (e2, _) = parse_list(b"[[6]]").unwrap();
    exprs.push(e1.clone());
    exprs.push(e2.clone());
    exprs.sort_by(|e1, e2| e1.partial_cmp(e2).unwrap());

    // for expr in exprs {
    //     println!("{:?}", expr);
    // }

    let mut idx1 = 1;
    let mut idx2 = 1;
    for (i, e) in exprs.iter().by_ref().enumerate() {
        let o = e.partial_cmp(&e1).unwrap();
        if o == std::cmp::Ordering::Equal {
            idx1 += i;
        }
        let o = e.partial_cmp(&e2).unwrap();
        if o == std::cmp::Ordering::Equal {
            idx2 += i;
        }
    }
    println!("solution 2: idx 1 = {} idx 2 = {}, prod = {}", idx1, idx2, idx1 * idx2);
    Ok(())
}

fn solve1(file: &File) -> std::io::Result<()> {
    let mut cnt = 0;
    let mut idx = 0;
    let reader = std::io::BufReader::new(file);
    reader.lines().array_chunks::<3>().for_each(|chunk|{
        idx += 1;
        let c0 = chunk[0].as_ref().unwrap();
        let c1 = chunk[1].as_ref().unwrap();
        let inp = c0.as_bytes();
        let (expr1, _) = parse_list(inp).unwrap();
        // println!("{:?}", expr1);
        let inp = c1.as_bytes();
        let (expr2, _) = parse_list(inp).unwrap();
        // println!("{:?}", expr2);
        let o = expr1.partial_cmp(&expr2).unwrap();
        // println!("---> {:?}", o);
        if o == std::cmp::Ordering::Less {
            cnt += idx;
        }
    });
    println!("solution 1: score = {}", cnt);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 2 {
        return Err(IoError::new(ErrorKind::InvalidInput, "Expected filename as argument"));
    }

    let file = File::open(&argv[1])?;
    solve1(&file)?;
    solve2(&file)?;

    Ok(())
}