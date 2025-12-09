#[allow(unused)]
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
fn sieve_test() {
    const M: usize = 20;
    let sieve = Sieve::new(M);
    assert_eq!(sieve.p[0], 2);
    assert_eq!(sieve.p[1], 3);
    assert_eq!(sieve.p[2], 5);
    assert_eq!(sieve.p[3], 7);

    assert_eq!(sieve.is_prime[20], false);
    assert_eq!(sieve.is_prime[13], true);
}
