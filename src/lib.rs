use std::iter::Peekable;

pub struct IteratorMerge<I: Iterator, F> {
    iters: Vec<Peekable<I>>,
    merge: F,
}

impl<I: Iterator, F: FnMut(&mut [Peekable<I>])->Option<I::Item>> IteratorMerge<I, F> {
    pub fn new(iters: Vec<I>, merge: F) -> Self {
        let iters = iters.into_iter().map(|i| i.peekable()).collect();
        Self { iters, merge }
    }
}

impl<I: Iterator, F: FnMut(&mut [Peekable<I>])->Option<I::Item>> Iterator for IteratorMerge<I, F> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        (self.merge)(&mut self.iters)
    }
}

#[test]
fn test_merge_sort() {
    let vals = [4u8, 2, 10, 6, 8, 5, 9, 3, 1, 7];

    fn merge_sort(es: &[u8]) -> Box<dyn Iterator<Item = u8>> {
        let nes = es.len();

        fn merge<I: Iterator<Item = u8>>(its: &mut[Peekable<I>]) -> Option<u8> {
            let mut vs = its.iter_mut().map(|v| v.peek());
            let v0 = vs.next().unwrap();
            let v1 = vs.next().unwrap();
            assert!(vs.next().is_none());
            if v1.is_none() {
                its[0].next()
            } else if v0.is_none() {
                its[1].next()
            } else if v0 <= v1 {
                its[0].next()
            } else {
                its[1].next()
            }
        }

        match nes {
            0 => Box::new(std::iter::empty()),
            1 => Box::new(std::iter::once(es[0])),
            n => {
                let split = n / 2;
                let i1 = merge_sort(&es[..split]);
                let i2 = merge_sort(&es[split..]);
                Box::new(IteratorMerge::new(vec![i1, i2], merge))
            }
        }
    }

    let sorted: Vec<u8> = merge_sort(&vals).collect();
    assert_eq!(&sorted, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}
