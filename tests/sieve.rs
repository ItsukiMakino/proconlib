struct Sieve {
    p: Vec<usize>,
    is_prime: Vec<bool>,
}
impl Sieve {
    fn new(n: usize) -> Self {
        let mut p = vec![];
        let mut is_prime = vec![true; n + 1];
        is_prime[0] = false;
        is_prime[1] = false;
        for i in 2..=n {
            if is_prime[i] {
                p.push(i);
                let mut now = i;
                while now + i <= n {
                    now += i;
                    is_prime[now] = false;
                }
            }
        }
        Sieve { p, is_prime }
    }
}
#[test]
fn sieve_primes_up_to_10() {
    let sieve = Sieve::new(10);
    assert_eq!(sieve.p, vec![2, 3, 5, 7]);
}

#[test]
fn sieve_is_prime_flags_up_to_10() {
    let sieve = Sieve::new(10);
    let expected = vec![
        false, false, true, true, false, true, false, true, false, false, false,
    ];
    assert_eq!(sieve.is_prime, expected);
}

#[test]
fn sieve_primes_up_to_100() {
    let sieve = Sieve::new(100);
    let expected = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97,
    ];
    assert_eq!(sieve.p, expected);
}

#[test]
fn sieve_empty_case() {
    let sieve = Sieve::new(1);
    assert_eq!(sieve.p, vec![]);
    assert_eq!(sieve.is_prime, vec![false, false]);
}
