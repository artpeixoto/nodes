use alloc::string::String;
use core::{fmt::{Debug, Write}};

use crate::nodes::base::time_keeper::CyclesKeeper;
use crate::nodes::base::process_errors::NodeBorrowError;
use crate::nodes::base::SimpleProcess;
use crate::nodes::sampling::sample_history::SampleHistory;
use crate::nodes::sampling::sample_history::Direction;
use crate::nodes::sampling::sample_history::FullIndex;
use crate::nodes::timing::clock::ClockNodeRef;

use super::super::base::NodeRef;

pub struct LogWriter<'a, TWrite: Write, const msg_queue_size: usize>{
	msg_queue: 		NodeRef<'a, SampleHistory<String, msg_queue_size>>,
	last_written_index: FullIndex,
	writer: 			TWrite,
}

impl<'a, TWrite: Write, const msg_queue_size: usize>
	SimpleProcess for LogWriter<'a, TWrite, msg_queue_size>
{
    fn next(&mut self) -> Result<(), NodeBorrowError> {
		let input_queue_last_index = 
			self
			.msg_queue
			.try_borrow()?
			.get_last_full_index();

        if self.last_written_index.0 < input_queue_last_index.0 {
			self.try_write_all();	
		} 

		Ok(())
    }
}

impl<TWrite: Write, const msg_queue_size: usize> LogWriter<'_, TWrite, msg_queue_size> {
	pub fn try_write_all(&mut self){
		let input_queue_ref = 
			self.msg_queue
			.try_borrow_mut()
			.unwrap();		

		let samples = { 
				let all_samples = 
					input_queue_ref
					.get_samples(Direction::UpCounting);

				let upper_bound = all_samples.len();

				let is_end_iter = 
					(0..(upper_bound - 1))
					.map(|_i| false)
					.chain(core::iter::once(true));
				
				all_samples
				.zip(is_end_iter)
			}
			.skip_while(|(sample, _is_last)| sample.index <= self.last_written_index);

		let mut possible_new_last_written_index = None;
		for (sample, is_last) in samples {
			self.writer.write_str(&sample.value).unwrap();
			if is_last  {
				possible_new_last_written_index = Some(sample.index.clone());
			}
		}

		if let Some(new_last_written_index) = possible_new_last_written_index{
			self.last_written_index = new_last_written_index;
		}
		
	}
}

pub struct PeriodicLogger<'a, TValue, TMsgMaker, const msg_queue_size: usize>
	where 
		TMsgMaker: FnMut(&TValue) -> String 
{
	node			: NodeRef<'a, TValue>,
	clock 			: ClockNodeRef<'a>,
    msg_queue 	 	: NodeRef<'a, SampleHistory<String, msg_queue_size>>,
	msg_maker	 	: TMsgMaker,
	cycles_keeper	: CyclesKeeper,
}

impl<'a, TValue, TMsgMaker: FnMut(&TValue) -> String, const msg_queue_size: usize>
	PeriodicLogger<'a, TValue, TMsgMaker, msg_queue_size>{
	pub fn new(
	 	node: 		NodeRef<'a, TValue>,
	 	clock: 		ClockNodeRef<'a>,
	 	msg_queue: 	NodeRef<'a, SampleHistory<String, msg_queue_size>>,
	 	msg_maker: 	TMsgMaker,
	 	frequency:	f32,
	) -> Self
	{
		let cycles_keeper = {
			let cycles_duration = (1_000_000_f32 / frequency) as u64;
			CyclesKeeper::new(cycles_duration)
		};

		Self {
			node,
			clock,
			msg_queue,
			msg_maker,	
			cycles_keeper,
		}
	}
}

impl<'a, TValue, TMsgMaker, const msg_queue_size: usize>
	SimpleProcess for PeriodicLogger<'a, TValue, TMsgMaker, msg_queue_size>
	where
		TMsgMaker: FnMut(&TValue) -> String
{
    fn next(&mut self) -> Result<(), NodeBorrowError> {
		let clock_reading = self.clock.try_borrow()?;

		let should_log = 
			self.cycles_keeper.get_cycles_and_update(&clock_reading.current_time) > 0;

        if should_log { 
			let value_ref 			= self.node.try_borrow()?;
			let mut msg_queue_ref 	= self.msg_queue.try_borrow_mut()?;
			let msg 				= (self.msg_maker)(&value_ref);
			
			msg_queue_ref.push_sample(msg);
		};

        Ok(())
    }
}