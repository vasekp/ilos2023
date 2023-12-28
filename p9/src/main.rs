use std::fs::File;
use std::io::{self, BufRead};
use std::collections::{HashMap, VecDeque};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Use {} input_file", args[0]);
    }
    let filename = &args[1];

    let file = File::open(filename).unwrap();
    let mut lines = io::BufReader::new(file).lines().map(Result::unwrap);
    lines.next();
    let dims = lines.next().unwrap();
    let mut it = dims.split(' ').map(|x| x.parse::<usize>().unwrap());
    let (y1, x1, y2, x2) = (it.next().unwrap(), it.next().unwrap(), it.next().unwrap(), it.next().unwrap());
    let tol = lines.next().unwrap().parse::<u64>().unwrap();
    lines.next();
    let field = lines.map(|line|
        line.split(' ').map(|val| val.parse::<u64>().unwrap())
            .collect::<Vec<_>>()).collect::<Vec<_>>();
    let w = field[0].len();
    let h = field.len();

    let mut map = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(((x1, y1), (0, 0)));

    while let Some(((x, y), (steps, total))) = queue.pop_front() {
        //println!("{}", queue.len());
        let vec = map.entry((x, y)).or_insert(vec![]);
        let steps = steps + 1;
        let total = total + field[y][x];
        if vec.iter().any(|(s, t)| *s <= steps && *t <= total) {
            continue;
        }
        vec.retain(|(s, t)| *s < steps || *t < total);
        vec.push((steps, total));
        //println!("{vec:?}");

        if x > 0 { queue.push_back(((x - 1, y), (steps, total))); }
        if x < w - 1 { queue.push_back(((x + 1, y), (steps, total))); }
        if y > 0 { queue.push_back(((x, y - 1), (steps, total))); }
        if y < h - 1 { queue.push_back(((x, y + 1), (steps, total))); }
    }

    let mut vec = map.remove(&(x2, y2)).unwrap();
    //println!("{vec:?}");
    vec.retain(|(_, t)| t <= &tol);
    let len = vec.into_iter().min_by(|x, y| x.0.cmp(&y.0)).unwrap().0 - 1; // start counted in
    println!("{len}");
}
