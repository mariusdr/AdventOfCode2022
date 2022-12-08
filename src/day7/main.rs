use std::borrow::{Borrow, BorrowMut};
use std::{env, usize};
use std::fs::File;
use std::io::prelude::*;
use std::cell::{RefCell, Cell};
use std::rc::Rc;

#[derive(Debug)]
struct FileNode {
    parent: Option<Rc<RefCell<FileNode>>>,
    children: Option<Vec<Rc<RefCell<FileNode>>>>,
    name: String,
    size: usize,
    is_dir: bool,
    is_root: bool
}

impl FileNode {
    fn new_dir(parent: &Rc<RefCell<FileNode>>, name: &str) -> Self {
        let s = Self { 
            parent: Some(parent.clone()), 
            children: Some(Vec::new()), 
            name: String::from(name), 
            size: 0, 
            is_dir: true, 
            is_root: false 
        };
        s
    }
    
    fn new_file(parent: &Rc<RefCell<FileNode>>, name: &str, size: usize) -> Self {
        let mut s = Self { 
            parent: Some(parent.clone()), 
            children: None, 
            name: String::from(name), 
            size: size, 
            is_dir: false, 
            is_root: false 
        };
        s.update_size(size);
        s
    }

    fn new_root() -> Self {
        Self { 
            parent: None, 
            name: String::from("/"), 
            children: Some(Vec::new()),
            size: 0, 
            is_dir: true, 
            is_root: true
        }
    }

    fn get_parent(&self) -> &RefCell<FileNode> {
        self.parent.as_ref().unwrap().borrow()
    }

    fn get_children(&self) -> Option<&Vec<Rc<RefCell<FileNode>>>> {
        self.borrow().children.as_ref()
    }

    fn get_children_mut(&mut self) -> Option<&mut Vec<Rc<RefCell<FileNode>>>> {
        self.borrow_mut().children.as_mut()
    }

    fn abs_pathname(&self) -> String {
        if self.is_root {
            return String::from("/");
        }
        let pn = self.get_parent().borrow().abs_pathname();
        if self.is_dir {
            pn + &self.name + "/"
        } else {
            pn + &self.name
        }
    }
    
    fn get_size(&self) -> usize {
        self.size
    }

    fn update_size(&mut self, size: usize) {
        if !self.is_root {
            let prefc = self.get_parent();
            prefc.borrow_mut().size += size;
            prefc.borrow_mut().update_size(size);
        }
    }
}

fn mk_root() -> Rc<RefCell<FileNode>> {
    Rc::new(RefCell::new(FileNode::new_root()))
}

fn mk_dir(parent: &mut Rc<RefCell<FileNode>>, name: &str) -> Rc<RefCell<FileNode>> {
    let node = Rc::new(RefCell::new(FileNode::new_dir(parent, name)));
    let prefc: &RefCell<FileNode> = parent.borrow_mut();
    let mut pnn = prefc.borrow_mut();
    let cs = pnn.children.as_mut().unwrap();
    cs.push(node.clone());
    node
}

fn mk_file(parent: &mut Rc<RefCell<FileNode>>, name: &str, size: usize) -> Rc<RefCell<FileNode>> {
    let node = Rc::new(RefCell::new(FileNode::new_file(parent, name, size)));
    let prefc: &RefCell<FileNode> = parent.borrow_mut();
    let mut pnn = prefc.borrow_mut();
    let cs = pnn.children.as_mut().unwrap();
    cs.push(node.clone());
    node
}

fn get_abs_pathname(node: &Rc<RefCell<FileNode>>) -> String {
    node.as_ref().borrow().borrow().abs_pathname()
}

fn get_size(node: &Rc<RefCell<FileNode>>) -> usize {
    node.as_ref().borrow().borrow().size
}

struct FileSystem {
    root: Rc<RefCell<FileNode>>,
    cwd: Rc<RefCell<FileNode>>
}

impl FileSystem {
    fn new(root: Rc<RefCell<FileNode>>) -> Self {
        Self { root: root.clone(), cwd: root }
    }

    fn chdir(&mut self, dirname: &str) -> Option<()> {
        if dirname == ".." {
            if !self.cwd.clone().as_ref().borrow().is_root {
                let c = self.cwd.clone().as_ref().borrow().parent.clone().unwrap();
                self.cwd = c;
            }
        } else if dirname == "/" {
            self.cwd = self.root.clone();
        } else {
            for cnode in self.cwd.clone().as_ref().borrow().get_children().unwrap() {
                if dirname == cnode.as_ref().borrow().name {
                    self.cwd = cnode.clone();
                    return Some(());
                }
            }
            return None;
        }
        Some(())
    }
}

