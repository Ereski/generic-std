//! A dynamically-sized view into a contiguous sequence, [T].

use crate::plug::PlugLifetime;
use std::marker::PhantomData;
use std::slice::Iter;

/// HTK `&'a [T]` iterator with a lifetime slot
pub struct TypedH1Iter<T>(PhantomData<T>);

impl<'a, T> PlugLifetime<'a> for TypedH1Iter<T>
where
    T: 'a,
{
    type T = Iter<'a, T>;
}
