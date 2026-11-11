use std::{io::{BufRead, BufReader}, path, fmt::{Display, Write}, ops::{Index, RangeBounds, IndexMut, Deref}, collections::{HashMap, HashSet}, str::ParseBoolError, iter};

use anyhow::{Error};

const DEBUG:bool = cfg!(debug_assertions);
#[derive(Debug,PartialEq,Eq,Hash,Copy,Clone)]
struct Node {
    x:usize,
    y:usize,
}
impl Node {
    pub fn new(x:usize, y:usize) -> Self {
        Self {x,y}
    }
}

fn is_valid(grid:&Vec<Vec<i32>>, x:isize, y:isize) -> bool {
    // bounds check
    if x < 0 || y < 0 || x >= grid.len() as isize || y >= grid[0].len() as isize {
        if DEBUG { eprintln!(", out of bounds") };
        return false;
    }
    return true;
}

fn find_lowest<'a>(queue:&HashSet<Node>, dist:&HashMap<Node,i32>) -> Node {
    let mut pt: Node;

    let n = queue.iter().min_by(|&n1,&n2|dist.get(n1).unwrap().cmp(dist.get(n2).unwrap())).unwrap();

    if DEBUG { eprintln!("Nearest {n:?} [{}]", dist.get(n).unwrap()) };
    return n.clone();
}

fn dijkstra(grid:&Vec<Vec<i32>>, sx:usize, sy:usize) -> HashMap<Node,(char,Node)>{

    let mut graph  = HashMap::<Node,i32>::new();
    // convert the grid to a node graph
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            assert!(graph.insert(Node::new(row,col), grid[row][col]).is_none());
        }
    }

    let mut dist:HashMap<Node,i32> = graph.iter().map(|(&k,&v)|(k,i32::MAX)).collect();
    let mut prev:HashMap<Node,(char,Node)> = HashMap::new();

    let mut queue:HashSet<Node> = graph.keys().cloned().collect();

    dist.insert(Node::new(sx,sy), 0);

    while !queue.is_empty() {
        let u = find_lowest(&queue, &dist);
        queue.remove(&u);

        for (dir,dx,dy,_) in DIRS {
            if !is_valid(grid, u.x as isize + dx, u.y as isize + dy) {
                continue;
            }
            let v = Node::new((u.x as isize + dx) as usize,(u.y as isize+dy) as usize);
            // check for 3-in-a-row
            let mut t = &('?',u.clone());
            let it:Vec<_> = iter::from_fn(||{t = prev.get(&t.1)?; Some(t)}).take(3).collect();
            // if DEBUG { eprintln!("HIST {it:?}") };
            if it.len() == 3 && it.iter().all(|(d,_n)|d == dir) {
                if DEBUG { eprintln!("Rejected 3 {dir}, {it:?}") };
                continue;
            }

            // if DEBUG { eprintln!("v: {:?}", v)};
            let alt = dist[&u] + graph[&v];
            if alt < dist[&v] {
                if DEBUG { eprintln!("Best to {v:?} is {} from {u:?} [{}]", dir, alt) };
                dist.insert(v, alt);
                prev.insert(v, (*dir,u));
            }
        }
    }

    return prev;
}

const DIRS:&[(char,isize,isize,char)] = &[('N',-1,0,'S'),('S',1,0,'N'),('E', 0, 1,'W'),('W',0,-1,'E'),];
    