struct InputReader<'a> {
    file: &'a File,
    fs: &'a mut FileSystem,
}

impl<'a> InputReader<'a> {
    fn new(file: &'a File, fs: &'a mut FileSystem) -> Self {
        Self { file: file, fs: fs }
    }

    fn parse(&mut self) -> std::io::Result<()> {
        let reader = std::io::BufReader::new(self.file);
        let mut lineit = reader.lines().peekable();
        while let Some(Ok(line)) = lineit.next() {
            if line.starts_with("$ cd") {
                if let Some(dirname) = line.split(' ').nth(2) {
                    self.fs.chdir(dirname).unwrap();
                }
            } else if line.starts_with("$ ls") {
                while let Some(Ok(listline)) = lineit.next() {
                    if listline.starts_with("dir ") {
                        if let Some(dirname) = listline.split(' ').nth(1) {
                            mk_dir(&mut self.fs.cwd.clone(), dirname);
                        }
                    } else {
                        let mut parts = listline.split(' ');
                        if let Some(fsize) = parts.nth(0) {
                            if let Some(fname) = parts.nth(0) {
                                let sz = fsize.parse::<usize>().unwrap();
                                mk_file(&mut self.fs.cwd.clone(), fname, sz);
                            }
                        }
                    }
                    if let Some(Ok(p)) = lineit.peek() {
                        if p.starts_with("$ ") {
                            break;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

fn traverse(root: &Rc<RefCell<FileNode>>) {
    if let Some(cs) = &root.as_ref().borrow().children {
        println!("# childrens of {} is {}", root.clone().as_ref().borrow().name, cs.len());
        for cnode in cs {
            traverse(&cnode);
        }
    }
    
    println!("{} | {} | dir? {}", 
        get_abs_pathname(root), 
        get_size(root), 
        root.clone().as_ref().borrow().is_dir);
}

fn solve1(root: &Rc<RefCell<FileNode>>) -> usize {
    if root.clone().as_ref().borrow().is_dir == false {
        return 0;
    }

    let mut coll = 0;
    if let Some(cs) = &root.as_ref().borrow().children {
        for cnode in cs {
            if !cnode.as_ref().borrow().is_dir {
                continue;
            }
            let s = get_size(cnode);
            if s < 100000 {
                println!("{} as {}", get_abs_pathname(cnode), s);
                coll += s;
            }
            coll += solve1(cnode);
        }
    }
    coll
}

fn print_childrens(root: &Rc<RefCell<FileNode>>) {
    println!("{} | size {}", get_abs_pathname(root), get_size(root));
    if let Some(cs) = &root.as_ref().borrow().children {
        for cnode in cs {
            println!("--   {} | size {}", get_abs_pathname(cnode), get_size(cnode));
        }
    }
}

fn collect(root: &Rc<RefCell<FileNode>>, candidates: &mut Vec<Rc<RefCell<FileNode>>>, required: usize) {
    if !root.as_ref().borrow().is_dir {
        return;
    } 
    
    if let Some(cs) = &root.as_ref().borrow().children {
        for cnode in cs {
            collect(cnode, candidates, required);
        }
    }    
    if get_size(root) >= required {
        println!("add {} with size {} to candidates", get_abs_pathname(root), get_size(root));
        candidates.push(root.clone());
    } 
}

fn solve2(root: &Rc<RefCell<FileNode>>) {
    let mut frontier: Vec<Rc<RefCell<FileNode>>> = Vec::new();
    let used = get_size(&root);
    let unused = 70000000 - used;
    let minimum = 30000000;
    let required = minimum - unused;
    println!("unused   {}", unused);
    println!("required {}", required);
    
    collect(root, &mut frontier, required);

    let mut curr: usize = 9999999999;
    for node in frontier {
        curr = std::cmp::min(curr, get_size(&node));
    }
    println!("best candidate has size {}", curr);
}

fn main() -> std::io::Result<()> {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 2 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Expected filename as argument"));
    }
    let file = File::open(&argv[1])?;
    let mut fs = FileSystem::new(mk_root());

    InputReader::new(&file, &mut fs).parse()?;
    // println!("-----------------");
    // let s = solve1(&fs.root);
    // println!("size {}", s);

    solve2(&fs.root);

    Ok(())
}
