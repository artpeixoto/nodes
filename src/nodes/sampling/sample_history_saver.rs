use core::{ops::Deref};
use crate::{nodes::base::{ NodeRef}};
use super::{SampleNodeRef};
use crate::nodes::sampling::sample_history::SampleHistory;
use crate::nodes::base::process_errors::NodeBorrowError;
use crate::nodes::base::SimpleProcess;


pub struct SampleHistorySaver<'a, T:Clone, const history_size: usize>{
    input:  SampleNodeRef<'a, T>,
    output: NodeRef<'a, SampleHistory<T, history_size>>,
}

impl<T:Clone, const history_size: usize>
    SimpleProcess for SampleHistorySaver<'_, T, history_size>
{
    fn next(&mut self) -> Result<(), NodeBorrowError>{
        let input_ref = self.input.try_borrow()?;

        if let Some(sample) = input_ref.deref() {
            let mut output = self.output.try_borrow_mut()?;
            output.push_sample(sample.clone());
        }
        Ok(())
    }
}

