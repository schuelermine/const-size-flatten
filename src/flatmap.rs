use crate::{flatten_base::ConstSizeFlattenBase, ConstSizeIntoIterator};
use core::{
    fmt::Debug,
    iter::{FusedIterator, Map},
};

/// A version of [`FlatMap`] that requires the produced [`IntoIterator`] implements [`ConstSizeIntoIterator`].
/// Notably, this `struct` produces accurate lower & upper bounds using [`Iterator::size_hint`].
/// Unfortunately it cannot implement [`ExactSizeIterator`] because the length may exceed [`usize::MAX`].
///
/// [`FlatMap`]: core::iter::FlatMap
pub struct ConstSizeFlatMap<I, U, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> U,
    U: ConstSizeIntoIterator,
    U::IntoIter: ExactSizeIterator,
{
    inner: ConstSizeFlattenBase<Map<I, F>, U::IntoIter>,
}

/// Construct a [`ConstSizeFlatMap`] from an [`Iterator`].
pub fn const_size_flat_map<I, U, F>(iter: I, f: F) -> ConstSizeFlatMap<I, U, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> U,
    U: ConstSizeIntoIterator,
    U::IntoIter: ExactSizeIterator,
{
    ConstSizeFlatMap {
        inner: ConstSizeFlattenBase {
            base_iter: iter.map(f).fuse(),
            front_sub_iter: None,
            back_sub_iter: None,
        },
    }
}

impl<I, U, F> Clone for ConstSizeFlatMap<I, U, F>
where
    I: Iterator + Clone,
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
    I: Iterator + Debug,
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
    I: Iterator + DoubleEndedIterator,
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
    I: Iterator,
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
    I: Iterator,
    F: FnMut(I::Item) -> U,
    U: ConstSizeIntoIterator,
    U::IntoIter: ExactSizeIterator,
{
}
