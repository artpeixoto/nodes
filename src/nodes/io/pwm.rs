use core::{ops::{Deref, Div}, error::Error};

use alloc::boxed::Box;

use crate::{nodes::base::{NodeRef, TimedProcess}, common_types::timing::{Duration, Time}, common_types::digital_value::DigitalValue, extensions::replace_with::TryReplace};


pub struct PwmGenerator<'a, TInput> 
    where 
		TInput: Div<TInput> + PartialEq<TInput> ,
		<TInput as Div>::Output: Into<f32>
	{
    pub input:  		NodeRef<'a, TInput>,
	pub output: 		NodeRef<'a, DigitalValue>,
    pub cycle_duration: Duration,

    input_max: 		TInput,
	last_input_cache: Option<(TInput, Duration)>
}

impl<'a, TInput> PwmGenerator<'a, TInput>
	where
		TInput: Div<TInput> + PartialEq<TInput>,
		<TInput as Div>::Output: 	Into<f32>
	{
	pub fn new(
		input: NodeRef<'a, TInput>,
		output: NodeRef<'a, DigitalValue>, 
		input_max: TInput,
		frequency: f32
	) -> Self {
		let cycle_duration = (1_000_000_f32 / frequency) as Duration;
		Self { 
			input, 
			output, 
			cycle_duration,  
            input_max,
			last_input_cache: None,
		}			
	}
}


impl<TInput> TimedProcess for PwmGenerator<'_, TInput>
	where
		TInput: Div<TInput> + PartialEq<TInput> + Clone,
		<TInput as Div>::Output: Into<f32>
	{

    fn next(&mut self, time: &Time) -> Result<(), Box<dyn Error>> {
        let current_input = self.input.try_borrow().unwrap().deref().clone();  
		let calculated_duration =
		 	match &self.last_input_cache {
                Some((old_input, old_calculated_duration)) => {
                    if old_input == &current_input {
                        Some(old_calculated_duration.clone())
                    } else {
                        None
                    }								
                },
                None =>  None,
            } 
            .unwrap_or_else(||{
                let ratio : f32 = 	(current_input.clone() / self.input_max.clone()).into() ;
				let calculated_duration = 	(ratio * (self.cycle_duration as f32))  as u64;
                self.last_input_cache = Some((current_input.clone(), calculated_duration));
                calculated_duration
				}
            );
        
		let mod_time = time % self.cycle_duration;

		let (current_value, _wait_time) = if mod_time <= calculated_duration {
			(DigitalValue::High, calculated_duration - mod_time)
		} else {
			(DigitalValue::Low, self.cycle_duration - mod_time)
		};

        self.output.try_replace(current_value).unwrap();

		Ok(())
    }
}
