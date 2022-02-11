use std::{cmp::Ordering, io::Write};

fn main() {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let mut split = buf.trim().split_whitespace();
    let n: usize = split.next().unwrap().parse().unwrap();
    let _q: usize = split.next().unwrap().parse().unwrap();

    // prepare character vec
    let mut chars = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    chars.truncate(n);

    chars.sort_unstable_by(|left, right| {
        println!("? {} {}", &left, &right);
        stdout.flush().ok().unwrap();
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        return match buf.trim() {
            "<" => Ordering::Less,
            ">" => Ordering::Greater,
            _ => panic!("unknown response received!"),
        };
    });

    print!("! ");
    for c in chars {
        print!("{}", c);
    }
    print!("\n");
    stdout.flush().ok().unwrap();
}
