pub fn nth(n: u32) -> Option<u32> {
    match n {
        n if n == 0 => None,
        n => (1..).filter(|i| prime(*i)).nth(n as usize),
    }
}

pub fn prime(n: u32) -> bool {
    !(2..n-1).any(|i| n % i == 0)
}
