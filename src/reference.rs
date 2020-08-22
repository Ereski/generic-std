//! HKT forms for references.

use crate::plug::{PlugLifetime, PlugType};
use std::marker::PhantomData;

/// HKT `&'a T` with a lifetime and a type slot.
pub struct H2Reference;

impl<'a> PlugLifetime<'a> for H2Reference {
    type T = H1Reference<'a>;
}

/// HKT `&'a T` with a type slot.
pub struct H1Reference<'a>(PhantomData<&'a ()>);

impl<'a, T> PlugType<T> for H1Reference<'a>
where
    T: 'a + ?Sized,
{
    type T = &'a T;
}

/// HKT `&'a T` with a lifetime slot.
pub struct TypedH1Reference<T>(PhantomData<T>)
where
    T: ?Sized;

impl<'a, T> PlugLifetime<'a> for TypedH1Reference<T>
where
    T: 'a + ?Sized,
{
    type T = &'a T;
}
