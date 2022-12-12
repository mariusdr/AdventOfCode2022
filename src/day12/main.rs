use std::cmp::min;
use std::{env, vec};
use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap};
use std::thread;
use std::sync::{Arc, Mutex};

use std::io::ErrorKind;
type IoError = std::io::Error;

type NodeId = usize;

struct Graph {
    adjlist: HashMap<NodeId, Vec<NodeId>>,
    node_weights: HashMap<NodeId, i32>,
    nnodes: usize,
}

impl Graph {
    fn new() -> Graph {
        Self { adjlist: HashMap::new(), node_weights: HashMap::new(), nnodes: 0 }
    }

    fn push_edge(&mut self, from: NodeId, to: NodeId) {
        let handle = self.adjlist.entry(from).or_insert(Vec::new());
        handle.push(to);
        self.calc_node_cnt();
    }

    fn set_node_weight(&mut self, node: NodeId, weight: i32) {
        self.node_weights.entry(node).and_modify(|w| *w = weight).or_insert(0);
    }

    fn node_cnt(&self) -> usize {
        self.nnodes
    }

    fn calc_node_cnt(&mut self) -> usize {
        let mut max_id = 0;
        self.adjlist.iter().for_each(|(_, list)| {
            let m = list.iter().map(|node| *node).max().unwrap();
            max_id = std::cmp::max(max_id, m);
        });
        self.nnodes = max_id + 1;
        self.nnodes 
    }

    fn neighbors(&self, node: NodeId) -> Option<&Vec<NodeId>> {
        self.adjlist.get(&node)
    }

    fn dijsktra(&self, start: NodeId, target: NodeId) -> i32 {
        let mut dist = vec![i32::MAX; self.node_cnt()];
        let mut done = vec![true; self.node_cnt()];
        for x in self.adjlist.keys() {
            done[*x] = false;
        }

        dist[start] = 0;
        
        let next_node = |dist: &[i32], done: &[bool]| -> Option<NodeId> {
            let res = done.iter().enumerate().filter(|(_, flag)| {
                **flag == false
            }).min_by(|&(x, _), &(y, _)| -> std::cmp::Ordering {
                if dist[x] < dist[y] {
                    return std::cmp::Ordering::Less;
                } else if dist[x] > dist[y] {
                    return std::cmp::Ordering::Greater;
                }
                std::cmp::Ordering::Equal
            });
            if let Some((x, _)) = res {
                return Some(x);
            }
            None
        };

        while let Some(x) = next_node(&dist, &done) {
            done[x] = true;
            if x == target {
                // found target, early stop possible
                break;
            }
            if dist[x] == i32::MAX {
                // isolated node, can be ignored
                continue;
            }
            for y in self.neighbors(x).unwrap_or(&Vec::new()) {
                if dist[x] + 1 < dist[*y] {
                    dist[*y] = dist[x] + 1;
                }
            }
        }
        dist[target]
    }

    fn is_connected(&self, from: NodeId, to: NodeId) -> bool {
        let mut frontier = vec![from];
        let mut done = vec![false; self.node_cnt()];

        while frontier.len() > 0 {
            let mut new_frontier: Vec<NodeId> = Vec::with_capacity(frontier.len());
            for x in &frontier {
                if done[*x] {
                    continue;
                }
                done[*x] = true;
                
                for y in self.neighbors(*x).unwrap_or(&Vec::new()) {
                    if done[*y] == true {
                        continue;
                    }
                    if *y == to {
                        return true;
                    }
                    new_frontier.push(*y);
                }
            }
            frontier.clear();
            frontier = new_frontier;
        }
        false
    }

    fn bruteforce_assp(&self, source: NodeId, target: NodeId) -> i32 {
        let initial_weight = *self.node_weights.get(&source).expect("each node with outgoing edges should have a weight");
        let mut best_dist = i32::MAX;
        for (s, sw) in self.node_weights.iter() {
            if *sw == initial_weight {
                if !self.is_connected(*s, target) {
                    continue;
                }
                let ds = self.dijsktra(source, target);
                best_dist = std::cmp::min(best_dist, ds);
                println!("best distance after node {} is {}", s, best_dist);
            }
        } 
        best_dist
    }

    fn print(&self) {
        println!("graph: node_cnt: {}", self.node_cnt());
        self.adjlist.iter().for_each(|(node, list)| {
            let w = self.node_weights.get(node).unwrap_or(&0);
            println!("{} / w {} -> {:?}", node, w, list);
        });
    }
}

struct GraphReader {
    buf: Vec<u8>,
    xlen: usize,
    ylen: usize,
    source: NodeId, 
    target: NodeId,
}

impl GraphReader {
    fn new(file: &File) -> std::io::Result<Self> {
        let mut buf: Vec<u8> = Vec::new();    
        let mut reader = std::io::BufReader::new(file);
        reader.read_to_end(&mut buf)?;
        buf.push('\n' as u8);
        let ylen = buf.iter().take_while(|x| **x != '\n' as u8).count();
        let xlen = buf.iter().filter(|x| **x == '\n' as u8).count();
        buf = buf.iter().filter(|x| **x != '\n' as u8).map(|&x| x).collect();
        Ok(Self { buf, xlen, ylen, source: 0, target: 0 })
    }

