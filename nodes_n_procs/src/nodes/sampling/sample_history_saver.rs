use core::marker::PhantomData;
use core::ops::Deref;

use crate::base::proc::Process;
use crate::sampling::sample_node::SampleNRef;

use super::SampleHistoryNMut;

pub struct SampleHistorySaver<T:Clone, const history_size: usize>(PhantomData<T>);

impl<'a, T:Clone, const history_size: usize>
    Process<'a> for SampleHistorySaver<T, history_size>
    where 
         T: 'a
{
    type TArgs  = (SampleNRef<'a, T>, SampleHistoryNMut<'a, T, history_size>);
    fn resume(&mut self, (sample, mut sample_history): Self::TArgs){
        if let Some(sample) = sample.deref(){
            sample_history.push_sample(sample.clone());
        }
    }
}

