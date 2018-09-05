pub fn nth(n: u32) -> Option<u32> {
    if n <= 0 { return None }
    let mut nth_prime_found = false;
    let mut nth_prime: u32 = 0;  
    let mut counter: u32 = 0;
    while nth_prime_found == false {
        let primes = sieve(counter);
        if primes.len() as u32 == n { 
            nth_prime_found = true;
            nth_prime = primes[(primes.len()-1) as usize] as u32
        }
        counter += 1;
    }
    Some(nth_prime)
}

pub fn sieve(n: u32) -> Vec<usize> {
    let mut is_prime = vec![true; (n+1) as usize];
    is_prime[0] = false;
    if n >= 1 { is_prime[1] = false }

    for i in 2..n+1 {
        if is_prime[i as usize] {
            let mut multiple = i.pow(2);
            while multiple <= n {
                is_prime[multiple as usize] = false;
                multiple += i;
            }
        }
    }

    let primes: Vec<usize> = is_prime.iter().enumerate()
        .filter_map(|(pr, &is_pr)| if is_pr {Some(pr)} else {None} )
        .collect();

    primes
}