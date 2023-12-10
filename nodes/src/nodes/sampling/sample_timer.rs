use core::error::Error;

use alloc::boxed::Box;
use core::cell::BorrowError;
use core::ops::Deref;

use crate::{common_types::timing::Time, extensions::replace_with::TryReplace};
use crate::nodes::base::process_errors::NodeBorrowError;
use crate::nodes::base::SimpleProcess;
use crate::nodes::timing::clock::{ClockNodeRef, ClockReading};
use super::SampleNodeRef;
pub struct SampleTimer<'a, T>{
    input:              SampleNodeRef<'a, T>,
    clock_reading:      ClockNodeRef<'a>,
    output:             SampleNodeRef<'a, ClockReading>,
    has_output: bool,
}
impl<T> SimpleProcess for SampleTimer<'_, T>{
    fn next(&mut self) -> Result<(), NodeBorrowError> {
        if self.input.try_has_value()? {
            let mut output_ref_mut = self.output.try_borrow_mut()?;
            let current_reading = self.clock_reading.try_borrow()?.clone();

            *output_ref_mut = Some(current_reading);
            self.has_output = true;
        } else {
            if self.has_output {
                let mut output_ref_mut=  self.output.try_borrow_mut()?;
                *output_ref_mut = None;
                self.has_output = false;
            }
        }

        Ok(())
    }
}