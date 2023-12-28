fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Use {} input_file", args[0]);
    }
    let filename = &args[1];

    let photo = std::fs::read_to_string(filename).unwrap()
        .lines().map(|l| l.as_bytes().to_owned()).collect::<Vec<_>>();
    let photo_h = photo.len();
    let photo_w = photo[0].len();

    let pattern = std::fs::read_to_string("pattern.txt").unwrap()
        .lines().map(|l| l.as_bytes().to_owned()).collect::<Vec<_>>();
    let pat_h = pattern.len();
    let pat_w = pattern[0].len();

    let mut count = 0u32;
    for y in 0..=(photo_h - pat_h) {
        for x in 0..=(photo_w - pat_w) {
            let olap = overlap(
                photo.iter().skip(y).map(|line| line.iter().skip(x)),
                pattern.iter().map(|line| line.iter()));
            if olap >= 30 {
                count += 1;
            }
        }
    }
    println!("{count}");
}

fn overlap<'a>(a: impl Iterator<Item = impl Iterator<Item = &'a u8>>,
    b: impl Iterator<Item = impl Iterator<Item = &'a u8>>) -> u32 {
    a.zip(b).map(|(u, v)|
        u.zip(v).map(|(x, y)| (*x == *y) as u32)
        .sum::<u32>()).sum::<u32>()
}
