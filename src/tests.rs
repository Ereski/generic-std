use crate::{
    plug::{PlugLifetime, PlugType},
    rc::H1Rc,
    reference::TypedH1Reference,
    sync::H1Arc,
    Rcb, StreamingIterator,
};
use async_executor::LocalExecutor;
use std::{
    future::Future,
    pin::Pin,
    rc::Rc,
    sync::Arc,
    task::{Context, Poll},
};

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

// Non-trivial async-like trait method. Has the advantage that there's no need
// to box the resulting future (like https://crates.io/crates/async-trait), but
// has the disadvantage of making the trait object-unsafe (can't be used as
// `dyn`). The implementation here is very manual and can probably be mostly
// abstracted by a macro.
//
// Also see:
// https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/
#[test]
fn async_trait_method() {
    struct H1RefFuture;

    impl<'a> PlugLifetime<'a> for H1RefFuture {
        type T = RefFuture<'a, usize>;
    }

    // This is the future that is returned by the implementation. This has to
    // be a named struct instead of an anonymous `impl Future`/`async` block
    // because it's referred by the `H1RefFuture` HKT form
    struct RefFuture<'a, T>(&'a T);

    impl<'a, T> Future for RefFuture<'a, T> {
        type Output = &'a T;

        fn poll(self: Pin<&mut Self>, _: &mut Context) -> Poll<Self::Output> {
            Poll::Ready(self.0)
        }
    }

    trait AsyncTrait {
        type H1NonTrivialFuture: for<'a> PlugLifetime<'a>;

        // This is basically the async identity function. The lifetime argument
        // is the non-trivial parts of this example
        fn non_trivial<'a>(
            &self,
            x: &'a usize,
        ) -> <Self::H1NonTrivialFuture as PlugLifetime<'a>>::T
        // By all rights this bound should be part of the declaration of
        // `H1NonTrivialFuture` or a separate associated type
        // `NonTrivialFuture`. Since `for<T>` is not a thing, we have to make
        // do with this
        where
            <Self::H1NonTrivialFuture as PlugLifetime<'a>>::T:
                Future<Output = &'a usize>;
    }

    struct AsyncTraitImpl;

    impl AsyncTrait for AsyncTraitImpl {
        type H1NonTrivialFuture = H1RefFuture;

        fn non_trivial<'a>(
            &self,
            x: &'a usize,
        ) -> <Self::H1NonTrivialFuture as PlugLifetime<'a>>::T
        where
            <Self::H1NonTrivialFuture as PlugLifetime<'a>>::T:
                Future<Output = &'a usize>,
        {
            RefFuture(x)
        }
    }

    let executor = LocalExecutor::new();
    let payload = 42_usize;
    let future = AsyncTraitImpl.non_trivial(&payload);

    assert_eq!(executor.run(future), &payload);
}
