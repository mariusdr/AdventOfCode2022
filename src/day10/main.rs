use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum OpCode {
    Noop,
    AddX(i32),
}

fn in_sprite_off(sprite_mid: i32, offset: i32, curpixel: i32) -> bool {
    let spritepos = sprite_mid + offset;
    if 0 <= spritepos && spritepos < 40 {
        return spritepos == curpixel;
    }
    false
}

fn in_sprite(sprite_mid: i32, curpixel: i32) -> bool {
    in_sprite_off(sprite_mid, -1, curpixel) || 
    in_sprite_off(sprite_mid, 0, curpixel) ||
    in_sprite_off(sprite_mid, 1, curpixel)
}

fn main() -> std::io::Result<()> {
    use std::io::ErrorKind;
    type IoError = std::io::Error;
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 2 {
        return Err(IoError::new(ErrorKind::InvalidInput, "Expected filename as argument"));
    }

    let file = File::open(&argv[1])?;
    let reader = std::io::BufReader::new(file);
    let lineit = reader.lines().take_while(|ln| {
        if let Ok(lnstr) = ln {
            return !lnstr.is_empty();
        } 
        false
    }).map(|ln| -> std::io::Result<OpCode> {
        let lnstr = ln?; 
        let mut parts = lnstr.split(' ');
        let opname = parts.next().ok_or(IoError::new(ErrorKind::InvalidInput, "expected opcode"))?;
        let opcode: OpCode;
        if opname == "noop" {
            opcode = OpCode::Noop;
        } else if opname == "addx" {
            let vstr = parts.next().ok_or(IoError::new(ErrorKind::InvalidInput, "expected opcode"))?;
            let v = vstr.parse::<i32>().map_err(|e| IoError::new(ErrorKind::InvalidData, e.to_string()))?;
            opcode = OpCode::AddX(v);
        } else {
            return Err(IoError::new(ErrorKind::InvalidInput, "unknown opcode"));
        }
        Ok(opcode)
    });

    let prog = lineit.collect::<Result<Vec<_>,_>>()?;
    let mut exec: Vec<OpCode> = Vec::new();
    for opc in prog {
        if let OpCode::AddX(x) = opc {
            exec.push(OpCode::Noop);
            exec.push(OpCode::AddX(x));
        } else {
            exec.push(OpCode::Noop);
        }
    }

    let mut rx: i32 = 1;
    let mut next_probe = 20;
    let mut total_sig = 0;

    let mut framebuf = [[' ' as u8; 40]; 6];
    let mut currow = 0;

    for (cycle, opc) in exec.iter().enumerate() {
        if cycle + 1 == next_probe {
            let signal = (cycle + 1) as i32 * rx;
            if next_probe <= 220 {
                total_sig += signal;
            }
            next_probe += 40;
        }
        
        let curpixel: i32 = cycle as i32 % 40;
        if in_sprite(rx, curpixel) {
            framebuf[currow][curpixel as usize] = '#' as u8;
        } else {
            framebuf[currow][curpixel as usize] = '.' as u8;
        }
        if curpixel == 39 {
            currow += 1;
        }
        
        if let OpCode::AddX(x) = opc {
            rx += x;
        }
    }
    println!("solution1: {}", total_sig);
    println!("solution2: ");
    for row in 0..6 {
        println!("{}", String::from_utf8_lossy(&framebuf[row]));
    }
    Ok(())
}