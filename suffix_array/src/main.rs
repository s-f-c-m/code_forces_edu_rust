// use std::env::args;
use std::io;
// use std::slice;

fn main() {
    // let args: Vec<String> = args().collect();
    let mut s: String = String::new();
    io::stdin().read_line(&mut s).expect("Error reading");
    s = s.trim().to_string();
    // let mut s = args[1].clone();
    s.push('$');

    let n = s.len();

    let mut p: Vec<usize> = vec![0; n];
    let mut c: Vec<usize> = vec![0; n];

    //k=0

    let mut a: Vec<(char, usize)> = vec![('$', 0); n];

    let mut idx = 0;
    for c in s.chars() {
        a[idx] = (c, idx);
        idx += 1;
    }

    a.sort_by_key(|k| k.0);

    for i in 0..n {
        p[i] = a[i].1;
    }

    c[p[0]] = 0;

    for i in 1..n {
        if a[i].0 == a[i - 1].0 {
            c[p[i]] = c[p[i - 1]];
        } else {
            c[p[i]] = c[p[i - 1]] + 1;
        }
    }

    let mut k = 0;

    while (1 << k) < n {
        //k -> k+1
        let mut a: Vec<((usize, usize), usize)> = vec![((0, 0), 0); n];

        for i in 0..n {
            a[i] = ((c[i], c[(i + (1 << k)) % n]), i);
        }
        a.sort_by_key(|k| k.0);

        for i in 0..n {
            p[i] = a[i].1;
        }

        c[p[0]] = 0;

        for i in 1..n {
            if a[i].0 == a[i - 1].0 {
                c[p[i]] = c[p[i - 1]];
            } else {
                c[p[i]] = c[p[i - 1]] + 1;
            }
        }

        k += 1;
    }

    for i in p {
        print!("{} ", i);
    }

    // println!("{:?}", p);
}
