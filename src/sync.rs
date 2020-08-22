//! Useful synchronization primitives.

use crate::{plug::PlugType, Rcb, WeakRcb};
use std::sync::{Arc, Weak};

/// HKT `std::sync::Arc<T>` with a type slot.
pub struct H1Arc;

impl<T> PlugType<T> for H1Arc {
    type T = Arc<T>;
}

/// HKT `std::sync::Weak<T>` with a type slot.
pub struct H1Weak;

impl<T> PlugType<T> for H1Weak {
    type T = Weak<T>;
}

impl<T> Rcb<T> for Arc<T> {
    type Weak = Weak<T>;

    fn new(x: T) -> Self {
        Arc::<T>::new(x)
    }

    fn try_unwrap(this: Self) -> Result<T, Self> {
        Arc::<T>::try_unwrap(this)
    }

    fn downgrade(this: &Self) -> Self::Weak {
        Arc::<T>::downgrade(this)
    }
}

impl<T> WeakRcb<T> for Weak<T> {
    type Strong = Arc<T>;

    fn upgrade(&self) -> Option<Self::Strong> {
        Weak::<T>::upgrade(self)
    }
}
