use std::env;
use bit_vec::BitVec;

fn main() {
    let mut args = env::args();

    let n = args.nth(1).unwrap_or_else(|| {
        eprintln!("Insert a positive number");
        std::process::exit(1);
    });

    let n = n.parse::<usize>().unwrap_or_else(|_| {
        eprintln!("Invalid positive number");
        std::process::exit(1);
    });

    let mut is_composite = BitVec::from_elem(n, false);

    let limit = usize::isqrt(n - 1);

    for p in 2..=limit {
        if is_composite[p] { continue; }

        for multiple in (p*p..n).step_by(p) {
            is_composite.set(multiple, true);
        }
    }

    for i in 2..n {
        if !is_composite[i] {
            println!("{}", i);
        }
    }
}
