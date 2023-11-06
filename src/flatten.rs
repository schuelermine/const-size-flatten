use crate::{flatten_base::ConstSizeFlattenBase, ConstSizeIntoIterator};
use core::{fmt::Debug, iter::FusedIterator};

/// A version of [`Flatten`] that knows its inner iterators’ size in advance,
/// and can produce accurate lower and upper bounds using [`Iterator::size_hint`].
/// This iterator does not require [`ExactSizeIterator`] for the inner iterators.
/// It can nonetheless provide an accurate length via [`Iterator::size_hint`] if they implement it.
/// This iterator does not implement [`ExactSizeIterator`], even if the inner iterators implement it.
/// This is because the nesting may cause the size to exceed [`usize::MAX`].
///
/// [`Flatten`]: core::iter::Flatten
pub struct ConstSizeFlatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    inner: ConstSizeFlattenBase<I, <I::Item as IntoIterator>::IntoIter>,
}

impl<I> ConstSizeFlatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    /// Construct a [`ConstSizeFlatten`] from an [`IntoIterator`] (which includes [`Iterator`]s).
    fn new<U: IntoIterator<IntoIter = I>>(iter: U) -> Self {
        Self {
            inner: ConstSizeFlattenBase::new(iter),
        }
    }
}

/// Construct a [`ConstSizeFlatten`] from an [`IntoIterator`] (which includes [`Iterator`]s).
pub fn const_size_flatten<I>(iter: I) -> ConstSizeFlatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    ConstSizeFlatten::new(iter)
}

impl<I> Clone for ConstSizeFlatten<I>
where
    I: Iterator + Clone,
    I::Item: IntoIterator,
    <I::Item as IntoIterator>::IntoIter: Clone,
{
    fn clone(&self) -> Self {
        ConstSizeFlatten {
            inner: self.inner.clone(),
        }
    }
}

impl<I> Debug for ConstSizeFlatten<I>
where
    I: Iterator + Debug,
    I::Item: IntoIterator,
    <I::Item as IntoIterator>::IntoIter: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ConstSizeFlatten")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<I> Iterator for ConstSizeFlatten<I>
where
    I: Iterator,
    I::Item: ConstSizeIntoIterator,
{
    type Item = <I::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<I> DoubleEndedIterator for ConstSizeFlatten<I>
where
    I: DoubleEndedIterator,
    I::Item: ConstSizeIntoIterator,
    <I::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl<I> FusedIterator for ConstSizeFlatten<I>
where
    I: Iterator,
    I::Item: ConstSizeIntoIterator,
{
}
