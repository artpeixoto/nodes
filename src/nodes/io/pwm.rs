use alloc::borrow::ToOwned;
use core::{ops::{ Div}, error::Error};

use alloc::boxed::Box;
use core::cmp::Ordering;
use core::ops::Deref;

use crate::{nodes::base::{NodeRef}, common_types::timing::{Duration, Time}, common_types::digital_value::DigitalValue, extensions::replace_with::TryReplace};
use crate::nodes::base::process_errors::NodeBorrowError;
use crate::nodes::base::SimpleProcess;
use crate::nodes::base::time_keeper::CyclesKeeper;
use crate::nodes::timing::clock::ClockNodeRef;


pub struct PwmGenerator<'a, TInput> 
    where 
		TInput: PartialEq<TInput> + Clone + Into<f32>,
{
    pub input:  		NodeRef<'a, TInput>,
	last_input_cache: 	Option<(TInput, Duration)>,

	pub clock: 			ClockNodeRef<'a>,
	cycles_keeper: 		CyclesKeeper,

	pub output: 		NodeRef<'a, DigitalValue>,
}

impl<'a, TInput> PwmGenerator<'a, TInput>
	where
		TInput: PartialEq<TInput> + Clone + Into<f32>,
{
	fn get_duration<'b>(mut last_input_cache: &'b mut Option<(TInput, Duration)>, input: &'b TInput, cycle_duration: &'b Duration) -> &'b Duration {
		let cache_is_valid=
			last_input_cache.as_ref()
			.is_some_and(|(cached_input, dur)|{
				cached_input.eq(input)
			});

		if !cache_is_valid{
			let new_cache = {
				let new_duration = (cycle_duration.clone() as f32 * <TInput as Into<f32>>::into(input.clone())) as u64;
				let new_input = input.clone();
				(new_input, new_duration)
			};
			last_input_cache.replace(new_cache);
		}

		&last_input_cache.as_ref().unwrap().1
	}
	pub fn new(
		input: NodeRef<'a, TInput>,
		clock: ClockNodeRef<'a>,
		output: NodeRef<'a, DigitalValue>,
		frequency: f32,
	) -> Self
	{
		let cycles_keeper = {
			let cycle_duration = (1_000_000_f32 / frequency) as Duration;
			CyclesKeeper::new(cycle_duration)
		};
		Self {
			input, 
			output,
			clock,
			cycles_keeper,
			last_input_cache: None,
		}
	}
}


impl<TInput> SimpleProcess for PwmGenerator<'_, TInput>
	where
		TInput:PartialEq<TInput> + Clone + Into<f32>,
{
    fn next(&mut self) -> Result<(), NodeBorrowError> {
        let current_input 		 = 		self.input.try_borrow()?;
		let clock_reading = self.clock.try_borrow()?;
		let mut output = self.output.try_borrow_mut()?;

		let current_cycle_dur = {
			let current_time = &clock_reading.current_time;
			self.cycles_keeper.get_cycles_and_update(current_time);
			let cycle_start = self.cycles_keeper.get_current_cycle_start_time();

			current_time - cycle_start
		};

		let input_calculate_dur =
			PwmGenerator::get_duration(&mut self.last_input_cache, current_input.deref(), self.cycles_keeper.get_current_cycle_start_time() );


		match current_cycle_dur.cmp(&input_calculate_dur) {
			Ordering::Less | Ordering::Equal 	=> {*output = DigitalValue::High}
			Ordering::Greater 					=> {*output = DigitalValue::Low }
		}

		Ok(())
    }
}
