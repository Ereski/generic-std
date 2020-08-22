//! Implementation of higher-kinded types in Rust using associated types.
//!
//! This is a variation of
//! [Edmund Smith's](https://gist.github.com/edmundsmith/855fcf0cb35dd467c29a9350481f0ecf)
//! method where instead of unplugging and plugging generic arguments between
//! fully defined types (i.e. `Vec<X>` <-> `Vec<Y>`), higher-kinded types are
//! represented as structs that implement either `PlugLifetime` or
//! `PlugType`. We refer to these structs as *HKT forms*. To be explicit,
//! sometimes we refer to types that are not HKT forms as *concrete types* or
//! `H0` types.
//!
//! For example, `H1Vec`, the higher-kinded form of `Vec`, implements
//! `PlugType`. Then, to get the concrete `Vec<T>` for a type `T`:
//!
//! ```text
//! <H1Vec as PlugType<T>>::T
//! ```
//!
//! # Conventions for HTK Forms
//!
//! As a convention, all HKT forms must be zero-sized structs, only implement
//! `PlugLifetime` or `PlugType` and respect the following naming convention:
//!
//! ```text
//! H<n><t>
//! ```
//!
//! Where `<n>` is the number of lifetype + type arguments left to be filled,
//! also referred to as slots, and `<t>` is the name of the concrete type. For
//! example, `Cow` has two HKT forms:
//!
//! - `H2Cow` which implements `PlugLifetime`, yielding `H1Cow<'a>`
//! - `H1Cow<'a>` which implements `PlugType`, yielding a concrete `Cow<'a, T>`
//!
//! The generic arguments are always filled from left to right, lifetimes
//! first. In some cases it might be useful to plug those out of order. In
//! those cases we prepend something descriptive to the type name. See for
//! example [`TypedH1Reference`](../reference/struct.TypedH1Reference.html).
//!
//! # HKT-Compatible Concrete Types
//!
//! The `PlugLifetime` and `PlugType` may also be implemented for concrete
//! types, in which case `Type` is just itself. This is useful to implement
//! streaming iterators and similar constructs. [`H0`](struct.H0.html) is a
//! type wrapper for exactly this case.

use std::marker::PhantomData;

/// Trait enabling a lifetime to plugged to HKT forms.
pub trait PlugLifetime<'a> {
    /// The resulting type after plugging the lifetime parameter `'a`.
    type T;
}

/// Trait enabling a type to be plugged to HKT forms.
pub trait PlugType<T>
where
    T: ?Sized,
{
    /// The resulting type after plugging the type parameter `T`.
    type T;
}

/// Type-level wrapper that yields `T` unmodified when `PlugLifetime` or
/// `PlugType` are applied.
pub struct H0<T>(PhantomData<T>);

impl<'dummy, T> PlugLifetime<'dummy> for H0<T> {
    type T = T;
}

impl<Dummy, T> PlugType<Dummy> for H0<T> {
    type T = T;
}
