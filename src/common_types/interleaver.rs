pub struct Interleaver<TItem, TIterIter>
    where
        TIterIter: Iterator,
        TIterIter::Item: Iterator<Item = TItem>
{
    iters: TIterIter,
}


impl<TItem, TIterIter> Interleaver<TItem, TIterIter>
    where
    TIterIter: Iterator,
    TIterIter::Item: Iterator<Item = TItem>
{

}


impl<TItem, TIterIter> Iterator for Interleaver<TItem, TIterIter>
    where
    TIterIter: Iterator,
    TIterIter::Item: Iterator<Item = TItem> {
    type Item = TItem;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}