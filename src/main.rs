use proconio::{input, source::line::LineSource};
use std::io::{stdin, BufReader};

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        n: usize,
        d: usize,
        q: usize,
    };

    let mut ans = vec![0; n];
    for i in 0..n {
        ans[i] = i % d;
    }

    for _ in 0..q {
        println!("{} {} {} {}", 1, 1, 0, 1);
        input! {
            from &mut source,
            _res: char,
        };
    }

    for i in 0..n {
        print!("{} ", ans[i]);
    }
    println!();
}
