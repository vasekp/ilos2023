fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Use {} input_file", args[0]);
    }
    let filename = &args[1];
    let field = std::fs::read_to_string(filename).unwrap()
        .lines().map(|line|
            line.split(", ").map(|num| num.parse::<u8>().unwrap()).collect::<Vec<_>>()
        ).collect::<Vec<_>>();
    //println!("{field:?}");

    let mut path = vec![];
    let w = field[0].len();
    for x in 0..w {
        if walk(&field, &mut path, 0, x) {
            break;
        }
    }
    //println!("{path:?}");

    let mut it = path.into_iter();
    let mut string = format!("{}", it.next().unwrap());
    for x in it {
        string += &format!(",{}", x);
    }
    println!("{string}");
}

fn walk(field: &Vec<Vec<u8>>, path: &mut Vec<u8>, y: usize, x: usize) -> bool {
    if path.contains(&field[y][x]) {
        return false;
    }
    path.push(field[y][x]);
    if y == field.len() - 1 {
        return true;
    }
    if (x > 0 && walk(field, path, y + 1, x - 1))
        || walk(field, path, y + 1, x)
        || (x + 1 < field[0].len() && walk(field, path, y + 1, x + 1)) {
            return true;
    }
    path.pop();
    false
}
