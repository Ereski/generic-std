//! Single-threaded reference-counting pointers.

use crate::{plug::PlugType, Rcb, WeakRcb};
use std::rc::{Rc, Weak};

/// HKT `std::rc::Rc<T>` with a type slot.
pub struct H1Rc;

impl<T> PlugType<T> for H1Rc {
    type T = Rc<T>;
}

/// HKT `std::rc::Weak<T>` with a type slot.
pub struct H1Weak;

impl<T> PlugType<T> for H1Weak {
    type T = Weak<T>;
}

impl<T> Rcb<T> for Rc<T> {
    type Weak = Weak<T>;

    fn new(x: T) -> Self {
        Rc::<T>::new(x)
    }

    fn try_unwrap(this: Self) -> Result<T, Self> {
        Rc::<T>::try_unwrap(this)
    }

    fn downgrade(this: &Self) -> Self::Weak {
        Rc::<T>::downgrade(this)
    }
}

impl<T> WeakRcb<T> for Weak<T> {
    type Strong = Rc<T>;

    fn upgrade(&self) -> Option<Self::Strong> {
        Weak::<T>::upgrade(self)
    }
}
