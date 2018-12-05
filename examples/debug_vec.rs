fn main() {
    let mut v = all_primes(10000);
    println!("v = {:?}", v);
}

fn all_primes(n: i32) -> Vec<i32> {
    let mut v = Vec::new();
    for x in 2..n {
        if is_prime(x) {
            v.push(x); // set breakpoint here to check vector elements
        }
    }
    v
}

fn is_prime(x: i32) -> bool {
    let mut i = 2;
    while i * i <= x {
        if x % i == 0 {
            return false;
        }
        i += 1;
    }
    return true
}