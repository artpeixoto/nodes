use crate::{nodes::{base::{NodeRef}}, common_types::digital_value::DigitalValue, extensions::replace_with::TryReplace};
use crate::nodes::base::process_errors::NodeBorrowError;
use crate::nodes::sampling::sample_history::{Direction, SampleHistory};
use crate::nodes::base::SimpleProcess;

pub struct BounceCleaner<'a, const msg_queue_size: usize> {
	samples: 		 NodeRef<'a, SampleHistory<DigitalValue, msg_queue_size>>,
	output:  		 NodeRef<'a, DigitalValue>,
	last_sample: 	 usize, 
}


impl<const msg_queue_size: usize>
	SimpleProcess for BounceCleaner<'_, msg_queue_size>
{
    fn next(&mut self) -> Result<(), NodeBorrowError> {
		let samples_ref = self.samples.try_borrow()?;

        if samples_ref.full_len() > self.last_sample{
			self.output
			.try_replace({
				let high_samples_count: usize = 
					samples_ref
					.get_samples(Direction::DownCounting)
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


