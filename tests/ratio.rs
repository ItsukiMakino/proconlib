use std::cmp::Ordering;
#[derive(Debug, Clone, Eq)]
struct Rational {
    numerator: i64,
    denominator: i64,
}

impl Rational {
    pub fn new(numerator: i64, denominator: i64) -> Self {
        let mut denom = if denominator == 0 { 1 } else { denominator };
        let mut num = numerator;
        if denom < 0 {
            num = -num;
            denom = -denom;
        }
        use num::integer::gcd;
        let g = gcd(num, denom);
        Rational {
            numerator: num / g,
            denominator: denom / g,
        }
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        self.numerator == other.numerator && self.denominator == other.denominator
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.numerator as i128 * other.denominator as i128)
            .partial_cmp(&(other.numerator as i128 * self.denominator as i128))
    }
}
#[test]
fn ratio_test_eq() {
    let a = Rational::new(3, 3);
    let b = Rational::new(1, 1);
    assert!(b == a);
}
#[test]
fn ratio_test_greater() {
    let a = Rational::new(2, 5);
    let b = Rational::new(1, 3);
    assert!(a > b);
}
#[test]
fn ratio_test_smaller() {
    let a = Rational::new(2, 5);
    let b = Rational::new(3, 5);
    assert!(a < b);
}
