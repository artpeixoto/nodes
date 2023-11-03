use alloc::vec::Vec;
use core::iter::from_fn;

pub struct Interleaver<TIter, TItem>
    where TIter: Iterator<Item = TItem>
{
    iters: Vec<TIter>,
}

impl<TIter, TItem> IntoIterator
    for Interleaver<TIter, TItem> where TIter: Iterator<Item = TItem> {
    type Item = TItem;
    type IntoIter = impl Iterator<Item=TItem> ;

    fn into_iter(mut self) -> Self::IntoIter {
        from_fn({
            let count = self.iters.len();
            let mut current = 0;
            move || {
                let res = unsafe{ self.iters.get_unchecked_mut(current)}.next();
                current = (current + 1) % count;
                res
            }
        })
    }
}


impl<TItem, TIter> Interleaver< TIter, TItem>
    where TIter: Iterator<Item = TItem>,
{
    pub fn new<TIterIter: Iterator<Item=TIter>>(iters: TIterIter) -> Self {
        Self{
            iters: iters.collect(),
        }
    }
}

