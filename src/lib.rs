#![no_std]

mod flatmap;
pub use flatmap::ConstSizeFlatMap;
mod flatten;
pub use flatten::ConstSizeFlatten;
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
