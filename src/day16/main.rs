#![feature(iterator_try_collect)]
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::ErrorKind;
type IoError = std::io::Error;

fn parse_valve_id(inp: &str) -> Option<(String, &str)> {
    let mut curr = &inp[0..];
    let valv_off = curr.find("Valve ")?;
    curr = &curr[valv_off + 6..];
    let offset = curr.chars().take_while(|c| !c.is_whitespace()).count();
    let id = String::from(&curr[0..offset]);
    Some((id, &curr[offset..]))
}

fn parse_rate(inp: &str) -> Option<(u32, &str)> {
    let mut curr = &inp[0..];
    let rate_off = curr.find("rate=")?;
    curr = &curr[rate_off + 5..];
    let offset = curr.chars().take_while(|c| c.is_digit(10)).count();
    curr[0..offset].parse::<u32>().ok().map(|p| {
        (p, &curr[offset..])
    })
}

fn parse_valves(inp: &str) -> Option<(Vec<String>, &str)> {
    let mut curr = &inp[0..];
    if let Some(valves_off) = curr.find("valves") {
        curr = &curr[valves_off + 6..];
        let vs: Vec<String> = curr.split(", ").map(|part| String::from(part.trim())).collect();
        let off = vs.iter().map(|s| s.len()).sum::<usize>() + (vs.len() - 1) * 2 + 1;
        return Some((vs, &curr[off..]));
    }
    if let Some(valve_off) = curr.find("valve") {
        curr = &curr[valve_off + 6..];
        let v = String::from(curr); 
        let off = v.len();
        return Some((vec![v], &curr[off..]));
    }
    None
}

fn parse_line(inp: &str) -> Option<(String, u32, Vec<String>, &str)> {
    let (v, rem) = parse_valve_id(inp)?;
    let (rate, rem) = parse_rate(rem)?;
    let (vs, rem) = parse_valves(rem)?;
    Some((v, rate, vs, rem))
}

#[derive(Debug, Clone)]
struct Matrix {
    n: usize,
    data: Vec<Vec<u32>>,
}

impl Matrix {
    fn new(n: usize) -> Self {
        let data = vec![vec![0u32; n]; n];
        Self {n, data}
    }

    fn at(&self, i: usize, j: usize) -> Option<u32> {
        if i >= self.n || j >= self.n {
            return None;
        }
        Some(self.data[i][j])
    }

    fn set(&mut self, i: usize, j: usize, val: u32) -> Option<()> {
        if i >= self.n || j >= self.n {
            return None;
        }
        self.data[i][j] = val;
        Some(())
    }
}

#[derive(Debug)]
struct NodeIdMap {
    next_id: usize,
    map: HashMap<String, usize>,
    rev: HashMap<usize, String>,
}

impl  NodeIdMap {
    fn new() -> Self {
        Self { next_id: 0, map: HashMap::new(), rev: HashMap::new() }
    }

    fn get(&mut self, node: &str) -> usize {
        if let Some(id) = self.map.get(node) {
            return *id;
        }
        let id = self.next_id;
        self.map.insert(node.to_string(), id);
        self.next_id += 1;
        self.rev.insert(id, node.to_string());
        id
    }
}


fn parse_graph(file: &File, idmap: &mut NodeIdMap) -> std::io::Result<(Matrix, Vec<u32>)> {
    let lines: Vec<String> = std::io::BufReader::new(file).lines().try_collect()?;
    let n = lines.len();
    let mut graph = Matrix::new(n);
    let mut flows = vec![0u32; n];

    for line in lines {
        if let Some((v, rate, vs, _)) = parse_line(&line) {
            let vi = idmap.get(&v);
            for w in vs {
                let vj = idmap.get(&w);
                graph.set(vi, vj, 1);
            }
            flows[vi] = rate;
        }
    }
    Ok((graph, flows))
}

fn floyd_warshall(adjmat: &Matrix) -> Option<Matrix> {
    let n = adjmat.n;
    let mut dist = adjmat.clone();
    for i in 0..n {
        for j in 0..n {
            if dist.at(i, j)? != 1 {
                dist.set(i, j, 99999999)?;
            }
        }
    }
    for i in 0..n {
        dist.set(i, i, 0)?;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let fst = dist.at(i, j)?;
                let snd = dist.at(i, k)? + dist.at(k, j)?;
                if fst > snd {
                    dist.set(i, j, snd);
                }
            }
        }
    }
    Some(dist)
}

#[derive(Clone)]
struct DfsSolver<'a> {
    marked: Vec<bool>,
    distmat: &'a Matrix,
    flows: &'a Vec<u32>,
}

