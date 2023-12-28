fn main() {
    let lim = 12345678987654321u64;
    let mut total = 0u64;
    for bits in (0..=(lim.ilog2() as u64 + 1)).step_by(3).skip(1) {
        let min = 1u64 << (bits - 1);
        let max = (1u64 << bits) - 1;
        if max <= lim {
            let zeroes = bits / 3;
            total += comb(bits - 1, zeroes);
        } else if lim >= min {
            let ones = bits * 2 / 3;
            total += remainder(bits - 1, ones - 1, lim - min);
            break;
        }
    }
    println!("{total}");
}

fn comb(n: u64, k: u64) -> u64 {
    if k > n {
        return 0;
    }
    let mut res = 1u64;
    for x in 0..k {
        res *= n - x;
        res /= x + 1;
    }
    res
}

fn remainder(bits: u64, ones: u64, max: u64) -> u64 {
    if ones == 0 {
        1
    } else if bits == 0 {
        0
    } else {
        let power = 1u64 << (bits - 1);
        if max & power != 0 {
            //println!("{} +", comb(bits - 1, ones));
            comb(bits - 1, ones) + remainder(bits - 1, ones - 1, max - power)
        } else {
            remainder(bits - 1, ones, max)
        }
    }
}
