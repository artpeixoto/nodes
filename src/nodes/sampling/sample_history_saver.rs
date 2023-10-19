use core::{error::Error, ops::{Deref}};
use alloc::boxed::Box;
use crate::{nodes::base::{TimedProcess, NodeRef}};
use super::{SampleNodeRef};
use crate::nodes::sampling::sample_history::SampleHistory;
use crate::common_types::timing::Time;


pub struct SampleHistorySaver<'a, T:Clone>{
    input:  SampleNodeRef<'a, T>,
    output: NodeRef<'a, SampleHistory<T>>,
}

impl<T:Clone> TimedProcess for SampleHistorySaver<'_, T>{
    fn next(&mut self, _current_time: &Time) -> Result<(), Box<dyn Error>>{
        let input_ref = self.input.try_borrow()?;
        if let Some(sample) = input_ref.deref(){
            let mut output = self.output.try_borrow_mut()?;
            output.push_sample(sample.clone());
        }

        Ok(())
    }

}

