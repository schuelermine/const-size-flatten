use super::ConstSizeIntoIterator;
use core::iter::Fuse;

#[derive(Clone, Debug)]
pub(crate) struct ConstSizeFlattenBase<I, U> {
    pub(crate) base_iter: Fuse<I>,
    pub(crate) front_sub_iter: Option<U>,
    pub(crate) back_sub_iter: Option<U>,
}

impl<I, U> ConstSizeFlattenBase<I, U>
where
    I: Iterator,
    I::Item: ConstSizeIntoIterator,
    U: Iterator,
{
    fn max_size(&self) -> Option<usize> {
        let base_iter_max_size = self.base_iter.size_hint().1?;
        let inner_iter_len = <I::Item as ConstSizeIntoIterator>::SIZE;
        let front_sub_iter_max_size = self
            .front_sub_iter
            .as_ref()
            .map_or(Some(0), |it| it.size_hint().1)?;
        let back_sub_iter_max_size = self
            .back_sub_iter
            .as_ref()
            .map_or(Some(0), |it| it.size_hint().1)?;
        base_iter_max_size
            .checked_mul(inner_iter_len)?
            .checked_add(front_sub_iter_max_size)?
            .checked_add(back_sub_iter_max_size)
    }
    fn min_size(&self) -> usize {
        let base_iter_min_size = self.base_iter.size_hint().0;
        let inner_iter_len = <I::Item as ConstSizeIntoIterator>::SIZE;
        let front_sub_iter_min_size = self
            .front_sub_iter
            .as_ref()
            .map_or(0, |it| it.size_hint().0);
        let back_sub_iter_min_size = self.back_sub_iter.as_ref().map_or(0, |it| it.size_hint().0);
        base_iter_min_size
            .saturating_mul(inner_iter_len)
            .saturating_add(front_sub_iter_min_size)
            .saturating_add(back_sub_iter_min_size)
    }
}

impl<I: Iterator, U> ConstSizeFlattenBase<I, U> {
    pub(crate) fn new<J: IntoIterator<IntoIter = I>>(iter: J) -> Self {
        Self {
            base_iter: iter.into_iter().fuse(),
            front_sub_iter: None,
            back_sub_iter: None,
        }
    }
}

impl<I, U> Iterator for ConstSizeFlattenBase<I, U>
where
    I: Iterator,
    I::Item: ConstSizeIntoIterator<IntoIter = U>,
    U: Iterator,
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
        (self.min_size(), self.max_size())
    }
}

impl<I, U> DoubleEndedIterator for ConstSizeFlattenBase<I, U>
where
    I: DoubleEndedIterator,
    I::Item: ConstSizeIntoIterator<IntoIter = U>,
    U: DoubleEndedIterator,
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

fn and_then_or_clear<T, U>(opt: &mut Option<T>, f: impl FnOnce(&mut T) -> Option<U>) -> Option<U> {
    let x = f(opt.as_mut()?);
    if x.is_none() {
        *opt = None;
    }
    x
}
