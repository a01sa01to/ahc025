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

static mut CNT: usize = 0;
fn query(
    l: &Vec<usize>,
    r: &Vec<usize>,
    q: usize,
    source: &mut LineSource<BufReader<StdinLock<'_>>>,
) -> (char, bool) {
    let mut rng = rand::thread_rng();

    if unsafe { CNT } >= q {
        let i = rng.gen_range(0..2);
        if i == 0 {
            return ('<', true);
        } else {
            return ('>', true);
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
        from source,
        res: char,
    };

    unsafe {
        CNT += 1;
    }

    (res, false)
}

fn merge_sort(
    ordered: &mut Vec<usize>,
    q: usize,
    source: &mut LineSource<BufReader<StdinLock<'_>>>,
) -> Vec<usize> {
    if ordered.len() == 1 {
        return ordered.clone();
    }

    let mut l = Vec::<usize>::new();
    let mut r = Vec::<usize>::new();
    for i in 0..ordered.len() {
        if i < ordered.len() / 2 {
            l.push(ordered[i]);
        } else {
            r.push(ordered[i]);
        }
    }

    let l = merge_sort(&mut l, q, source);
    let r = merge_sort(&mut r, q, source);

    let mut res = Vec::<usize>::new();
    let mut i = 0;
    let mut j = 0;
    while i < l.len() || j < r.len() {
        if i == l.len() {
            res.push(r[j]);
            j += 1;
            continue;
        }
        if j == r.len() {
            res.push(l[i]);
            i += 1;
            continue;
        }

        let mut l2 = Vec::<usize>::new();
        let mut r2 = Vec::<usize>::new();
        l2.push(l[i]);
        r2.push(r[j]);
        let res2 = query(&l2, &r2, q, source);
        if res2.0 == '<' {
            res.push(l[i]);
            i += 1;
        } else {
            res.push(r[j]);
            j += 1;
        }
    }

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

    // ordered_idx[i] : i 番目に小さいものの index
    let mut ordered_idx = vec![0; n];
    for i in 0..n {
        ordered_idx[i] = i;
    }
    ordered_idx = merge_sort(&mut ordered_idx, q, &mut source);

    // idx_order[i] : i が何番目に小さいか
    let mut idx_order = vec![0; n];
    for i in 0..n {
        idx_order[ordered_idx[i]] = i;
    }

    println!("# {:?}", ordered_idx);

    let mut ans = vec![0; n];
    for i in 0..n {
        let idx_temp = i % (2 * d);
        let idx = if idx_temp < d {
            idx_temp
        } else {
            2 * d - idx_temp - 1
        };
        ans[ordered_idx[n - i - 1]] = idx;
    }
    let mut non_changable = HashSet::<usize>::new();

    loop {
        if non_changable.len() == n {
            let res = query(&vec![0], &vec![1], q, &mut source);
            if res.1 {
                // query limit exceeded
                break;
            }
        }

        let idx = rng.gen_range(0..n);
        let to = rng.gen_range(0..d);
        if ans[idx] == to {
            continue;
        }
        if non_changable.contains(&ans[idx]) || non_changable.contains(&to) {
            continue;
        }
        if idx_order[idx] > n / 5 {
            continue;
        }
        let mut l = Vec::<usize>::new();
        let mut r = Vec::<usize>::new();
        for i in 0..n {
            if ans[i] == ans[idx] {
                l.push(i);
            }
            if ans[i] == to {
                r.push(i);
            }
        }

        output_answer(&ans, true);
        let res = query(&l, &r, q, &mut source);
        if res.1 {
            // query limit exceeded
            break;
        }

        if res.0 == '<' {
            // do nothing
        }
        if res.0 == '>' {
            ans[idx] = to;
        }
        if res.0 == '=' {
            non_changable.insert(ans[idx]);
            non_changable.insert(to);
        }
    }

    output_answer(&ans, false);
}
