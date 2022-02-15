use proconio::input;

fn main() {
    input! {
        n: usize,
        targets: [(i32,i32,i32); n],
    }
    let mut ti = 0;
    let mut xi = 0;
    let mut yi = 0;
    for (t, x, y) in targets {
        let dt = t - ti;
        let dx = x - xi;
        let dy = y - yi;
        if (dt + dx + dy) % 2 == 0 && dx.abs() + dy.abs() <= dt {
            // reachable
            ti = t;
            xi = x;
            yi = y;
        } else {
            // unreachable
            println!("No");
            return;
        }
    }
    println!("Yes");
}
