use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

type Pos = (i32, i32);

#[derive(Debug)]
enum Step {
    Up(i32),
    Down(i32),
    Right(i32),
    Left(i32)
}

fn update_headpos(head: Pos, step: Step) -> Pos {
    let (x, y) = head;
    match step {
        Step::Up(dx) => (x + dx, y),
        Step::Down(dx) => (x - dx, y),
        Step::Right(dy) => (x, y + dy),
        Step::Left(dy) => (x, y - dy),
    }
}

fn update_tailpos(tail: Pos, head: Pos) -> Pos {
    let (hx, hy) = head;
    let (tx, ty) = tail;
    let (dx, dy) = (hx - tx, hy - ty);
    // directional offset
    let (sx, sy) = (dx.signum(), dy.signum());
    // offset to fix vertical states
    let (mut vx, mut vy) = (0, 0);
    if sx * dx + sy * dy > 2 {
        (vx, vy) = (sx * (2 - sx * dx), sy * (2 - sy * dy));
    }
    (tx + dx - sx + vx, ty + dy - sy + vy)
}

fn read_step(direction: &str) -> Step {
    match direction {
        "U" => Step::Up(1),
        "D" => Step::Down(1),
        "R" => Step::Right(1),
        "L" => Step::Left(1),
        _ => panic!("invalid direction given")
    }
}

fn visualize<const XI: i32, const XN: i32, const YI: i32, const YN: i32>(head: Pos, tails: &[Pos]) {
    let in_tails = |cur| {
        for (i, t) in tails.iter().enumerate() {
            if *t == cur {
                return i + 1;
            }
        }
        0
    };
    for row in (XI..XN).rev() {
        for col in YI..YN {
            let cur = (row, col);
            if cur == head {
                print!("H"); 
            } else if in_tails(cur) > 0 {
                print!("{}", in_tails(cur)); 
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("")
}

fn main() -> std::io::Result<()> {
    use std::io::ErrorKind;
    type IoError = std::io::Error;
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 2 {
        return Err(IoError::new(ErrorKind::InvalidInput, "Expected filename as argument"));
    }

    // initial values
    let mut head = (0, 0);
    let mut tails = [(0, 0); 9];

    // tracker keeps track of the positions the tail visited already
    let mut tracker: HashSet<Pos> = HashSet::new();
    // how many unique positions the tail visited
    let mut cnt = 0;

    let file = File::open(&argv[1])?;
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        let lstr = line?;
        let mut parts = lstr.split(' ');
        let dir = parts.next().ok_or_else(|| IoError::new(ErrorKind::InvalidInput, "no direction given"))?;
        let stepstr = parts.next().ok_or_else(|| IoError::new(ErrorKind::InvalidInput, "no steps given"))?;
        let steps = stepstr.parse::<i32>().map_err(|err| IoError::new(ErrorKind::InvalidInput, err.to_string()))?;
        
        for _ in 0..steps {
            let s = read_step(dir);
            head = update_headpos(head, s);
            let mut prec = head;
            for i in 0..tails.len() {
                tails[i] = update_tailpos(tails[i], prec);
                prec = tails[i];
            }

            if tracker.insert(tails[tails.len() - 1]) {
                cnt += 1;
            }
        }
        // visualize::<-20,20,-20,20>(head, &tails);
    }
    println!("solution: # unique tail positions = {}", cnt);
    Ok(())
}
