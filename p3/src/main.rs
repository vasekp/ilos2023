fn main() {
    let mut shape = std::fs::read_to_string("logo.csv").unwrap()
        .lines().map(|line| line.as_bytes().iter().filter(|&b| *b != b';')
            .map(|&b| match b { b'X' => true, b'_' => false, _ => panic!() })
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Each u8 in shape represents 20×20 pixels.
    // We reduce by 43 pixels, which is 2× 20-pixel blocks plus 3 more pixels.
    let (w, h) = (shape[0].len(), shape.len());
    /*for y in 0..h {
        for x in 0..w {
            if shape[y][x] {
                println!("M {} {} h 63 v 63 h -63 z", x as i64 * 20 - 43, y as i64 * 20 - 43);
            }
        }
    }*/

    for y in 0..(h - 2) {
        for x in 0..w {
            shape[y][x] |= shape[y + 1][x] | shape[y + 2][x];
        }
    }
    for y in 0..h {
        for x in 0..(w - 2) {
            shape[y][x] |= shape[y][x + 1] | shape[y][x + 2];
        }
    }

    // Here we inflate by a factor of 20
    let mut shape = shape.into_iter()
        .flat_map(|row| std::iter::repeat_with(move ||
                row.iter().flat_map(|b| [*b].repeat(20)).collect::<Vec<_>>())
            .take(20))
        .collect::<Vec<_>>();

    let (w, h) = (shape[0].len(), shape.len());
    for y in 0..(h - 3) {
        for x in 0..w {
            shape[y][x] |= shape[y + 1][x] | shape[y + 2][x] | shape[y + 3][x];
        }
    }
    for y in 0..h {
        for x in 0..(w - 3) {
            shape[y][x] |= shape[y][x + 1] | shape[y][x + 2] | shape[y][x + 3];
        }
    }

    let (mut x, mut y) = (365, 342);
    let (mut xr, mut yr) = (0, 0);
    let (mut vx, mut vy) = (-8i32, -5i32);

    let mut count = 0;
    loop {
        let xp = x - (xr == 0 && vx < 0) as usize;
        let yp = y - (yr == 0 && vy < 0) as usize;
        let xm = x - (xr == 0 && vx > 0) as usize;
        let ym = y - (yr == 0 && vy > 0) as usize;
        let refl = match (xr, yr) {
            (0, 0) => match (shape[yp][xp], shape[ym][xp], shape[yp][xm]) {
                (false, _, _) => (false, false),
                (true, x, y) => (x, y)
            },
            (0, _) => (shape[ym][xp], false),
            (_, 0) => (false, shape[yp][xm]),
            (_, _) => (false, false)
        };
        if refl == (false, false) {
            if xr == 0 && vx < 0 { x -= 1; }
            if yr == 0 && vy < 0 { y -= 1; }
            loop {
                xr = (xr + vx).rem_euclid(40);
                yr = (yr + vy).rem_euclid(40);
                if xr == 0 || yr == 0 {
                    break;
                }
            }
            if xr == 0 && vx > 0 { x += 1; }
            if yr == 0 && vy > 0 { y += 1; }
        } else {
            //println!("L {} {}", x as f64 + xr as f64 / 40.0, y as f64 + yr as f64 / 40.0);
            count += 1;
            match refl {
                (true, false) => vx = -vx,
                (false, true) => vy = -vy,
                (true, true) => break,
                (false, false) => unreachable!()
            }
        }
    }
    println!("{}", count - 1);
}
