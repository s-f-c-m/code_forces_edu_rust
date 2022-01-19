use std::io;

fn count_sort(p: &Vec<usize>, c: &Vec<usize>) -> Vec<usize> {
    let n = p.len();
    let mut cnt: Vec<usize> = vec![0; n];
    for x in c.clone() {
        cnt[x] += 1;
    }

    let mut p_new: Vec<usize> = vec![0; n];
    let mut pos: Vec<usize> = vec![0; n];
    // pos[0] = 0;
    for i in 1..n {
        pos[i] = pos[i - 1] + cnt[i - 1];
    }

    for x in p.clone() {
        let i = c[x];
        p_new[pos[i]] = x;
        pos[i] += 1;
    }

    p_new
}

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
    for ch in s.chars() {
        a[idx] = (ch, idx);
        idx += 1;
    }

    a.sort_by_key(|k| k.0);

    for i in 0..n {
        p[i] = a[i].1;
    }

    // c[p[0]] = 0;

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

        for i in 0..n {
            let tmp: i32 = (p[i] as i32) - (1 << k);
            let tmp2 = (tmp + (n as i32)) as usize;
            p[i] = tmp2 % n;
        }

        // let a = radix_sort(a);
        p = count_sort(&p, &c);

        let mut c_new: Vec<usize> = vec![0; n];
        // c_new[p[0]] = 0;

        for i in 1..n {
            let prev: (usize, usize) = (c[p[i - 1]], c[(p[i - 1] + (1 << k)) % n]);
            let now: (usize, usize) = (c[p[i]], c[(p[i] + (1 << k)) % n]);
            if now == prev {
                c_new[p[i]] = c_new[p[i - 1]];
            } else {
                c_new[p[i]] = c_new[p[i - 1]] + 1;
            }
        }
        c = c_new;
        k += 1;
    }

    for i in p {
        print!("{} ", i);
    }
}
