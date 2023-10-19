use alloc::collections::VecDeque;
pub enum Direction{
    Downcounting,
    Upcounting,
}

use crate::common_types::rolling_deque::RollingDeque;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct FullIndex(pub usize);

pub struct SampleData<T>{
    pub index:  FullIndex,	
    pub value:  T,
}

pub struct SampleHistory<T>{
    samples:          VecDeque<T>,
    full_len:         usize,
}

impl<T> SampleHistory<T>{
    pub fn new(capacity: usize) -> Self{
        SampleHistory { 
            samples: VecDeque::with_capacity(capacity),
            full_len: 0 
        }            
    }

    #[inline]
    pub fn get_first_full_index(&self) -> FullIndex{
        FullIndex(self.full_len - self.samples.len())
    }

    #[inline]
    pub fn get_last_full_index(&self) -> FullIndex{
        FullIndex(self.full_len - 1)
    }

    pub fn get_samples(&self, dir: Direction) -> impl ExactSizeIterator<Item = SampleData<&T>>  {
        (1..self.len())
        .map({
            let rev = self.len() - 1;
            move |i| {
                match dir{
                    Direction::Downcounting => rev - i ,
                    Direction::Upcounting => i 
                }
            }
        })
        .map(|i| unsafe{
            self.get(i).unwrap_unchecked()
        })
    }

    #[inline]
    pub fn full_len(&self) -> usize{
        self.full_len
    }

    #[inline]
    pub fn capacity(&self) -> usize{
        self.samples.capacity()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    pub fn get_rev(&self, index: usize) -> Option<SampleData<&T>>{
        let rev_index = {
            let last_index = self.samples.len() - 1;
            last_index.checked_sub(index)
        }?;
        
        let sample_data = self.samples.get(rev_index)?;
        
        Some( SampleData{
            index: FullIndex((self.full_len - 1) - rev_index),
            value: sample_data
        } )
    }


    pub fn get(&self, index: usize) -> Option<SampleData<&T>>{
        let sample = self.samples.get(index)?;
        let sample_full_index = FullIndex(self.get_first_full_index().0 + index);

        Some( SampleData {
            index: sample_full_index,
            value: sample
        })
    }

    pub fn push_sample(&mut self, sample: T) {
        self.samples.push_roll_forward(sample);
        self.full_len += 1;
    }
}