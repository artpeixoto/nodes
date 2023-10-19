use core::{error::Error};

use alloc::boxed::Box;
use crate::extensions::used_in::UsedInTrait;

use crate::common_types::timing::Time;

pub trait Process{

	type NextOutput<'output>;
	type NextInput<'input>;
	fn next<'input, 'output>(&mut self, input: Self::NextInput<'input>) -> Self::NextOutput<'output>;
}

pub trait TimedProcess {
	fn next(&mut self, current_time: &Time) -> Result<(), Box<dyn Error>>;
}


impl<TProc: TimedProcess> Process for TProc{
    type NextInput<'input> = &'input Time;
    type NextOutput<'output> = Result<(), Box<dyn Error>>;

    fn next<'input, 'output>(&mut self, input: Self::NextInput<'input>) -> Self::NextOutput<'output> {
        <Self as TimedProcess>::next(self, input)
    }
}
