use proconio::{input, marker::Bytes};

fn main() {
    let candidates: Vec<&[u8]> = vec!["dream", "dreamer", "erase", "eraser"]
        .iter()
        .map(|s| s.as_bytes())
        .collect();
    input! {
        target: Bytes,
    }
    if judge(&target, 0, &candidates) {
        println!("YES");
    } else {
        print!("NO");
    }
}

fn judge(target: &Vec<u8>, index: usize, candidates: &Vec<&[u8]>) -> bool {
    if target.len() == index {
        return true;
    }
    for candidate in candidates {
        let (_, right) = target.split_at(index);
        if right.starts_with(candidate) {
            if judge(target, index + candidate.len(), candidates) {
                return true;
            }
        }
    }
    return false;
}
