#![no_std]
//! Versions of [`FlatMap`] and [`Flatten`] that know their inner iterators’ size in advance.
//! Note that `core` & `std` already provide this functionality for some types through a hack using specialization.
//! This crate’s contribution is that the trait [`ConstSizeIntoIterator`] is public and the functionality is therefore extensible.
//!
//! To use, just `use const_size_flatten::IteratorExtension`.
//!
//! [`FlatMap`]: core::iter::FlatMap
//! [`Flatten`]: core::iter::Flatten

mod flatmap;
pub use flatmap::*;
mod flatten;
pub use flatten::*;
mod flatten_base;

/// Implementors of this trait promise that all iterators they produce always produce the same number of elements.
/// This number is given by the associated constant [`SIZE`].
/// Note that this trait should not be implemented for [`Iterator`]s, since they can be iterated through,
/// which changes the amount of elements they produce.
///
/// [`SIZE`]: ConstSizeIntoIterator::SIZE
pub trait ConstSizeIntoIterator: IntoIterator {
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

mod iterator_extension {
    pub trait Sealed: IntoIterator {}
    impl<T: IntoIterator> Sealed for T {}
}

/// Convenience `trait` that allows you to construct [`ConstSizeFlatten`] and [`ConstSizeFlatMap`].
/// This trait is sealed, you cannot implement it.
pub trait IteratorExtension: IntoIterator + iterator_extension::Sealed {
    /// Construct a [`ConstSizeFlatten`] from an [`IntoIterator`] (which includes [`Iterator`]s).
    /// This is the `impl` version of [`const_size_flatten`].
    fn const_size_flatten(self) -> ConstSizeFlatten<Self::IntoIter>
    where
        Self: Sized,
        Self::Item: IntoIterator,
    {
        const_size_flatten(self)
    }

    /// Construct a [`ConstSizeFlatMap`] from an [`IntoIterator`] (which includes [`Iterator`]s).
    /// This is the `impl` version of [`const_size_flat_map`].
    fn const_size_flat_map<U, F>(self, f: F) -> ConstSizeFlatMap<Self::IntoIter, U, F>
    where
        Self: Sized,
        U: IntoIterator,
        F: FnMut(Self::Item) -> U,
    {
        const_size_flat_map(self, f)
    }
}

impl<T: IntoIterator> IteratorExtension for T {}
