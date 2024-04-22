#![feature(as_array_of_cells, impl_trait_in_assoc_type)]

mod windowed;

use std::{
    array,
    ops::{Deref, DerefMut, Index, IndexMut},
    slice,
};

pub use self::windowed::{window_idx as W, WindowIdx, WindowedArray};

/// Array that provides a [`Default`] impl for any `N`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
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

impl<'a, T, const N: usize> IntoIterator for &'a Array<T, N> {
    type Item = &'a T;

    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut Array<T, N> {
    type Item = &'a mut T;

    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T, Idx, A: ?Sized, const N: usize> Index<Idx> for Array<T, N>
where
    [T; N]: Index<Idx, Output = A>,
{
    type Output = A;

    fn index(&self, index: Idx) -> &Self::Output {
        self.0.index(index)
    }
}

impl<T, Idx, A: ?Sized, const N: usize> IndexMut<Idx> for Array<T, N>
where
    [T; N]: IndexMut<Idx, Output = A>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}
