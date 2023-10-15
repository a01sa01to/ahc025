use proconio::{input, source::line::LineSource};
use std::io::{stdin, BufReader, StdinLock};
extern crate rand;
use rand::Rng;
use std::collections::HashSet;

fn output_answer(ans: &Vec<usize>, is_debug: bool) {
    if is_debug {
        print!("#c ");
    }
    for i in 0..ans.len() {
        print!("{} ", ans[i]);
    }
    println!();
}

fn query(
    l: &Vec<usize>,
    r: &Vec<usize>,
    source: &mut LineSource<BufReader<StdinLock<'_>>>,
) -> char {
    print!("{} {}", l.len(), r.len());
    for i in 0..l.len() {
        print!(" {}", l[i]);
    }
    for i in 0..r.len() {
        print!(" {}", r[i]);
    }
    println!();

    input! {
        from source,
        res: char,
    };
    res
}

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

        output_answer(&ans, true);
        let res = query(&l, &r, &mut source);
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
    output_answer(&ans, false);
}
