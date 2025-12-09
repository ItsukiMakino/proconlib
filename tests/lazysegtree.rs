use ac_library::{MapMonoid, Min, Monoid};

// struct M;
// impl Monoid for M {
//     type S = usize;
//     fn identity() -> Self::S {
//         0
//     }

//     fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
//         a ^ b
//     }
// }
#[derive(Clone, Copy)]
struct F;
impl MapMonoid for F {
    type M = Min<usize>;
    type F = usize;

    fn identity_map() -> Self::F {
        0
    }
    //遅延成分を要素に適用する
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        x + f
    }
    //遅延成分同士の合成
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f + g
    }
}
#[test]
fn should_one() {
    use ac_library::LazySegtree;
    let n = 5;
    let mut seg = LazySegtree::<F>::new(n);
    for i in 0..n {
        seg.set(i, i + 1);
    }
    assert_eq!(seg.all_prod(), 1);

    seg.apply_range(0..3, 2usize);
    assert_eq!(seg.all_prod(), 3);

    seg.apply(0, 4);
    assert!(seg.get(0) == 7);
}
