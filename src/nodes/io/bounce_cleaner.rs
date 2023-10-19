use core::error::Error;
use alloc::boxed::Box;

use crate::{nodes::{base::{NodeRef, TimedProcess}}, common_types::digital_value::DigitalValue, extensions::replace_with::TryReplace};
use crate::nodes::sampling::sample_history::{Direction, SampleHistory};
use crate::common_types::timing;

pub struct BounceCleaner<'a> {
	samples: 		 NodeRef<'a, SampleHistory<DigitalValue>>,
	output:  		 NodeRef<'a, DigitalValue>,
	last_sample: 	 usize, 
}


impl TimedProcess for BounceCleaner<'_>{
    fn next(&mut self, _current_time: &timing::Time) -> Result<(), Box<dyn Error>> {
		let samples_ref = self.samples.try_borrow()?;

        if samples_ref.full_len() > self.last_sample{
			self.output
			.try_replace({
				let high_samples_count: usize = 
					samples_ref
					.get_samples(Direction::Downcounting)
					.map(|s| match s.value{
                        DigitalValue::High => 1_usize,
                        DigitalValue::Low =>  0_usize,
					})
					.sum();

                (high_samples_count >= (samples_ref.capacity()) / 2).into() 
			})
			.unwrap();

			self.last_sample = samples_ref.full_len();
		};
        Ok(())
    }
}


