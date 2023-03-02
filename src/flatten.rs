use crate::{flatten_base::ConstSizeFlattenBase, ConstSizeIntoIterator};
use core::{fmt::Debug, iter::FusedIterator};

/// A version of [`Flatten`] that requires the produced [`IntoIterator`] implements [`ConstSizeIntoIterator`].
/// Notably, this `struct` implements [`ExactSizeIterator`].
///
/// [`Flatten`]: core::iter::Flatten
pub struct ConstSizeFlatten<I>
where
    I: ExactSizeIterator,
    I::Item: ConstSizeIntoIterator,
    <I::Item as IntoIterator>::IntoIter: ExactSizeIterator,
{
    inner: ConstSizeFlattenBase<I, <I::Item as IntoIterator>::IntoIter>,
}

impl<I> Clone for ConstSizeFlatten<I>
where
    I: ExactSizeIterator + Clone,
    I::Item: ConstSizeIntoIterator,
    <I::Item as IntoIterator>::IntoIter: ExactSizeIterator + Clone,
{
    fn clone(&self) -> Self {
        ConstSizeFlatten {
            inner: self.inner.clone(),
        }
    }
}

impl<I> Debug for ConstSizeFlatten<I>
where
    I: ExactSizeIterator + Debug,
    I::Item: ConstSizeIntoIterator,
    <I::Item as IntoIterator>::IntoIter: ExactSizeIterator + Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ConstSizeFlatten")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<I> DoubleEndedIterator for ConstSizeFlatten<I>
where
    I: ExactSizeIterator + DoubleEndedIterator,
    I::Item: ConstSizeIntoIterator,
    <I::Item as IntoIterator>::IntoIter: ExactSizeIterator + DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl<I> Iterator for ConstSizeFlatten<I>
where
    I: ExactSizeIterator,
    I::Item: ConstSizeIntoIterator,
    <I::Item as IntoIterator>::IntoIter: ExactSizeIterator,
{
    type Item = <I::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<I> FusedIterator for ConstSizeFlatten<I>
where
    I: ExactSizeIterator,
    I::Item: ConstSizeIntoIterator,
    <I::Item as IntoIterator>::IntoIter: ExactSizeIterator,
{
}

impl<I> ExactSizeIterator for ConstSizeFlatten<I>
where
    I: ExactSizeIterator,
    I::Item: ConstSizeIntoIterator,
    <I::Item as IntoIterator>::IntoIter: ExactSizeIterator,
{
}
