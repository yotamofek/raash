use std::{
    fmt::Debug,
    ops::{Deref, DerefMut, Index, IndexMut, RangeFrom},
};

use num_traits::AsPrimitive;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct WindowIdx(usize);

pub fn window_idx<T, Err>(t: T) -> WindowIdx
where
    T: TryInto<usize, Error = Err> + AsPrimitive<usize>,
    Err: Debug,
{
    WindowIdx(if !cfg!(debug_assertions) {
        t.as_()
    } else {
        t.try_into().unwrap()
    })
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct WindowedArray<A, const W_SIZE: usize>(pub A);

impl<A, const W_SIZE: usize> WindowedArray<A, W_SIZE> {
    const fn window_range(WindowIdx(idx): WindowIdx) -> RangeFrom<usize> {
        idx * W_SIZE..
    }
}

impl<A, const W_SIZE: usize> Deref for WindowedArray<A, W_SIZE> {
    type Target = A;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<A, const W_SIZE: usize> DerefMut for WindowedArray<A, W_SIZE> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<A, T, const W_SIZE: usize> Index<WindowIdx> for WindowedArray<A, W_SIZE>
where
    A: Index<RangeFrom<usize>, Output = [T]>,
{
    type Output = [T];

    fn index(&self, index: WindowIdx) -> &Self::Output {
        self.0.index(Self::window_range(index))
    }
}

impl<A, T, const W_SIZE: usize> IndexMut<WindowIdx> for WindowedArray<A, W_SIZE>
where
    A: IndexMut<RangeFrom<usize>, Output = [T]>,
{
    fn index_mut(&mut self, index: WindowIdx) -> &mut Self::Output {
        self.0.index_mut(Self::window_range(index))
    }
}

impl<A, T, const W_SIZE: usize> Index<usize> for WindowedArray<A, W_SIZE>
where
    A: Index<usize, Output = T>,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<A, T, const W_SIZE: usize> IndexMut<usize> for WindowedArray<A, W_SIZE>
where
    A: IndexMut<usize, Output = T>,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<A, T: ?Sized, const W_SIZE: usize> Index<RangeFrom<usize>> for WindowedArray<A, W_SIZE>
where
    A: Index<RangeFrom<usize>, Output = T>,
{
    type Output = T;

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        self.0.index(index)
    }
}

impl<A, T: ?Sized, const W_SIZE: usize> IndexMut<RangeFrom<usize>> for WindowedArray<A, W_SIZE>
where
    A: IndexMut<RangeFrom<usize>, Output = T>,
{
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Array;

    #[test]
    fn test_windowed_array() {
        let arr = WindowedArray::<_, 4>(Array([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
        assert_eq!(&arr[WindowIdx(0)], &(*arr)[0..]);
        assert_eq!(&arr[WindowIdx(1)], &(*arr)[4..]);
        assert_eq!(&arr[WindowIdx(2)], &(*arr)[8..]);
        // assert_eq!(arr[0], 0);

        // let arr = Array([0, 1, 2]);
        // let a = &arr[1..];
    }
}
