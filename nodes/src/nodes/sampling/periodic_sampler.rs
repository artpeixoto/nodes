use core::{error::Error, ops::Deref};

use alloc::boxed::Box;

use crate::{nodes::base::{NodeRef, time_keeper::CyclesKeeper}, common_types::timing::Duration, extensions::{replace_with::TryReplace, used_in::UsedInTrait}};
use crate::common_types::timing::Time;
use crate::nodes::base::process_errors::NodeBorrowError;
use crate::nodes::base::SimpleProcess;
use crate::nodes::timing::clock::ClockNodeRef;
use super::SampleNodeRef;

pub struct PeriodicSampler<'a, T: Clone>{
    pub input: 	    NodeRef<'a, T>,
    pub output:     SampleNodeRef<'a, T>,
    pub clock:      ClockNodeRef<'a>,

    cycles_keeper:  CyclesKeeper,
}

impl<'proc, T:Clone> PeriodicSampler<'proc, T>{
    pub fn new<'node: 'proc>
        (input: NodeRef<'node, T>,
         clock: ClockNodeRef<'node>,
         output: SampleNodeRef<'node, T>,
         period: Duration
        ) -> Self
    {
        PeriodicSampler::<'proc> { 
            input, 
            output,
            clock,
            cycles_keeper: CyclesKeeper::new(period)
        }
    }
}

impl<T:Clone> SimpleProcess for PeriodicSampler<'_, T> {
    fn next(&mut self) -> Result<(), NodeBorrowError>
    {
        let clock_reading = self.clock.try_borrow_mut()?;
        let cycles   = self.cycles_keeper.get_cycles_and_update(&clock_reading.current_time);
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
