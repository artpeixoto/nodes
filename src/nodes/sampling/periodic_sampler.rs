use core::{error::Error, ops::Deref};

use alloc::boxed::Box;

use crate::{nodes::base::{NodeRef, TimedProcess, time_keeper::CyclesKeeper}, common_types::timing::Duration, extensions::{replace_with::TryReplace, used_in::UsedInTrait}};
use crate::common_types::timing::Time;
use super::SampleNodeRef;

pub struct PeriodicSampler<'a, T: Clone>{
    pub input: 	    NodeRef<'a, T>,
    pub output:     SampleNodeRef<'a, T>,
    cycles_keeper:  CyclesKeeper,
}

impl<'proc, T:Clone> PeriodicSampler<'proc, T>{
    pub fn new<'node: 'proc>(input: NodeRef<'node, T>, output: SampleNodeRef<'node, T>, period: Duration) -> Self {
        PeriodicSampler::<'proc> { 
            input, 
            output,
            cycles_keeper: CyclesKeeper::new(period)
        }
    }
}

impl<T:Clone> TimedProcess for PeriodicSampler<'_, T> {
    fn next(&mut self, input: &Time) -> Result<(), Box<dyn Error>>
    {
        let cycles              = self.cycles_keeper.get_cycles_and_update(&input);
        let mut output_ref_mut  = self.output.try_borrow_mut()?;

        if cycles > 0 {
            let input_value = 
                self.input
                .try_borrow()?
                .clone();

            *output_ref_mut  = Some(input_value);

        } else if output_ref_mut.is_some() {
            *output_ref_mut  = None;
        }

        Ok(())
    }
}