    fn as_nid(&self, x: usize, y: usize) -> NodeId {
        x * self.ylen + y
    }

    fn upper(&self, x: usize, y: usize) -> Option<NodeId> {
        if x > 0 {
            return Some(self.as_nid(x - 1, y));
        }
        None
    }

    fn lower(&self, x: usize, y: usize) -> Option<NodeId> {
        if x < self.xlen - 1 {
            return Some(self.as_nid(x + 1, y));
        }
        None
    }

    fn left(&self, x: usize, y: usize) -> Option<NodeId> {
        if y > 0 {
            return Some(self.as_nid(x, y - 1));
        }
        None
    }

    fn right(&self, x: usize, y: usize) -> Option<NodeId> {
        if y < self.ylen - 1 {
            return Some(self.as_nid(x, y + 1));
        }
        None
    }

    fn has_edge(&self, from: NodeId, to: NodeId) -> bool {
        let mut cfrom = self.buf[from];
        let mut cto = self.buf[to];
        
        if cfrom == 'S' as u8 {
            cfrom = 'a' as u8;
        }
        if cto == 'E' as u8 {
            cto = 'z' as u8;
        }
        cfrom > cto || cfrom == cto || cfrom + 1 == cto
    }

    fn node_weight(&self, node: NodeId) -> i32 {
        let cn = self.buf[node];
        if cn == 'S' as u8 {
            return 'a' as i32;
        } 
        if cn == 'E' as u8 {
            return 'z' as i32;
        }
        cn as i32
    }

    fn add_edge(&self, graph: &mut Graph, from: NodeId, to: NodeId) {
        if self.has_edge(from, to) {
            graph.push_edge(from, to);
            graph.set_node_weight(from, self.node_weight(from));
        }
    }

    fn read(&mut self) -> (Graph, NodeId, NodeId) {
        let mut graph = Graph::new();
        for x in 0..self.xlen {
            for y in 0..self.ylen {
                let from = self.as_nid(x, y);
                if self.buf[from] == 'S' as u8 {
                    self.source = from;
                } else if self.buf[from] == 'E' as u8 {
                    self.target = from;
                }

                if let Some(to) = self.upper(x, y) {
                    self.add_edge(&mut graph, from, to);
                }
                if let Some(to) = self.lower(x, y) {
                    self.add_edge(&mut graph, from, to);
                }
                if let Some(to) = self.left(x, y) {
                    self.add_edge(&mut graph, from, to);
                }
                if let Some(to) = self.right(x, y) {
                    self.add_edge(&mut graph, from, to);
                }
            }
        }
        (graph, self.source, self.target)
    }
}


fn multithread_assp<const NTHREADS: usize>(graph: Arc<Graph>, source: NodeId, target: NodeId) -> std::thread::Result<i32> {
    let initial_weight = *graph.node_weights.get(&source).expect("each node with outgoing edges should have a weight");
    let start_set: Vec<NodeId> = graph.node_weights.iter()
                                      .filter(|(_, xw)| **xw == initial_weight)
                                      .map(|(x, _)| *x)
                                      .collect();
    
    let work_set = Arc::new(Mutex::new(start_set));
    let results: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(Vec::new()));
    
    let mut handles: Vec<std::thread::JoinHandle<()>> = Vec::new();
    for _ in 0..NTHREADS {
        let graph = graph.clone();
        let work_set = work_set.clone();
        let results = results.clone();
        let jh = thread::spawn(move || {
            loop {
                let node: NodeId;
                let mut work_set = work_set.lock().unwrap();            
                if let Some(n) = work_set.pop() {
                    node = n;
                } else {
                    break;
                }
                drop(work_set); // release mutex
                
                if graph.is_connected(node, target) {
                    let dist = graph.dijsktra(node, target);
                    println!("found dist {} for start node {}", dist, node);
                    let mut results = results.lock().unwrap();
                    results.push(dist);
                }
            }
        });
        handles.push(jh);
    }

    for jh in handles {
        jh.join()?;
    }

    let rs = results.lock().unwrap();
    Ok(*rs.iter().min().unwrap())
}

fn main() -> std::io::Result<()> {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 2 {
        return Err(IoError::new(ErrorKind::InvalidInput, "Expected filename as argument"));
    }
    let file = File::open(&argv[1])?;
    let (graph, source, target) = GraphReader::new(&file)?.read();

    // g.print();
    println!("read graph start algo..");

    // println!("{} and {} are connected? {}", g.source, g.drain, g.is_connected(g.source, g.drain));

    // let d1 = graph.dijsktra(source, target);
    // println!("solution 1: dist {} to {} is {}", source, target, d1);

    // let d2 = graph.bruteforce_assp(source, target);
    // println!("solution 2: shortest overall distance is {}", d2);
    
    let graphptr = Arc::new(graph);
    if let Ok(d2) = multithread_assp::<64>(graphptr, source, target) {
        println!("solution 2: shortes overall distance is {}", d2);
    } else {
        eprintln!("thread error");
    }
    Ok(())
}