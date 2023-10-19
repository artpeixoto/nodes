use crate::common_types::timing::{Time, Duration};

pub struct CyclesKeeper{
	pub cycle_duration: Duration,
	last_update:	 Time,
}

impl CyclesKeeper {

	pub fn new(cycle_duration: Duration) -> Self{
		Self {
			cycle_duration: cycle_duration, 
			last_update: 0 
		}
	}

	#[inline]
	pub fn get_cycles(self: &Self, current_time: &Time) -> u32{
		let res = (current_time - self.last_update ) / self.cycle_duration;
		res as u32
	}

	#[inline]
	pub fn add_cycles(self: &mut Self, cycles_count: u32){
		self.last_update += ( cycles_count as u64 )  *  self.cycle_duration;
	}


	pub fn get_cycles_and_update(self: &mut Self, current_time: &Time) -> u32{
		let cycles_passed = self.get_cycles(current_time);
		if cycles_passed > 0 { self.add_cycles(cycles_passed);}

		cycles_passed
	}
}