// X is rows, Y is columns, because grid[x][y].
// recursively follow the path to next passed postition
fn compute_follow_path(direction:char, count:u32, grid:&Vec<Vec<i32>>, x:isize, y:isize, last_loss:i32, cost:&mut Vec<Vec<i32>>, pathtracker:&mut Vec<(char,(isize,isize))>) {

    if DEBUG { eprint!("Going {direction} to [{x},{y}]") };

    // bounds check
    if x < 0 || y < 0 || x >= grid.len() as isize || y >= grid[0].len() as isize {
        if DEBUG { eprintln!(", out of bounds") };
        return;
    }

    // gone too far in one direction
    if count > 2 {
        if DEBUG { eprintln!(", rolled too far") };
        return;
    }

    // value the square
    let loss = last_loss + grid[x as usize][y as usize];
    if loss < cost[x as usize][y as usize] {
        cost[x as usize][y as usize] = loss;
    }
    // doesn't work; some paths which will be excluded for rolling too far have an optimal cost lower than the minimum possible.
    //} else {
    //     if DEBUG { eprintln!(", path is worse ({}>={})", loss, cost[x as usize][y as usize]) };
    //     return;
    // }

    // check if we've been to this square before
    if pathtracker.iter().any(|(_,p)| p == &(x,y)) {
        if DEBUG { eprintln!(", path loop") };
        return;
    }

    if x as usize == grid.len() - 1 && y as usize == grid[0].len() - 1 {
        if DEBUG { eprintln!(", at destination, (cost {loss}), path {}", pathtracker.iter().map(|(c,_p)|c).collect::<String>()) };
        return;
    }

    if DEBUG { eprintln!(". {loss}")}

    for d in DIRS.iter() {
        // don't backtrack
        if direction == d.3 {
            continue;
        }
        pathtracker.push((d.0,(x,y)));
        let c = if direction == d.0 { count + 1 } else { 0 };
        compute_follow_path(d.0, c, grid, x + d.1, y + d.2, loss, cost, pathtracker);
        pathtracker.pop();
    }
}

fn load_grid(input:&mut dyn BufRead) -> Result<Vec<Vec<i32>>,Error> {
    let mut lines = input.lines();
    let mut grid = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            return Ok(grid);
        }
        grid.push(line.chars().map(|c|c.to_digit(10).unwrap() as i32).collect::<Vec<_>>());
    }
    anyhow::ensure!(!grid.is_empty(), "No more");
    return Ok(grid);
}

fn go(input:&mut dyn BufRead) -> Result<(),Error>{
    // crucible rolling
    // puzzle input, grid of heat costs to enter square
    let mut grid = load_grid(input).unwrap();

    if DEBUG { eprintln!("grid: {:?}", Grid::from(&grid)) };

    // dynamic programming, calculate the minimum cost to reach each square as we get there

    // get the result grid ready
    // let mut pathtracker = Vec::new();

    // walk the grid, top left, heading right or down
    //compute_follow_path('?', 0, &grid,0, 0, -(grid[0][0] as i32), &mut cost, &mut pathtracker);
    let pathtracker = dijkstra(&grid, 0, 0);
    let mut t = &('?',Node::new(grid.len()-1,grid[0].len()-1));
    let mut path:Vec<_> = iter::from_fn(||{t = pathtracker.get(&t.1)?; Some(t)}).collect();
    path.reverse();

    let costs = path.iter().skip(1).map(|(d,n)|grid[n.x][n.y]).collect::<Vec<_>>();

    if DEBUG { eprintln!("costs: {:X?}", costs)};
    if DEBUG { eprintln!("path: {:X?}", path) };

    // output the minimum heat lost
    println!("{}", costs.iter().sum::<i32>());

    // PART TWO
    // eprintln!("PART TWO");

    return Ok(());
}

fn main() -> Result<(),Error> {
    go(&mut std::io::stdin().lock())
}

#[derive(PartialEq)]
struct Grid<'a,T:PartialEq> (&'a Vec<Vec<T>>);

impl<'a, T:PartialEq> From<&'a Vec<Vec<T>>> for Grid<'a,T> {
    fn from(g: &'a Vec<Vec<T>>) -> Self {
        Self(g)
    }
}

impl<'a,T:std::fmt::Debug + std::cmp::PartialEq> std::fmt::Debug for Grid<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('\n')?;
        for v in self.0.iter() {
            f.write_char('[')?;
            for c in v.iter() {
                c.fmt(f)?;
                f.write_char(',');
            }
            f.write_str("],\n")?;
        }
        Ok(())
    }
}


#[test]
fn example() -> Result<(),Error> {
    let testinput = 
r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    go(&mut testinput.as_bytes())
}

#[test]
fn testinput1() -> Result<(),Error> {
    let testinput = 
r"241
321
325";

    go(&mut testinput.as_bytes())
}