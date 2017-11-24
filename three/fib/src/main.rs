fn main() {
    let recursive = [false, true];
    let fib_actuals = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34];

    for truth in recursive.iter() {
        if *truth == true {
            println!("Running recursive fibonacci sequence...");
        } else {
            println!("Running non-recursive fibonacci sequence...");
        };

        for nth_fib in 0..10 {
            let fib_calc = fib(&nth_fib, truth);
            println!("{:?}", fib_calc);
            assert!(fib_calc == fib_actuals[nth_fib])
        };
    };
}

fn fib(n : &usize, recurse : &bool) -> u32 {
    let mut fib = 0;
    if *recurse {
        println!("Calling recursive method on n={}", *n);
        fib = fib_recur(&n);
    } else {
        println!("Calling non-recursive method on n={}", *n);
        fib_iter(&mut fib, &n);
    }

    fib
}

fn fib_recur(n: &usize) -> u32 {
    let fib = if *n == 0 {
        0
    } else if *n == 1 {
        1
    } else {
        let fib_n = fib_recur(&(n-1)) + fib_recur(&(n-2));
        fib_n
    };

    fib
}

fn fib_iter(fib : &mut u32, n : &usize) {
    *fib = if *n == 0 {
        0
    } else {
        let mut last = 0;
        let mut curr = 1;
        let mut next;
        for m in 1..*n {
                next = curr + last;
                last = curr;
                curr = next;
        };
        curr
    };
}
