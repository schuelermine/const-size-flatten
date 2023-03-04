#![no_std]
//! To use, just `use const_size_flatten::ConstSizeFlattenIteratorExtension`.

mod flatmap;
pub use flatmap::*;
mod flatten;
pub use flatten::*;
mod flatten_base;

/// Implementors of this trait promise that all iterators they produce always produce the same number of elements.
/// This number is given by the associated constant [`SIZE`].
/// The iterator they produce must always know its remaining length (implement [`ExactSizeIterator`]).
///
/// [`SIZE`]: ConstSizeIntoIterator::SIZE
pub trait ConstSizeIntoIterator: IntoIterator
where
    Self::IntoIter: ExactSizeIterator,
{
    const SIZE: usize;
}

impl<T, const N: usize> ConstSizeIntoIterator for [T; N] {
    const SIZE: usize = N;
}

impl<T, const N: usize> ConstSizeIntoIterator for &[T; N] {
    const SIZE: usize = N;
}

impl<T, const N: usize> ConstSizeIntoIterator for &mut [T; N] {
    const SIZE: usize = N;
}

/// Convenience `trait` that allows you to construct [`ConstSizeFlatten`] and [`ConstSizeFlatMap`].
pub trait ConstSizeIteratorExtension {
    /// Construct a [`ConstSizeFlatten`] from an [`Iterator`].
    /// This is the `impl` version of [`const_size_flatten`]
    fn const_size_flatten(self) -> ConstSizeFlatten<Self>
    where
        Self: Iterator,
        Self::Item: ConstSizeIntoIterator,
        <Self::Item as IntoIterator>::IntoIter: ExactSizeIterator,
        Self: Sized,
    {
        const_size_flatten(self)
    }

    /// Construct a [`ConstSizeFlatMap`] from an [`Iterator`].
    /// This is the `impl` version of [`const_size_flat_map`]
    fn const_size_flat_map<U, F>(self, f: F) -> ConstSizeFlatMap<Self, U, F>
    where
        Self: Iterator,
        F: FnMut(Self::Item) -> U,
        U: ConstSizeIntoIterator,
        U::IntoIter: ExactSizeIterator,
        Self: Sized,
    {
        const_size_flat_map(self, f)
    }
}

impl<T> ConstSizeIteratorExtension for T {}
