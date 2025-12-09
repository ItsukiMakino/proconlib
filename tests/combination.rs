type Mint = ac_library::ModInt998244353;
pub struct Combination {
    fact: Vec<Mint>,
    ifact: Vec<Mint>,
}

impl Combination {
    pub fn new(n: usize) -> Self {
        let mut fact: Vec<Mint> = vec![1.into(); n + 1];
        let mut ifact: Vec<Mint> = vec![1.into(); n + 1];

        for i in 1..=n {
            fact[i] = fact[i - 1] * Mint::new(i as u32);
        }

        ifact[n] = fact[n].inv();

        for i in (1..=n).rev() {
            ifact[i - 1] = ifact[i] * Mint::new(i as u32);
        }
        Self { fact, ifact }
    }

    pub fn comb(&self, n: usize, k: usize) -> Mint {
        if k > n {
            Mint::new(0)
        } else {
            self.fact[n] * self.ifact[k] * self.ifact[n - k]
        }
    }
}

#[test]
fn should_one() {
    let n = 10;
    let c = Combination::new(n);
    assert_eq!(c.comb(n, 0), 1.into());
}
