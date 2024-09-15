pub use use_camera::*;
pub use use_pointer::*;
pub use use_toolbar::*;

pub mod components;
mod use_camera;
mod use_pointer;
mod use_stack;
mod use_toolbar;

use std::{ops::Deref, rc::Rc};

/// An [`Rc`] that can be compared for **referential** equality using [`PartialEq`] and [`Eq`].
#[derive(Debug)]
pub struct EqRc<T>(Rc<T>);

impl<T> Clone for EqRc<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T> Deref for EqRc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<T> PartialEq for EqRc<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<T> Eq for EqRc<T> {}

impl<T> EqRc<T> {
    /// Create a new [`EqRc`].
    pub fn new(value: T) -> Self {
        Self(Rc::new(value))
    }

    /// Get the [`Rc`] corresponding to this [`EqRc`].
    pub fn into_inner(self) -> Rc<T> {
        self.0
    }
}

impl<T> From<Rc<T>> for EqRc<T> {
    fn from(rc: Rc<T>) -> Self {
        Self(rc)
    }
}

impl<T> From<EqRc<T>> for Rc<T> {
    fn from(eq_rc: EqRc<T>) -> Self {
        eq_rc.into_inner()
    }
}

impl<T> Default for EqRc<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}
