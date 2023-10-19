use core::error::Error;

use alloc::boxed::Box;

use crate::{common_types::timing::Time, nodes::base::TimedProcess, extensions::replace_with::TryReplace};

use super::SampleNodeRef;


pub struct SampleTimer<'a, T>{
    input:  SampleNodeRef<'a, T>,
    output: SampleNodeRef<'a, Time>,
}

impl<T> TimedProcess for SampleTimer<'_, T>{
    fn next(&mut self, current_time: &Time) -> Result<(), Box<dyn Error>>{
        if self.input.try_has_value()? {
            self.output.try_replace(Some(current_time.clone()))?;                
        } else {
            if self.output.try_has_value()? {
                self.output.try_replace(None)?;
            }
        }

        Ok(())
    }
}