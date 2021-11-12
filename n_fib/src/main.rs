fn main() {
    let n = 50;
    let x = iter_n_fib(n);
    println!("{} Fibonnaci Number: {}", n, x)
}

// fn recursive_n_fib(n: u32) -> u32 {
//     if n <= 1 {
//         return 1;
//     } else {
//         return recursive_n_fib(n - 1) + recursive_n_fib(n - 2);
//     }
// }

fn iter_n_fib(n: i64) -> i64 {
    if n == 0 {
        return 1;
    }
    let mut i = 0;
    let mut x = 0;
    let mut first_prev = 1;
    let mut second_prev = 0;
    while i < n {
        x = first_prev + second_prev;
        second_prev = first_prev;
        first_prev = x;
        i += 1
    }
    return x;
}
