use core::marker::PhantomData;
use core::ops::Deref;

use crate::base::Process;

use crate::sampling::sample_node::SampleNRef;

use super::SampleHistoryNMut;

pub struct SampleHistorySaver<T:Clone, const history_size: usize>(PhantomData<T>);

impl<T:Clone, const history_size: usize>
    Process for SampleHistorySaver<T, history_size>
    where 
        for<'a>  T: 'a
{
    type TArgs<'a>  = (SampleNRef<'a, T>, SampleHistoryNMut<'a, T, history_size>);
    fn resume<'a>(&mut self, (sample, mut sample_history): Self::TArgs<'a>){
        if let Some(sample) = sample.deref(){
            sample_history.push_sample(sample.clone());
        }
    }
}

