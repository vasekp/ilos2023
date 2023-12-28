use std::collections::BTreeMap;

fn main() {
    let mut counts: [_; 256] = (0..=255)
        .map(|x| std::iter::once((row_score(x), 1usize)).collect::<BTreeMap<_, _>>())
        .collect::<Vec<_>>().try_into().unwrap();
    for _ in 1..8 {
        let counts_new = (0..=255).map(|y| {
            let mut scores: BTreeMap<_, usize> = BTreeMap::new();
            let y_score = row_score(y);
            for x in 0..=255 {
                let xy_score = pair_score(x, y);
                for (&score, &count) in &counts[x as usize] {
                    scores.entry(score + y_score + xy_score)
                        .and_modify(|c| *c += count)
                        .or_insert(count);
                }
            }
            scores
        }).collect::<Vec<_>>().try_into().unwrap();
        counts = counts_new;
    }
    //println!("{counts:?}");
    let count = |target| counts.iter().map(|btree| btree.get(&target).map(|&x| x).unwrap_or(0)).sum::<usize>();
    println!("{}X{}", count(24), count(42));
}

fn row_score(a: u8) -> i32 {
    2 * (((a ^ (a >> 1)) & 0x7F).count_ones() as i32) - 7
}

fn pair_score(a: u8, b: u8) -> i32 {
    2 * ((a ^ b).count_ones() as i32) - 8
}
