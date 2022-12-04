use std::env;
use std::fs::File;
use std::io::prelude::*;

// map [a..zA..Z] -> [1..26..52]
fn offset(c: char) -> u8 {
    let off;
    if 'a' <= c && c <= 'z' {
        off = (c as u8) - ('a' as u8);
    } else {
        off = (c as u8) - ('A' as u8) + 26;
    }
    off
}

fn offset_to_char(off: u8) -> char {
    if off < 26 {
        return (off + 'a' as u8) as char
    } else {
        return (off - 26 + 'A' as u8) as char
    }
}

fn onehot(c: char) -> u64 {
    let mask: u64 = 1 << offset(c);
    mask
}

fn first_set_bit(mask: u64) -> Option<u8> {
    for pos in 0..64 {
        let tmp = 1 << pos;
        if tmp & mask != 0 {
            return Some(pos);
        }
    }
    None
}

fn inverse_onehot(mask: u64) -> Option<char> {
    let off = first_set_bit(mask)?;
    Some(offset_to_char(off))
}

fn find_dup1(input: &str) -> Option<char> {
    let mut coll = 0u64;
    let ilen = input.len();

    for (i, c) in input.chars().enumerate() {
        if i < ilen / 2 {
            coll |= onehot(c);
        } else {
            let mask = onehot(c);
            if mask & coll > 0 {
                return Some(c);
            }
        }
    }
    None
}

/// O(n) solution: Encode characters a..zA..Z into bitmaps
/// a = 001 b = 010 c = 100 ...
/// Or the first half of the input string into one bitmask coll, then 
/// look for the common character using 
///     coll & mask of c > 0 
/// which is only true if c is contained in both halfs.
fn challenge1(file: File) -> std::io::Result<()> {
    let reader = std::io::BufReader::new(file);
    let mut valsum = 0u64;
    for line in reader.lines() {
        let c = find_dup1(&line?).unwrap();
        let value = offset(c) + 1;
        // println!("duplicate is {} with value {}", c, value);
        valsum += value as u64;
    }
    println!("solution1: {}", valsum);
    Ok(())
}

fn to_mask(input: &str) -> u64 {
    let mut mask = 0u64;
    for c in input.chars() {
        mask |= onehot(c);
    }
    mask
}

fn find_common(mask1: u64, mask2: u64, mask3: u64) -> char {
    let coll = mask1 & mask2 & mask3;
    let c = inverse_onehot(coll).unwrap();
    c 
}

/// O(n) solution: For each of the three input strings compute the
/// bitmap from challenge 1. Then the common char is the only bit set 
/// in 
///     mask of str 1 & mask of str 2 & mask of str3.
fn challenge2(file: File) -> std::io::Result<()> {
    let reader = std::io::BufReader::new(file);
    let mut iter = reader.lines();
    let mut valsum = 0u64;
    loop {
        let mask1;
        let n = iter.next();
        if n.is_none() {
            break;
        } else {
            mask1 = to_mask(&n.unwrap()?);
        }
        
        let mask2;
        let n = iter.next();
        if n.is_none() {
            break;
        } else {
            mask2 = to_mask(&n.unwrap()?);
        }

        let mask3;
        let n = iter.next();
        if n.is_none() {
            break;
        } else {
            mask3 = to_mask(&n.unwrap()?);
        }
        
        let c = find_common(mask1, mask2, mask3);
        let value = offset(c) + 1;
        valsum += value as u64;
    }
    println!("solution2: {}", valsum);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 2 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Expected filename as argument"));
    }
    challenge1(File::open(&argv[1])?)?;
    challenge2(File::open(&argv[1])?)?;
    Ok(())
}
