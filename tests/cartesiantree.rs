#[derive(Debug)]
struct CartesianTree {
    n: usize,
    root: usize,
    l: Vec<Option<usize>>,
    r: Vec<Option<usize>>,
}

impl CartesianTree {
    fn from<T>(a: &[T]) -> Self
    where
        T: Ord,
    {
        let n = a.len();
        assert!(n > 0);

        let mut l = vec![None; n];
        let mut r = vec![None; n];
        let mut stack: Vec<usize> = Vec::new();

        for i in 0..n {
            let mut last = None;
            while let Some(&j) = stack.last() {
                if a[j] >= a[i] {
                    break;
                }
                stack.pop();
                last = Some(j);
            }

            if let Some(&j) = stack.last() {
                r[j] = Some(i);
            }
            if let Some(j) = last {
                l[i] = Some(j);
            }
            stack.push(i);
        }

        let root = stack[0];
        Self { n, root, l, r }
    }
}
#[test]
fn test_increasing() {
    let a = vec![1, 2, 3, 4, 5];
    let ct = CartesianTree::from(&a);

    assert_eq!(ct.root, 4);

    assert_eq!(ct.l[4], Some(3));
    assert_eq!(ct.l[3], Some(2));
    assert_eq!(ct.l[2], Some(1));
    assert_eq!(ct.l[1], Some(0));

    for i in 0..5 {
        assert_eq!(ct.r[i], None);
    }
}
#[test]
fn test_random() {
    let a = vec![3, 1, 4, 5, 2];
    let ct = CartesianTree::from(&a);

    let max_index = a.iter().enumerate().max_by_key(|(_, v)| *v).unwrap().0;
    assert_eq!(ct.root, max_index);

    check_properties(&a, &ct, ct.root);
}
fn check_properties<T: Ord>(a: &[T], ct: &CartesianTree, v: usize) {
    if let Some(l) = ct.l[v] {
        assert!(a[l] <= a[v]);
        check_properties(a, ct, l);
    }
    if let Some(r) = ct.r[v] {
        assert!(a[r] <= a[v]);
        check_properties(a, ct, r);
    }
}
