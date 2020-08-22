//! A contiguous growable array type with heap-allocated contents, written
//! `Vec<T>`.

use crate::{
    plug::{PlugLifetime, PlugType},
    slice::TypedH1Iter,
    Sequence, SequenceMut, StreamingIterator, WithCapacity,
};
use std::vec::Vec;

/// HKT `Vec` with a type slot.
pub struct H1Vec;

impl<T> PlugType<T> for H1Vec {
    type T = Vec<T>;
}

impl<T> WithCapacity for Vec<T> {
    fn with_capacity(capacity: usize) -> Self {
        Vec::<T>::with_capacity(capacity)
    }
}

impl<T> Sequence<T> for Vec<T>
where
    T: 'static,
{
    type H1Iterator = TypedH1Iter<T>;

    fn len(&self) -> usize {
        Vec::<T>::len(self)
    }

    fn is_empty(&self) -> bool {
        Vec::<T>::is_empty(self)
    }

    fn contains<'a>(&'a self, x: &T) -> bool
    where
        T: PartialEq,
    {
        <[T]>::contains(self, x)
    }

    fn get(&self, index: usize) -> Option<&T> {
        <[T]>::get(self, index)
    }

    fn first(&self) -> Option<&T> {
        <[T]>::first(self)
    }

    fn last(&self) -> Option<&T> {
        <[T]>::last(self)
    }

    fn iter<'a>(&'a self) -> <Self::H1Iterator as PlugLifetime<'a>>::T
    where
        <Self::H1Iterator as PlugLifetime<'a>>::T: StreamingIterator,
    {
        <[T]>::iter(self)
    }
}

impl<T> SequenceMut<T> for Vec<T> {
    fn capacity(&self) -> usize {
        Vec::<T>::capacity(self)
    }

    fn clear(&mut self) {
        Vec::<T>::clear(self)
    }

    fn reserve(&mut self, additional: usize) {
        Vec::<T>::reserve(self, additional)
    }

    fn reserve_exact(&mut self, additional: usize) {
        Vec::<T>::reserve_exact(self, additional)
    }

    fn shrink_to_fit(&mut self) {
        Vec::<T>::shrink_to_fit(self)
    }

    fn push(&mut self, x: T) {
        Vec::<T>::push(self, x)
    }

    fn pop(&mut self) -> Option<T> {
        Vec::<T>::pop(self)
    }

    fn insert(&mut self, index: usize, x: T) {
        Vec::<T>::insert(self, index, x)
    }

    fn remove(&mut self, index: usize) -> T {
        Vec::<T>::remove(self, index)
    }
}
