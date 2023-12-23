
use crate::base::{node::NodeRefMut, proc::Process };
use crate::common_types::digital_value::DigitalValue;
use crate::sampling::sample_history::{Direction, FullIndex, SampleHistoryNRef};

pub struct BounceCleaner<const sample_queue_size: usize> {
	last_full_index: 	 FullIndex,
}


impl<const SAMPLE_QUEUE_SIZE: usize>
	Process for BounceCleaner<SAMPLE_QUEUE_SIZE>
{
	type TArgs<'a>  = (SampleHistoryNRef<'a, DigitalValue, SAMPLE_QUEUE_SIZE>, NodeRefMut<'a, DigitalValue>) where Self: 'a;
    fn resume<'a>(&mut self, (sample_history, mut output) : Self::TArgs<'a>){
        if sample_history.last_full_index() > self.last_full_index{
			*output = DigitalValue::from({
				let high_samples_count: usize =
				sample_history
				.get_samples(Direction::DownCounting)
				.map(|s| match s.value {
					DigitalValue::High => 1_usize,
					DigitalValue::Low => 0_usize,
				})
				.sum();

				high_samples_count >= (SAMPLE_QUEUE_SIZE / 2)
			});

			self.last_full_index = sample_history.last_full_index();
		};
    }
}


