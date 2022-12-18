#![feature(iterator_try_collect)]
use std::borrow::Borrow;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::btree_map::OccupiedEntry;
use std::env;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::io::ErrorKind;

type IoError = std::io::Error;

type Cube = (i32, i32, i32);

fn cabs(c: Cube) -> i32 {
    c.0.abs() + c.1.abs() + c.2.abs()
}

fn cdiff(lhs: Cube, rhs: Cube) -> i32 {
    (lhs.0 - rhs.0).abs() + (lhs.1 - rhs.1).abs() + (lhs.2 - rhs.2).abs()
}

fn vdiff(lhs: Cube, rhs: Cube) -> Cube {
    (lhs.0 - rhs.0, lhs.1 - rhs.1, lhs.2 - rhs.2)
}

fn vadd(lhs: Cube, rhs: Cube) -> Cube {
    (lhs.0 + rhs.0, lhs.1 + rhs.1, lhs.2 + rhs.2)
}

fn vmult(c: Cube, s: i32) -> Cube {
    (c.0 * s, c.1 * s, c.2 * s)
}

fn vdiv(c: Cube, s: i32) -> Cube {
    (c.0 / s, c.1 / s, c.2 / s)
}

fn solve1(cubes: &Vec<Cube>) -> std::io::Result<()> {
    let ftotal = cubes.iter().map(|c1| {
        let n = cubes.iter().filter(|&c2| cdiff(*c1, *c2) == 1).count();
        6 - n 
    }).reduce(|acc, f| acc + f).unwrap();
    println!("solution 1: free faces total {}", ftotal);
    Ok(())
}

fn occupied_faces<'a>(c1: &'a Cube, cubes: &'a Vec<Cube>) -> impl Iterator<Item = Cube> + 'a {
    cubes.iter().filter(|&c2| cdiff(*c1, *c2) == 1).map(|c2| vdiff(*c1, *c2))
}

fn face_each_other(lhs: &Cube, rhs: &Cube) -> bool {
    (lhs.0 != rhs.0 && lhs.1 == rhs.1 && lhs.2 == rhs.2) ||
    (lhs.0 == rhs.0 && lhs.1 != rhs.1 && lhs.2 == rhs.2) ||
    (lhs.0 == rhs.0 && lhs.1 == rhs.1 && lhs.2 != rhs.2) 
}

fn cube_ray<'a>(first: &'a Cube, second: &'a Cube) -> Option<impl Iterator<Item = Cube> + 'a> {
    if face_each_other(first, second) {
        let vdir = vdiff(*first, *second);
        let len = cabs(vdir);
        let dir = vdiv(vdir, len);
        let it = (1..len).map(move |step| vadd(*first, vmult(dir, -1 * step as i32)));
        return Some(it)
    }
    None
}

// The trap here is that you need to consider enclosed spaces larger than a single cube 
// as opposed to the trivial example from the task description.
// Not the most efficient solution but it works: ...
fn solve2(cubes: &Vec<Cube>) -> std::io::Result<()> {
    // First, fill all potential free spaces between two cubes that face each other
    // by casting "rays" of cubes between them. The HashSet is there to avoid endless 
    // dups.. 
    let mut candidates: HashSet<Cube> = HashSet::new();
    for c1 in cubes {
        for c2 in cubes {
            if let Some(rayit) = cube_ray(&c1, &c2) {
                let ray: Vec<Cube> = rayit.collect();
                if ray.is_empty() {
                    continue;
                }
                let broken = ray.iter().map(|c| cubes.contains(c)).any(|v| v == true);
                if broken {
                    continue;
                }
                for c in ray {
                    candidates.insert(c);
                }
            }
        }
    } 

    // A cube in an enclosed space is itself enclosed by cubes from the input (cubes vec)
    // or other enclosed space cubes (candidates) on all sides.
    // So remove any cube that does not have this invariant. Iterate until no further invalid
    // cube was found.
    let mut to_filter = candidates.iter().map(|&c| c).collect::<Vec<Cube>>();
    loop {
        let mut next_filter: Vec<Cube> = Vec::new();
        for c in &to_filter {
            let occupied_cubes = occupied_faces(c, cubes).count();
            let occupied_filter = occupied_faces(c, &to_filter).count();
            if occupied_cubes + occupied_filter == 6 {
                next_filter.push(*c);
            }
        }
        if next_filter.len() == to_filter.len() {
            println!("no additional cubes filtered out, leave..");
            break;
        }
        to_filter = next_filter.drain(0..).collect();
    }

    // to_filter contains all cubes that are in an enclosed space now. Merge with
    // input cubes and compute solution like in part 1.
    let mut all_cubes = cubes.clone();
    all_cubes.append(&mut to_filter);
    let ftotal = all_cubes.iter().map(|c1| {
        let n = all_cubes.iter().filter(|&c2| cdiff(*c1, *c2) == 1).count();
        6 - n 
    }).reduce(|acc, f| acc + f).unwrap();
    println!("solution 2: free faces total {}", ftotal);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 2 {
        return Err(IoError::new(ErrorKind::InvalidInput, "Expected filename as argument"));
    }
    let file = File::open(&argv[1])?;
    let reader = std::io::BufReader::new(file);

    let mut cubes: Vec<Cube> = reader.lines().map(|ln| -> std::io::Result<Cube> {
        let lstr = ln?;
        let mut parts = lstr.split(",").map(|p| p.parse::<i32>().map_err(|e| IoError::new(ErrorKind::InvalidInput, e.to_string())));
        let x = parts.next().ok_or(IoError::new(ErrorKind::InvalidInput, "part iterator returned None"))?;
        let y = parts.next().ok_or(IoError::new(ErrorKind::InvalidInput, "part iterator returned None"))?;
        let z = parts.next().ok_or(IoError::new(ErrorKind::InvalidInput, "part iterator returned None"))?;
        Ok((x?, y?, z?))
    }).try_collect()?;
    cubes.sort();
    // solve1(&cubes)?;
    solve2(&cubes)?;

    Ok(())
}
