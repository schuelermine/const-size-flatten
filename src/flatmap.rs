use crate::{flatten_base::ConstSizeFlattenBase, ConstSizeIntoIterator};
use core::{
    fmt::Debug,
    iter::{FusedIterator, Map},
};

/// A version of [`FlatMap`] that requires the produced [`IntoIterator`] implements [`ConstSizeIntoIterator`].
/// Notably, this `struct` implements [`ExactSizeIterator`].
///
/// [`FlatMap`]: core::iter::FlatMap
pub struct ConstSizeFlatMap<I, U, F>
where
    I: ExactSizeIterator,
    F: FnMut(I::Item) -> U,
    U: ConstSizeIntoIterator,
    U::IntoIter: ExactSizeIterator,
{
    inner: ConstSizeFlattenBase<Map<I, F>, U::IntoIter>,
}

impl<I, U, F> Clone for ConstSizeFlatMap<I, U, F>
where
    I: ExactSizeIterator + Clone,
    F: FnMut(I::Item) -> U + Clone,
    U: ConstSizeIntoIterator + Clone,
    U::IntoIter: ExactSizeIterator + Clone,
{
    fn clone(&self) -> Self {
        ConstSizeFlatMap {
            inner: self.inner.clone(),
        }
    }
}

impl<I, U, F> Debug for ConstSizeFlatMap<I, U, F>
where
    I: ExactSizeIterator + Debug,
    F: FnMut(I::Item) -> U,
    U: ConstSizeIntoIterator,
    U::IntoIter: ExactSizeIterator + Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ConstSizeFlatMap")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<I, U, F> DoubleEndedIterator for ConstSizeFlatMap<I, U, F>
where
    I: ExactSizeIterator + DoubleEndedIterator,
    F: FnMut(I::Item) -> U,
    U: ConstSizeIntoIterator,
    U::IntoIter: ExactSizeIterator + DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl<I, U, F> Iterator for ConstSizeFlatMap<I, U, F>
where
    I: ExactSizeIterator,
    F: FnMut(I::Item) -> U,
    U: ConstSizeIntoIterator,
    U::IntoIter: ExactSizeIterator,
{
    type Item = U::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<I, U, F> FusedIterator for ConstSizeFlatMap<I, U, F>
where
    I: ExactSizeIterator,
    F: FnMut(I::Item) -> U,
    U: ConstSizeIntoIterator,
    U::IntoIter: ExactSizeIterator,
{
}

impl<I, U, F> ExactSizeIterator for ConstSizeFlatMap<I, U, F>
where
    I: ExactSizeIterator,
    F: FnMut(I::Item) -> U,
    U: ConstSizeIntoIterator,
    U::IntoIter: ExactSizeIterator,
{
}
