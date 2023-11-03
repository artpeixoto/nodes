use heapless::Deque;

use crate::common_types::rolling_deque::RollingDeque;
use crate::extensions::used_in::UsedInTrait;

pub enum Direction{
    DownCounting,
    UpCounting,
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct FullIndex(pub usize);


pub struct SampleData<T>{
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
    pub fn get_last_full_index(&self) -> FullIndex{
        FullIndex(self.full_len - 1)
    }
    pub fn get_samples<'a>(&'a self, dir: Direction) -> impl ExactSizeIterator<Item =
    SampleData<&'a T>> + 'a
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
    pub fn full_len(&self) -> usize{
        self.full_len
    }

    #[inline]
    pub fn capacity(&self) -> usize{
        self.samples.capacity()
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.samples.len()
    }

    pub fn get(&self, index: isize) -> Option<SampleData<&T>>{
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

    fn internal_get(&self, index: usize) -> Option<SampleData<&T>>{
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