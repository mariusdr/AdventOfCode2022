use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Grid {
    data: Vec<u8>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn read_file(file: File) -> std::io::Result<Grid> {
        let reader = std::io::BufReader::new(file);
        let mut data: Vec<u8> = Vec::new();
        let mut rows = 0;
        let mut cols = 0;
        for line in reader.lines() {
            let lstr = line?;
            if rows == 0 {
                rows = lstr.as_bytes().len();
            }
            data.extend(lstr.as_bytes().iter().map(|b| b - '0' as u8));
            cols += 1;
        }
        Ok(Self { data: data, rows: rows, cols: cols })
    }

    fn empty(rows: usize, cols: usize) -> Self {
        let data = vec![0u8; rows * cols];
        Self { data: data, rows: rows, cols: cols}
    }

    fn read(&self, i: usize, j: usize) -> Option<u8> {
        if i > self.rows || j > self.cols {
            return None;
        }
        Some(self.data[i * self.cols + j])
    }

    fn set(&mut self, i: usize, j: usize, val: u8) -> Option<()> {
        if i > self.rows || j > self.cols {
            return None;
        }
        self.data[i * self.cols + j] = val;
        Some(())
    }
}


fn compute_vmap(grid: &Grid) -> Grid {
    let mut vmap = Grid::empty(grid.rows, grid.cols);
    
    let mut rvmap = Grid::empty(grid.rows, grid.cols);
    for i in 0..grid.rows {
        let mut max_height = grid.read(i, 0).unwrap();
        rvmap.set(i, 0, 1).unwrap();
        for j in 0..grid.cols {
            if grid.read(i, j).unwrap() > max_height {
                rvmap.set(i, j, 1).unwrap();
                max_height = grid.read(i, j).unwrap();
            }
        }
    }
    
    let mut uvmap = Grid::empty(grid.rows, grid.cols);
    for j in 0..grid.cols {
        let mut max_height = grid.read(0, j).unwrap();
        uvmap.set(0, j, 1).unwrap();
        for i in 0..grid.rows {
            if grid.read(i, j).unwrap() > max_height {
                uvmap.set(i, j, 1).unwrap();
                max_height = grid.read(i, j).unwrap();
            }
        }
    }

    let mut lvmap = Grid::empty(grid.rows, grid.cols);
    for i in 0..grid.rows {
        let mut max_height = grid.read(i, grid.cols-1).unwrap();
        lvmap.set(i, grid.cols-1, 1).unwrap();
        for j in (0..grid.cols-1).rev() {
            if grid.read(i, j).unwrap() > max_height {
                lvmap.set(i, j, 1).unwrap();
                max_height = grid.read(i, j).unwrap();
            }
        }
    }

    let mut dvmap = Grid::empty(grid.rows, grid.cols);
    for j in 0..grid.cols {
        let mut max_height = grid.read(grid.rows-1, j).unwrap();
        dvmap.set(grid.rows-1, j, 1).unwrap();
        for i in (0..grid.rows-1).rev() {
            if grid.read(i, j).unwrap() > max_height {
                dvmap.set(i, j, 1).unwrap();
                max_height = grid.read(i, j).unwrap();
            }
        } 
    }
    
    for i in 0..grid.rows {
        for j in 0..grid.cols {
            vmap.set(i, j, 
                    rvmap.read(i, j).unwrap() | lvmap.read(i, j).unwrap() |
                    uvmap.read(i, j).unwrap() | dvmap.read(i, j).unwrap());
        }
    }
    
    vmap 
}

fn compute_max_scenic_score(grid: &Grid) -> usize {
    let mut max_score = 0;
    for i in 0..grid.rows {
        for j in 0..grid.cols {
            let height = grid.read(i, j).unwrap();
            let mut right_dist = 0;
            for dj in j+1..grid.cols {
                right_dist += 1;
                if grid.read(i, dj).unwrap() >= height {
                    break;
                }
            }
            let mut left_dist = 0; 
            for dj in (0..j).rev() {
                left_dist += 1;
                if grid.read(i, dj).unwrap() >= height {
                    break;
                }
            }
            let mut down_dist = 0;
            for di in i+1..grid.rows {
                down_dist += 1;
                if grid.read(di, j).unwrap() >= height {
                    break;
                }
            }
            let mut up_dist = 0;
            for di in (0..i).rev() {
                up_dist += 1;
                if grid.read(di, j).unwrap() >= height {
                    break;
                }
            } 
            let score = left_dist * right_dist * up_dist * down_dist;    
            max_score = std::cmp::max(max_score, score);
        }
    }
    max_score
}

fn main() -> std::io::Result<()> {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 2 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Expected filename as argument"));
    }
    let file = File::open(&argv[1])?;
    let grid = Grid::read_file(file)?;
    let vmap = compute_vmap(&grid);
    println!("sum visible: {}", vmap.data.iter().map(|x| *x as usize).sum::<usize>());
    let score = compute_max_scenic_score(&grid);
    println!("max scenic score is {}", score);
    Ok(())
}