impl<'a> DfsSolver<'a> {
    fn new(distmat: &'a Matrix, flows: &'a Vec<u32>) -> Self {
        Self { marked: vec![false; distmat.n], distmat, flows }
    }

    fn is_marked(&self, node: usize) -> bool {
        self.marked[node]
    }

    fn mark(&mut self, node: usize) {
        if !self.is_marked(node) {
            self.marked[node] = true;
        }
    }

    fn unmark(&mut self, node: usize) {
        if self.is_marked(node) {
            self.marked[node] = false;
        }
    }

    fn targets(&self, ni: usize, depth: u32) -> Vec<(usize, u32)> {
        let mut ts: Vec<(usize, u32)> = Vec::new();
        for nj in 0..self.distmat.n {
            let dist = self.distmat.at(ni, nj).unwrap();
            
            // skip nodes where flow = 0 --> cuts down recursion to a manageable level!
            if depth >= dist && dist > 0 && !self.is_marked(nj) && self.flows[nj] > 0 {
                ts.push((nj, dist));
            }
        }
        ts
    }

    fn dfs(&mut self, node: usize, mut depth: u32) -> u32 {
        if depth <= 1 {
            return 0;
        }

        let mut add_flow = 0;
        if self.flows[node] > 0 {
            self.mark(node);
            depth -= 1;
            add_flow = depth * self.flows[node];
        }

        let mut max_flow = 0;
        let mut max_flow_markings = self.marked.clone();
        for (tar, dist) in self.targets(node, depth) {
            let mut s = self.clone();
            let f = s.dfs(tar, depth - dist);
            if f > max_flow {
                max_flow_markings = s.marked.clone();
                max_flow = f;
            }
        }
        self.marked = max_flow_markings;         

        // println!("at node {} with total flow {}", node, add_flow + max_flow);
        add_flow + max_flow
    }
    
    fn dfs2(&mut self, node: usize, mut depth: u32, snode: usize, sdepth: u32, is_elephant: bool) -> u32 {
        if depth <= 1 {
            return 0;
        }

        let mut add_flow = 0;
        if self.flows[node] > 0 {
            self.mark(node);
            depth -= 1;
            add_flow = depth * self.flows[node];
        }

        let mut max_flow = 0;
        let mut max_flow_markings = self.marked.clone();
        for (tar, dist) in self.targets(node, depth) {
            let mut s = self.clone();
            let f = s.dfs2(tar, depth - dist, snode, sdepth, is_elephant);
            if f > max_flow {
                max_flow_markings = s.marked.clone();
                max_flow = f;
            }
        }
        self.marked = max_flow_markings;         

        let mut max_flow_e = 0;
        if is_elephant == false {
            max_flow_e = self.dfs2(snode, sdepth, snode, sdepth, true);
        }

        // println!("at node {} with total flow {}", node, add_flow + max_flow);
        add_flow + max_flow + max_flow_e
    }
}

fn solve1(file: &File) -> std::io::Result<()> {
    let mut idmap = NodeIdMap::new();
    let (adjmat, flows) = parse_graph(&file, &mut idmap)?;
    let distmat = floyd_warshall(&adjmat).unwrap();
    let start_id = idmap.get("AA");
    
    let mut solver = DfsSolver::new(&distmat, &flows);
    let total_flow = solver.dfs(start_id, 30);
    println!("solution 1: {}", total_flow);
    println!("{:?}", solver.marked);
    // println!("{:?}", idmap.rev.get(&1));
    Ok(())
}

fn solve2(file: &File) -> std::io::Result<()> {
    let mut idmap = NodeIdMap::new();
    let (adjmat, flows) = parse_graph(&file, &mut idmap)?;
    let distmat = floyd_warshall(&adjmat).unwrap();
    let start_id = idmap.get("AA");
    
    let mut solver = DfsSolver::new(&distmat, &flows);
    // let total_flow = solver.dfs2(start_id, 26, start_id, 26, false);
    let total_flow_h = solver.dfs(start_id, 26);
    let total_flow_e = solver.dfs(start_id, 26);

    let total_flow = total_flow_e + total_flow_h;
    println!("{}", total_flow);
    // println!("{:?}", solver.marked);
    // println!("{:?}", idmap.rev.get(&1));
    Ok(())
}

fn main() -> std::io::Result<()> {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 2 {
        return Err(IoError::new(ErrorKind::InvalidInput, "Expected filename as argument"));
    }
    let file = File::open(&argv[1])?;

    solve2(&file)?;

    Ok(())
}
