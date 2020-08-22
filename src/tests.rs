use crate::{
    plug::{PlugLifetime, PlugType},
    rc::H1Rc,
    reference::TypedH1Reference,
    sync::H1Arc,
    Rcb, StreamingIterator,
};
use std::rc::Rc;
use std::sync::Arc;

#[test]
fn self_borrowing_iterator() {
    struct SelfBorrowingIterator {
        buf: [usize; 1],
    }

    impl StreamingIterator for SelfBorrowingIterator {
        type H1Item = TypedH1Reference<[usize]>;

        fn next(&mut self) -> Option<<Self::H1Item as PlugLifetime>::T> {
            if self.buf[0] == 2 {
                None
            } else {
                self.buf[0] += 1;

                Some(&self.buf)
            }
        }
    }

    let mut iter = SelfBorrowingIterator { buf: [0] };

    assert_eq!(iter.next(), Some([1].as_ref()));
    assert_eq!(iter.next(), Some([2].as_ref()));
    assert_eq!(iter.next(), None);
}

#[test]
fn struct_using_either_rc_or_arc() {
    #[derive(Clone)]
    struct StructWithReferenceCount<R> {
        secret: R,
    }

    impl<R> StructWithReferenceCount<R>
    where
        R: Rcb<String>,
    {
        fn new() -> Self {
            Self {
                secret: R::new("xpotato".to_string()),
            }
        }

        fn secret(&self) -> &str {
            self.secret.as_str()
        }
    }

    assert_eq!(
        StructWithReferenceCount::<Rc<String>>::new()
            .clone()
            .secret(),
        "xpotato"
    );
    assert_eq!(
        StructWithReferenceCount::<Arc<String>>::new()
            .clone()
            .secret(),
        "xpotato"
    );
}

#[test]
fn struct_using_either_rc_or_arc_with_hkt() {
    struct StructWithReferenceCount<R>
    where
        R: PlugType<String> + PlugType<usize>,
    {
        secret1: <R as PlugType<String>>::T,
        secret2: <R as PlugType<usize>>::T,
    }

    impl<R> StructWithReferenceCount<R>
    where
        R: PlugType<String> + PlugType<usize>,
        <R as PlugType<String>>::T: Rcb<String>,
        <R as PlugType<usize>>::T: Rcb<usize>,
    {
        fn new() -> Self {
            Self {
                secret1: <R as PlugType<String>>::T::new("xpotato".to_string()),
                secret2: <R as PlugType<usize>>::T::new(42),
            }
        }

        fn secrets(&self) -> (&str, usize) {
            (self.secret1.as_str(), *self.secret2)
        }
    }

    // Autoderive doesn't work in this case
    impl<R> Clone for StructWithReferenceCount<R>
    where
        R: PlugType<String> + PlugType<usize>,
        <R as PlugType<String>>::T: Rcb<String>,
        <R as PlugType<usize>>::T: Rcb<usize>,
    {
        fn clone(&self) -> Self {
            Self {
                secret1: self.secret1.clone(),
                secret2: self.secret2.clone(),
            }
        }
    }

    assert_eq!(
        StructWithReferenceCount::<H1Rc>::new().clone().secrets(),
        ("xpotato", 42)
    );
    assert_eq!(
        StructWithReferenceCount::<H1Arc>::new().clone().secrets(),
        ("xpotato", 42)
    );
}
