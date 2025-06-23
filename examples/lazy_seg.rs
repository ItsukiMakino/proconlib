use ac_library::{LazySegtree, MapMonoid, ModInt998244353 as Mint, Monoid};
use itertools::Itertools;
use proconio::{fastout, input};
// type Mint = ac_library::ModInt998244353;
// #[derive(Clone, Copy, Debug)]
// struct S {
//     ab: Mint,
//     a: Mint,
//     b: Mint,
//     len: usize,
// }

// #[derive(Clone, Copy, Debug)]
// struct F {
//     add_a: Mint,
//     add_b: Mint,
// }
// // 値モノイド：A・B・AB 積・長さ を保持
// struct ABMonoid;

// impl ac_library::Monoid for ABMonoid {
//     type S = S;

//     fn identity() -> Self::S {
//         S {
//             ab: Mint::new(0),
//             a: Mint::new(0),
//             b: Mint::new(0),
//             len: 0,
//         }
//     }

//     fn binary_operation(x: &Self::S, y: &Self::S) -> Self::S {
//         S {
//             ab: x.ab + y.ab,
//             a: x.a + y.a,
//             b: x.b + y.b,
//             len: x.len + y.len,
//         }
//     }
// }

// // 遅延セグメント木に必要な MapMonoid（区間加算）
// struct RangeAddABProduct;

// impl MapMonoid for RangeAddABProduct {
//     type M = ABMonoid;
//     type F = F;

//     fn identity_map() -> Self::F {
//         F {
//             add_a: Mint::new(0),
//             add_b: Mint::new(0),
//         }
//     }

//     fn mapping(
//         f: &Self::F,
//         x: &<Self::M as ac_library::Monoid>::S,
//     ) -> <Self::M as ac_library::Monoid>::S {
//         let ab = x.ab + f.add_a * x.b + f.add_b * x.a + f.add_a * f.add_b * x.len;
//         let a = x.a + f.add_a * x.len;
//         let b = x.b + f.add_b * x.len;
//         S {
//             ab,
//             a,
//             b,
//             len: x.len,
//         }
//     }

//     fn composition(f: &Self::F, g: &Self::F) -> Self::F {
//         F {
//             add_a: f.add_a + g.add_a,
//             add_b: f.add_b + g.add_b,
//         }
//     }
// }
struct Sum;
impl Monoid for Sum {
    type S = (Mint, usize); // (a, size)

    fn identity() -> Self::S {
        (0.into(), 0)
    }

    fn binary_operation(&(a0, n0): &Self::S, &(a1, n1): &Self::S) -> Self::S {
        (a0 + a1, n0 + n1)
    }
}

struct Affine;
impl MapMonoid for Affine {
    type M = Sum;
    type F = (Mint, Mint); // (b, c)

    fn identity_map() -> Self::F {
        (1.into(), 0.into())
    }

    fn mapping(&(b, c): &Self::F, &(a, n): &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        (b * a + c * Mint::new(n), n)
    }

    // b0 * (b1 * a1 + c1) + c0 = (b0 * b1) * a1 + (b0 * c1 + c0)
    fn composition(&(b0, c0): &Self::F, &(b1, c1): &Self::F) -> Self::F {
        (b0 * b1, b0 * c1 + c0)
    }
}
#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [Mint; n],
    }

    let mut segtree: LazySegtree<Affine> = a.iter().map(|&x| (x, 1usize)).collect_vec().into();
    for _ in 0..q {
        input! { kind: usize }
        match kind {
            0 => {
                input! {
                    l: usize,
                    r: usize,
                    b: Mint,
                    c: Mint,
                }
                segtree.apply_range(l..r, (b, c));
            }
            1 => {
                input! {
                    l: usize,
                    r: usize,
                }
                println!("{}", segtree.prod(l..r).0);
            }
            _ => unreachable!(),
        }
    }
}
