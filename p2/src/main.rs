use std::io::{self, BufRead};
use std::cell::Cell;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
enum Ori {
    Horiz,
    Vert
}

use Ori::*;

#[derive(Debug)]
struct Semaphore {
    period: u32,
    phase: u32,
    orient: Ori
}

impl Semaphore {
    fn from(string: &str) -> Semaphore {
        let (t, phi) = string.split_once(',').unwrap();
        let period = t.parse::<u32>().unwrap();
        let phase = phi.parse::<u32>().unwrap();
        Semaphore{period, phase, orient: Horiz}
    }

    fn step(&mut self) {
        self.phase += 1;
        if self.phase == self.period {
            self.orient = match self.orient {
                Horiz => Vert,
                Vert => Horiz
            };
            self.phase = 0;
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Dir {
    PlusX,
    MinusX,
    PlusY,
    MinusY
}

use Dir::*;

impl Dir {
    fn from(s: &str) -> Dir {
        match s {
            "+i" => PlusX,
            "-i" => MinusX,
            "+j" => PlusY,
            "-j" => MinusY,
            _ => panic!("dir {s}")
        }
    }

    fn index(self) -> usize {
        match self {
            PlusX => 0,
            MinusX => 1,
            PlusY => 2,
            MinusY => 3
        }
    }

    fn rev(self) -> Dir {
        use Dir::*;
        match self {
            PlusX => MinusX,
            MinusX => PlusX,
            PlusY => MinusY,
            MinusY => PlusY
        }
    }

    fn displace(self, x: usize, y: usize) -> (usize, usize) {
        match self {
            PlusX => (x + 1, y),
            MinusX => (x - 1, y),
            PlusY => (x, y + 1),
            MinusY => (x, y - 1)
        }
    }
}

#[derive(Debug)]
struct Car {
    target_pos: (usize, usize),
    target_dir: Dir,
    moved: Cell<bool>
}

impl Car {
    fn pref_dirs(&self, x: usize, y: usize) -> [Option<Dir>; 3] {
        let pref_x = match x.cmp(&self.target_pos.0) {
            Ordering::Less => Some(PlusX),
            Ordering::Greater => Some(MinusX),
            Ordering::Equal => None };
        let pref_y = match y.cmp(&self.target_pos.1) {
            Ordering::Less => Some(PlusY),
            Ordering::Greater => Some(MinusY),
            Ordering::Equal => None };
        let pref_t = if x == self.target_pos.0 && y == self.target_pos.1 { Some(self.target_dir) }
            else { None };
        [pref_x, pref_y, pref_t]
    }

    fn is_done(&self, x: usize, y: usize, dir: Dir) -> bool {
        x == self.target_pos.0 && y == self.target_pos.1 && dir == self.target_dir
    }
}

#[derive(Debug)]
struct Node {
    smp: Semaphore,
    cars: [Option<Car>; 4]
}

impl Node {
    fn from(smp: Semaphore) -> Node {
        Node { smp, cars: Default::default() }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Use {} input_file", args[0]);
    }
    let filename = &args[1];
    let file = std::fs::File::open(filename).unwrap();

    let mut it = io::BufReader::new(file).lines().map(Result::unwrap);

    let line = it.next().unwrap();
    let dims = line.split(' ').map(|num| num.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let (w, h) = (dims[0], dims[1]);

    let mut field = vec![];
    for _ in 0..h {
        let line = it.next().unwrap();
        let row = line.split(' ')
            .map(Semaphore::from)
            .map(Node::from)
            .collect::<Vec<_>>();
        field.push(row);
    };

    it.next(); // ignore car count
    for line in it {
        let mut iit = line.split(' ');
        let x1 = iit.next().unwrap().parse::<usize>().unwrap();
        let y1 = iit.next().unwrap().parse::<usize>().unwrap();
        let d1 = Dir::from(iit.next().unwrap());
        let x2 = iit.next().unwrap().parse::<usize>().unwrap();
        let y2 = iit.next().unwrap().parse::<usize>().unwrap();
        let d2 = Dir::from(iit.next().unwrap());
        field[y1][x1].cars[d1.rev().index()] = Some(Car{ target_pos: (x2, y2), target_dir: d2, moved: Cell::new(false) })
    }

    for t in 1.. {
        for y in 0..h {
            for x in 0..w {
                for d in 1..=2 {
                    let node = &field[y][x];
                    let dir_in = match (node.smp.orient, d) {
                        (Horiz, 1) => PlusX,
                        (Horiz, 2) => MinusX,
                        (Vert, 1) => PlusY,
                        (Vert, 2) => MinusY,
                        _ => unreachable!()
                    };
                    let Some(ref car) = field[y][x].cars[dir_in.index()] else { continue };
                    if car.moved.get() { continue; }
                    for dir_out in car.pref_dirs(x, y) {
                        let Some(dir_out) = dir_out else { continue };
                        let (x1, y1) = dir_out.displace(x, y);
                        if field[y1][x1].cars[dir_out.index()].is_none() {
                            let Some(car) = field[y][x].cars[dir_in.index()].take() else { unreachable!() };
                            if !car.is_done(x, y, dir_out) {
                                //println!("{x},{y} {dir_in:?} → {x1},{y1} {dir_out:?}");
                                car.moved.replace(true);
                                field[y1][x1].cars[dir_out.index()] = Some(car);
                            } else {
                                //println!("{t}: reached {x} {y} {dir_out:?}");
                            }
                            break;
                        }
                    }
                }
                field[y][x].smp.step();
            }
        }
        //println!("{field:?}");
        let count = field.iter()
            .map(|row| row.iter()
                .map(|node| node.cars.iter()
                    .map(|car| car.is_some() as u32)
                    .sum::<u32>())
                .sum::<u32>())
            .sum::<u32>();
        //println!("{t} {count}");
        if count == 0 {
            println!("{t}");
            return;
        }
        field.iter().for_each(|row|
            row.iter().for_each(|node|
                node.cars.iter().for_each(|opt_car| {
                    if let Some(car) = opt_car {
                        car.moved.replace(false);
                    }
                })));
    }
}
