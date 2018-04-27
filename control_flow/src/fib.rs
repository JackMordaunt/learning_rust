
// calculate generates the nth number in the fibbonacci sequence.
pub fn calculate(nth: u64) -> u64 {
    fib(nth).0
}

// fib: fast doubling.
// returns (F(n), F(n+1))
fn fib(n: u64) -> (u64, u64) {
    if n == 0{
        (0, 1)
    } else {
        let (a, b) = fib(n/2);
        let c = a * ((b * 2) - a);
        let d = a * a + b * b;
        if n % 2 == 0 {
            (c, d)
        } else {
            (d, c+d)
        }
    }
}
