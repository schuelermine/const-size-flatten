use crate::{flatten_base::ConstSizeFlattenBase, ConstSizeIntoIterator};
use core::{
    fmt::Debug,
    iter::{FusedIterator, Map},
};

/// A version of [`FlatMap`] that knows its inner iterators’ size in advance,
/// and can produce accurate lower and upper bounds using [`Iterator::size_hint`].
/// This iterator does not require [`ExactSizeIterator`] for the inner iterators.
/// It can nonetheless provide an accurate length via [`Iterator::size_hint`] if they implement it.
/// This iterator does not implement [`ExactSizeIterator`], even if the inner iterators implement it.
/// This is because the nesting may cause the size to exceed [`usize::MAX`].
///
/// [`FlatMap`]: core::iter::FlatMap
pub struct ConstSizeFlatMap<I, U: IntoIterator, F> {
    inner: ConstSizeFlattenBase<Map<I, F>, U::IntoIter>,
}

impl<I, U, F> ConstSizeFlatMap<I, U, F>
where
    I: Iterator,
    U: IntoIterator,
    F: FnMut(I::Item) -> U,
{
    /// Construct a [`ConstSizeFlatMap`] from an [`IntoIterator`] (which includes [`Iterator`]s).
    fn new<J: IntoIterator<IntoIter = I>>(iter: J, f: F) -> Self {
        Self {
            inner: ConstSizeFlattenBase::new(iter.into_iter().map(f)),
        }
    }
}

/// Construct a [`ConstSizeFlatMap`] from an [`IntoIterator`] (which includes [`Iterator`]s).
pub fn const_size_flat_map<I, U, F>(iter: I, f: F) -> ConstSizeFlatMap<I::IntoIter, U, F>
where
    I: IntoIterator,
    U: IntoIterator,
    F: FnMut(I::Item) -> U,
{
    ConstSizeFlatMap::new(iter, f)
}

impl<I, U, F> Clone for ConstSizeFlatMap<I, U, F>
where
    I: Clone,
    F: Clone,
    U: IntoIterator,
    U::IntoIter: Clone,
{
    fn clone(&self) -> Self {
        ConstSizeFlatMap {
            inner: self.inner.clone(),
        }
    }
}

impl<I, U, F> Debug for ConstSizeFlatMap<I, U, F>
where
    I: Debug,
    U: IntoIterator,
    U::IntoIter: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ConstSizeFlatMap")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<I, U, F> Iterator for ConstSizeFlatMap<I, U, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> U,
    U: ConstSizeIntoIterator,
{
    type Item = U::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<I, U, F> DoubleEndedIterator for ConstSizeFlatMap<I, U, F>
where
    I: DoubleEndedIterator,
    F: FnMut(I::Item) -> U,
    U: ConstSizeIntoIterator,
    U::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl<I, U, F> FusedIterator for ConstSizeFlatMap<I, U, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> U,
    U: ConstSizeIntoIterator,
{
}
