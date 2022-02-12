use proconio::input;

fn main() {
    input! {
        n: usize,
        y: usize,
    }
    for ichiman in (0..(n + 1)).rev() {
        let sum1 = ichiman * 10000;
        if y < sum1 {
            continue;
        }
        for gosen in (0..(n + 1 - ichiman)).rev() {
            let sum2 = sum1 + gosen * 5000;
            if y < sum2 {
                continue;
            }
            for sen in (0..(n + 1 - ichiman - gosen)).rev() {
                let sum3 = sum2 + sen * 1000;
                if y < sum3 {
                    continue;
                }
                if ichiman + gosen + sen == n && sum3 == y {
                    println!("{} {} {}", ichiman, gosen, sen);
                    return;
                }
            }
        }
    }
    println!("-1 -1 -1");
}
