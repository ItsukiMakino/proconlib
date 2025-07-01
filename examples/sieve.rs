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
fn main() {
    const M: usize = 100;
    // M以下の素数を全列挙する
    let sieve = Sieve::new(M);
    for prime in sieve.p {
        println!("{}", prime);
    }
}
