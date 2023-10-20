use proconio::{input, source::line::LineSource};
use std::{
    io::{stdin, BufReader, StdinLock},
    mem::swap,
};
extern crate rand;
use rand::Rng;

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
    ans: &Vec<usize>,
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

    let l = merge_sort(&mut l, q, ans, source);
    let r = merge_sort(&mut r, q, ans, source);

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

        let mut l_vec = Vec::<usize>::new();
        let mut r_vec = Vec::<usize>::new();
        for k in 0..ans.len() {
            if ans[k] == l[i] {
                l_vec.push(k);
            }
            if ans[k] == r[j] {
                r_vec.push(k);
            }
        }
        let res2 = query(&l_vec, &r_vec, q, source);
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

    let mut ans = vec![0; n];
    for i in 0..n {
        ans[i] = i % d;
    }

    // ordered_idx[i] : i 番目に小さいものの index
    let mut ordered_idx = vec![0; d];
    for i in 0..d {
        ordered_idx[i] = i;
    }
    ordered_idx = merge_sort(&mut ordered_idx, q, &ans, &mut source);

    println!("# {:?}", ordered_idx);
    let mut maximum_idx = d - 1;

    loop {
        if maximum_idx == 0 {
            let res = query(&vec![0], &vec![1], q, &mut source);
            if res.1 {
                // query limit exceeded
                break;
            }
        }
        let mut minim = Vec::<usize>::new();
        let mut maxim = Vec::<usize>::new();
        for i in 0..n {
            if ans[i] == ordered_idx[0] {
                minim.push(i);
            }
            if ans[i] == ordered_idx[maximum_idx] {
                maxim.push(i);
            }
        }
        if maxim.len() <= 1 {
            maximum_idx -= 1;
            continue;
        }

        let idx = {
            let mut remaining = Vec::<usize>::new();
            for i in 0..maxim.len() {
                remaining.push(i);
            }
            let mut min = {
                let idx = rng.gen_range(0..remaining.len());
                remaining.remove(idx);
                idx
            };
            let mut cnt = 0;
            loop {
                if remaining.len() == 0 {
                    break;
                }
                let other_idx = rng.gen_range(0..remaining.len());
                let other = remaining[other_idx];
                remaining.remove(other_idx);
                let res = query(&vec![maxim[min]], &vec![maxim[other]], q, &mut source);
                if res.1 {
                    // query limit exceeded
                    break;
                }
                if res.0 == '>' {
                    min = other;
                }
                cnt += 1;
                if cnt >= 3 {
                    break;
                }
            }
            min
        };
        let mut minim_idx = ordered_idx[0];
        let mut maxim_idx = ordered_idx[maximum_idx];
        ans[maxim[idx]] = minim_idx;
        minim.push(maxim[idx]);
        maxim.remove(idx);

        println!("# minim: {:?}", minim);
        println!("# maxim: {:?}", maxim);
        let res = query(&minim, &maxim, q, &mut source);
        if res.1 {
            // query limit exceeded
            break;
        }
        if res.0 == '>' {
            swap(&mut minim, &mut maxim);
            swap(&mut minim_idx, &mut maxim_idx);
        }

        let mut new_ordered = Vec::<usize>::new();
        let mut todo_vec = Vec::<usize>::new();
        for i in 0..d {
            if ordered_idx[i] == minim_idx || ordered_idx[i] == maxim_idx {
                continue;
            }
            todo_vec.push(ordered_idx[i]);
        }

        // sort minim
        {
            let mut l = 0;
            let mut r = todo_vec.len();
            while r - l > 1 {
                let mid = (l + r) / 2;
                let mut midvec = Vec::<usize>::new();
                for i in 0..n {
                    if ans[i] == todo_vec[mid] {
                        midvec.push(i);
                    }
                }
                println!(
                    "# minim l={} r={} mid={} minim={} maxim={}",
                    l, r, mid, minim_idx, maxim_idx
                );
                let res = query(&minim, &midvec, q, &mut source);
                if res.0 == '<' {
                    r = mid;
                } else {
                    l = mid;
                }
            }
            for i in 0..l {
                new_ordered.push(todo_vec[i]);
            }
            new_ordered.push(minim_idx);
            let mut todo_tmp = Vec::<usize>::new();
            for i in l..todo_vec.len() {
                todo_tmp.push(todo_vec[i]);
            }
            swap(&mut todo_vec, &mut todo_tmp);
        }
        // sort maxim
        {
            let mut l = 0;
            let mut r = todo_vec.len();
            while r - l > 1 {
                let mid = (l + r) / 2;
                let mut midvec = Vec::<usize>::new();
                for i in 0..n {
                    if ans[i] == todo_vec[mid] {
                        midvec.push(i);
                    }
                }
                println!(
                    "# maxim l={} r={} mid={} minim={} maxim={}",
                    l, r, mid, minim_idx, maxim_idx
                );
                let res = query(&maxim, &midvec, q, &mut source);
                if res.0 == '<' {
                    r = mid;
                } else {
                    l = mid;
                }
            }
            for i in 0..l {
                new_ordered.push(todo_vec[i]);
            }
            new_ordered.push(maxim_idx);
            for i in l..todo_vec.len() {
                new_ordered.push(todo_vec[i]);
            }
        }
        swap(&mut ordered_idx, &mut new_ordered);

        output_answer(&ans, true);
    }

    output_answer(&ans, false);
}
