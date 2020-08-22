//! Standard generic traits.
//!
//! # Why
//!
//! Did you ever need a generic `Sequence` trait? Ever wanted a data structure
//! that was generic over either `Rc` or `Arc`? An iterator that returns
//! borrowed data?
//!
//! These are not usually thought to be possible in stable Rust due to lack of
//! native support for higher-kinded types (HKTs). HKTs is a feature that would
//! allow us to reason about, say, `Vec` without fully defining it with type
//! arguments (`Vec<T>`). The classic example that is usually thought
//! impossible is the streaming (or borrowing) iterator, where `next()` returns
//! an item wih a lifetime bound by the iterator itself:
//!
//! ```compile_fail
//! trait StreamingIterator {
//!   type Item = &str;
//!   fn next<'a>(&'a mut self) -> Option<&'a str> {
//!     unimplemented!()
//!   }
//! }
//! ```
//!
//! This does not compile because the reference `&str` must be declared with a
//! lifetime, but we only have the lifetime for `self` in `next()` itself and
//! not in the associated type declaration. Unlike `type` aliases, associated
//! types (`type` in a trait) cannot have type arguments. That is, the
//! following is not valid:
//!
//! ```compile_fail
//! trait StreamingIterator {
//!   type Item<'a> = &'a str;
//!   fn next<'a>(&'a mut self) -> Option<&'a str> {
//!     unimplemented!()
//!   }
//! }
//! ```
//!
//! This is called an associated type constructor. For more information, see
//! the
//! [RFC](https://github.com/rust-lang/rfcs/blob/master/text/1598-generic_associated_types.md)
//! and
//! [Nicholas'](https://smallcultfollowing.com/babysteps/blog/2016/11/02/associated-type-constructors-part-1-basic-concepts-and-introduction/)
//! ATC post series.
//!
//! However, it is possible to emulate this behaviour in stable Rust with more
//! boilerplate. See the [`StreamingIterator`](trait.StreamingIterator.html)
//! trait.
//!
//! # How
//!
//! This library implements multiple generic traits for `std` types using a
//! variation of
//! [Edmund Smith's](https://gist.github.com/edmundsmith/855fcf0cb35dd467c29a9350481f0ecf)
//! method to emulate higher-kinded types. See the [plug](plug/index.html)
//! module for details.
//!
//! # Limitations
//!
//! HKT for types seems to be complete using the plug method. That is, any
//! functionality you could get with native HKT support for types, you can get
//! with this method. Ergonomy is not great though, even if it works.
//!
//! There are limitations regarding HKT lifetimes due to the fact that is
//! impossible to put bounds on HRTB lifetimes. That is, something like
//! `for<'a: 'b>` is inexpressible. As a result some traits and impls may have
//! more restrictive lifetime bounds than necessary.
//!
//! # Current Status
//!
//! This crate is highly experimental and many traits have limited
//! functionality.

pub mod plug;
pub mod rc;
pub mod reference;
pub mod slice;
pub mod sync;
pub mod vec;

#[cfg(test)]
mod tests;

use crate::plug::*;
use std::ops::Deref;

/// Trait for structs that can be constructed with a preallocated capacity.
pub trait WithCapacity {
    fn with_capacity(capacity: usize) -> Self;
}

/// Trait for collections that store elements in a linear sequence, allowing
/// for linear traversal and indexing with an `usize`.
///
/// # Note
///
/// The `H1Iterator` bounds are not specific enough due to language
/// limitations. The correct declaration for the trait and `H1Iterator` is:
///
/// ```compile_fail
/// trait Sequence<'a, T>
/// where
///   T: 'a
/// {
///   type H1Iterator: for<'b> PlugLifetime<'b>
///   where
///     'a: 'b,
///     <H1Iterator as PlugLifetime<'b>>::T: StreamingIterator;
///   ...
/// }
/// ```
///
/// Because we can't declare `'a: 'b`, implementors must only allow
/// `T: 'static`. In other words, we can't declare that all items outlive the
/// iterator for any specific lifetime (so that stuff like `next()` returning a
/// `&T` is valid if `T` contains references), so we must use a shotgun and
/// prohibit `T` from containing any non-`'static` references.
///
/// Also, where clauses in associated types are not stable so we have to move
/// `StreamingIterator` bound to the `iter()` declaration.
pub trait Sequence<T> {
    /// HKT iterator with a lifetime slot.
    type H1Iterator: for<'a> PlugLifetime<'a>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn contains<'a>(&'a self, x: &T) -> bool
    where
        T: PartialEq;

    fn get(&self, index: usize) -> Option<&T>;

    fn first(&self) -> Option<&T> {
        self.get(0)
    }

    fn last(&self) -> Option<&T> {
        self.get(self.len() - 1)
    }

    fn iter<'a>(&'a self) -> <Self::H1Iterator as PlugLifetime<'a>>::T
    where
        <Self::H1Iterator as PlugLifetime<'a>>::T: StreamingIterator;
}

/// Trait for mutable collections that store elements in a linear sequence,
/// allowing for linear traversal and indexing with an `usize`.
pub trait SequenceMut<T> {
    fn capacity(&self) -> usize;

    fn clear(&mut self);

    fn reserve(&mut self, additional: usize);

    fn reserve_exact(&mut self, additional: usize);

    fn shrink_to_fit(&mut self);

    fn push(&mut self, x: T);

    fn pop(&mut self) -> Option<T>;

    fn insert(&mut self, index: usize, x: T);

    fn remove(&mut self, index: usize) -> T;
}

/// Trait for iterators that can return elements borrowed from itself.
pub trait StreamingIterator {
    /// HTK item with a lifetime slot.
    type H1Item: for<'a> PlugLifetime<'a>;

    fn next(&mut self) -> Option<<Self::H1Item as PlugLifetime>::T>;
}

impl<I> StreamingIterator for I
where
    I: Iterator,
{
    type H1Item = H0<I::Item>;

    fn next(&mut self) -> Option<<Self::H1Item as PlugLifetime>::T> {
        Iterator::next(self)
    }
}

/// Trait for reference-counted boxes.
pub trait Rcb<T>: Clone + Deref<Target = T> {
    type Weak: WeakRcb<T>;

    fn new(x: T) -> Self;

    fn try_unwrap(this: Self) -> Result<T, Self>;

    fn downgrade(this: &Self) -> Self::Weak;
}

/// Trait for weak pointers to reference-counted boxes.
pub trait WeakRcb<T> {
    type Strong: Rcb<T>;

    fn upgrade(&self) -> Option<Self::Strong>;
}
