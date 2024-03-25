use std::{
    array,
    ops::{Deref, DerefMut},
};

/// Array that provides a [`Default`] impl for any `N`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Array<T, const N: usize>(pub [T; N]);

impl<T: Default, const N: usize> Default for Array<T, N> {
    fn default() -> Self {
        Self(array::from_fn(|_| Default::default()))
    }
}

impl<T, const N: usize> Deref for Array<T, N> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize> DerefMut for Array<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
