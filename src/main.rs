use proconio::{input, source::line::LineSource};
use std::io::{stdin, BufReader};
extern crate rand;
use rand::Rng;
use std::collections::HashSet;

fn main() {
    let mut rng = rand::thread_rng();
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
    let mut non_changable = HashSet::<usize>::new();

    let mut cnt = 0;
    while cnt < q {
        if non_changable.len() == n {
            println!("1 1 0 1");
            continue;
        }

        let i1 = rng.gen_range(0..n);
        let i2 = rng.gen_range(0..n);
        if ans[i1] == ans[i2] {
            continue;
        }
        if non_changable.contains(&i1) || non_changable.contains(&i2) {
            continue;
        }
        let mut l = Vec::<usize>::new();
        let mut r = Vec::<usize>::new();
        for i in 0..n {
            if ans[i] == ans[i1] {
                l.push(i);
            }
            if ans[i] == ans[i2] {
                r.push(i);
            }
        }

        print!("{} {}", l.len(), r.len());
        for i in 0..l.len() {
            print!(" {}", l[i]);
        }
        for i in 0..r.len() {
            print!(" {}", r[i]);
        }
        println!();

        input! {
            from &mut source,
            res: char,
        };
        cnt += 1;

        if res == '<' {
            ans[i2] = ans[i1];
        }
        if res == '>' {
            ans[i1] = ans[i2];
        }
        if res == '=' {
            non_changable.insert(i1);
            non_changable.insert(i2);
        }
    }

    for i in 0..n {
        print!("{} ", ans[i]);
    }
    println!();
}
