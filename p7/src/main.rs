fn main() {
    let mut last = None;
    let mut field = [[0i8; 257]; 257];

    for line in std::fs::read_to_string("park.txt").unwrap().lines() {
        let pos: [i32; 2] = line.split(' ').map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>().try_into().unwrap();
        let pos: (i32, i32) = pos.try_into().unwrap();
        if let Some(last) = last {
            let tang = diff(pos, last);
            let norm = (tang.1, -tang.0);
            let n2 = len2(tang);
            for y in 0..=256 {
                for x in 0..=256 {
                    let vec = diff((x, y), last);
                    let dp = dot(vec, tang);
                    let np = dot(vec, norm);
                    if dp >= 0 && dp <= n2 && np * np <= 25 * len2(norm) {
                        field[y as usize][x as usize] = 1;
                    }
                }
            }
        }
        for y in 0..=256 {
            for x in 0..=256 {
                if dist2(pos, (x, y)) <= 25 {
                    field[y as usize][x as usize] = 1;
                }
            }
        }
        last = Some(pos);
    }

    let count = field.into_iter().map(|row|
        row.into_iter().map(|x| (1 - x) as i32).sum::<i32>())
        .sum::<i32>();
    println!("{count}");
}

fn dot(a: (i32, i32), b: (i32, i32)) -> i32 {
    a.0 * b.0 + a.1 * b.1
}

fn len2(a: (i32, i32)) -> i32 {
    dot(a, a)
}

fn diff(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (b.0 - a.0, b.1 - a.1)
}

fn dist2(a: (i32, i32), b: (i32, i32)) -> i32 {
    len2(diff(a, b))
}
