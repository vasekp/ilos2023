use std::fs::File;
use std::io::{self, BufRead};

struct Plotter {
    mat: Vec<Vec<u8>>,
    x: usize,
    y: usize
}

impl Plotter {
    fn new() -> Plotter {
        Plotter{ mat: Default::default(), x: 0, y: 0}
    }

    fn process(&mut self, ch: u8, dir: u8) {
        let (x, y) = (self.x, self.y);
        if self.mat.len() <= y {
            self.mat.resize(y + 1, Default::default());
        }
        if self.mat[y].len() <= x {
            self.mat[y].resize(x + 1, b' ');
        }
        self.mat[y][x] = ch;
        match dir {
            b'R' => self.x += 1,
            b'L' => self.x -= 1,
            b'U' => self.y -= 1,
            b'D' => self.y += 1,
            _ => panic!("dir {}", dir as char)
        }
    }

    fn writeout(&self) {
        for row in &self.mat {
            println!("{}", std::str::from_utf8(row).unwrap());
        }
    }

    fn lines(&self) -> impl Iterator<Item = &Vec<u8>> {
        self.mat.iter()
    }
}

fn main() {
    if std::env::args().count() > 1 {
        println!("No argument expected, reading 'losi-plotr.txt'.");
    }

    let file = File::open("losi-plotr.txt").unwrap();
    let mut p1 = Plotter::new();
    for line in io::BufReader::new(file).lines().map(Result::unwrap) {
        let line = line.as_bytes();
        let (ch, ins) = (line[0], line[1]);
        p1.process(ch, ins);
    }
    //p1.writeout();

    let mut p2 = Plotter::new();
    for line in p1.lines().step_by(6) {
        let ch = line[3];
        let rest = &line[9..];
        let ins = match rest.iter().filter(|x| **x == b'#').count() {
            1 => b'L',
            2 => b'U',
            4 => b'R',
            5 => b'D',
            _ => panic!("{}", std::str::from_utf8(rest).unwrap())
        };
        p2.process(ch, ins);
    }
    p2.writeout();
}
