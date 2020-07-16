#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
use std::collections::HashMap;
#[fastout]
fn main() {
    input! {
        n: usize,
        d: [usize; n],
        m: usize,
        t: [usize; m],
    }
    let mut map_d: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        let counter = map_d.entry(d[i]).or_insert(0);
        *counter += 1;
    }
    for i in 0..m {
        let counter = map_d.entry(t[i]).or_insert(0);
        if *counter > 0 {
            *counter -= 1;
        } else {
            println!("NO");
            return;
        }
    }
    println!("YES");
}
