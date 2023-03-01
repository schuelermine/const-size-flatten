use super::ConstSizeIntoIterator;
use core::iter::Fuse;

#[derive(Clone, Debug)]
pub(crate) struct ConstSizeFlattenBase<I, U>
where
    I: ExactSizeIterator,
    U: ExactSizeIterator,
    <I as Iterator>::Item:
        ConstSizeIntoIterator + IntoIterator<IntoIter = U, Item = <U as Iterator>::Item>,
{
    base_iter: Fuse<I>,
    front_sub_iter: Option<U>,
    back_sub_iter: Option<U>,
}

impl<I, U> ConstSizeFlattenBase<I, U>
where
    I: ExactSizeIterator,
    U: ExactSizeIterator,
    <I as Iterator>::Item:
        ConstSizeIntoIterator + IntoIterator<IntoIter = U, Item = <U as Iterator>::Item>,
{
    fn len_opt(&self) -> Option<usize> {
        let base_len = self.base_iter.len().checked_mul(I::Item::SIZE)?;
        let front_len = self.front_sub_iter.as_ref().map_or(0, U::len);
        let back_len = self.back_sub_iter.as_ref().map_or(0, U::len);
        base_len.checked_add(front_len)?.checked_add(back_len)
    }
}

impl<I, U> Iterator for ConstSizeFlattenBase<I, U>
where
    I: ExactSizeIterator,
    U: ExactSizeIterator,
    <I as Iterator>::Item:
        ConstSizeIntoIterator + IntoIterator<IntoIter = U, Item = <U as Iterator>::Item>,
{
    type Item = <U as Iterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let next @ Some(_) = and_then_or_clear(&mut self.front_sub_iter, Iterator::next) {
                return next;
            }
            match self.base_iter.next() {
                None => return and_then_or_clear(&mut self.back_sub_iter, Iterator::next),
                Some(next) => self.front_sub_iter = Some(next.into_iter()),
            }
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len_opt = self.len_opt();
        (len_opt.unwrap_or(usize::MAX), len_opt)
    }
}

impl<I, U> DoubleEndedIterator for ConstSizeFlattenBase<I, U>
where
    I: ExactSizeIterator + DoubleEndedIterator,
    U: ExactSizeIterator + DoubleEndedIterator,
    <I as Iterator>::Item:
        ConstSizeIntoIterator + IntoIterator<IntoIter = U, Item = <U as Iterator>::Item>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let next @ Some(_) = and_then_or_clear(&mut self.back_sub_iter, |b| b.next_back()) {
                return next;
            }
            match self.base_iter.next_back() {
                None => return and_then_or_clear(&mut self.front_sub_iter, |f| f.next_back()),
                Some(next) => self.back_sub_iter = Some(next.into_iter()),
            }
        }
    }
}

impl<I, U> ExactSizeIterator for ConstSizeFlattenBase<I, U>
where
    I: ExactSizeIterator,
    U: ExactSizeIterator,
    <I as Iterator>::Item:
        ConstSizeIntoIterator + IntoIterator<IntoIter = U, Item = <U as Iterator>::Item>,
{
}

fn and_then_or_clear<T, U>(opt: &mut Option<T>, f: impl FnOnce(&mut T) -> Option<U>) -> Option<U> {
    let x = f(opt.as_mut()?);
    if x.is_none() {
        *opt = None;
    }
    x
}
