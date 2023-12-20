use core::convert::TryFrom;
use heapless::Deque;
use crate::base::extensions::used_in::UsedInTrait;
use crate::base::{Node, NodeRef, NodeRefMut};

pub type SampleHistoryNode<T, const HISTORY_SIZE: usize> = Node<SampleHistory<T, HISTORY_SIZE>>;
pub type SampleHistoryNRef<'a, T, const HISTORY_SIZE: usize>
    = NodeRef<'a, SampleHistory<T, HISTORY_SIZE>>;
pub type SampleHistoryNMut<'a, T, const HISTORY_SIZE: usize>
    = NodeRefMut<'a, SampleHistory<T, HISTORY_SIZE>>;


pub enum Direction{
    DownCounting,
    UpCounting,
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct FullIndex(pub usize);

pub struct SampleHistoryDataPoint<T>{
    pub index:  FullIndex,	
    pub value:  T,
}

pub struct SampleHistory<T, const history_size: usize>{
    samples:          Deque<T, history_size>,
    full_len:         usize,
}

impl<T, const history_size: usize> SampleHistory<T, history_size>{
    pub fn new() -> Self {
        SampleHistory { 
            samples:    Deque::new(),
            full_len:   0,
        }            
    }

    #[inline]
    pub fn get_first_full_index(&self) -> FullIndex{
        FullIndex(self.full_len - self.samples.len())
    }

    #[inline]
    pub fn last_full_index(&self) -> FullIndex{
        FullIndex(self.full_len - 1)
    }
    pub fn get_samples<'a>(&'a self, dir: Direction) -> impl ExactSizeIterator<Item =
    SampleHistoryDataPoint<&'a T>> + 'a
    {
        let mut generator = ({
            let (mut current, step) = match dir {
                Direction::DownCounting => { (0, 1) }
                Direction::UpCounting => { (-1, -1) }
            };

            move || {
                let res = unsafe { self.get(current).unwrap_unchecked() };
                current += step;
                res
            }
        });

        (0..self.len()).map(move |_i| generator())
    }

    #[inline]
    pub fn full_len(&self) -> FullIndex{
        FullIndex(self.full_len)
    }

    #[inline]
    pub fn capacity(&self) -> usize{
        self.samples.capacity()
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.samples.len()
    }

    pub fn get(&self, index: isize) -> Option<SampleHistoryDataPoint<&T>>{
        let u_index = {
            let len = self.len() as isize;
            if index >= -len && index < len {
                (if index > 0 { index } else { len - index })
                .used_in(usize::try_from)
                .unwrap()
                .used_in(Some)
            } else {
                None
            }
        }?;

        self.internal_get(u_index)
    }

    fn internal_get(&self, mut index: usize) -> Option<SampleHistoryDataPoint<&T>>{
        let sample = {
            let slices = self.samples.as_slices();
            if index < slices.0.len() {
                Some(unsafe{slices.0.get_unchecked(index)})
            } else {
                slices.1.get(index)
            }
        }?;

        let sample_full_index = FullIndex(self.get_first_full_index().0 + index);

        Some( SampleHistoryDataPoint {
            index: sample_full_index,
            value: sample
        })
    }

    pub fn push_sample(&mut self, sample: T) {
        self.samples.push_front(sample);
        self.full_len += 1;
    }
}