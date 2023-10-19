use core::cmp::{*};
use crate::nodes::base::*;
use crate::common_types::digital_value::DigitalValue;
use crate::common_types::timing::*;


pub type StepperPosition = i64;


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum StepperStepPinStatus{
	High,
	Low,
	Awaiting,
}


#[derive(Clone, Copy)]
pub enum RotationDir{
	Clockwise, 
	CounterClockwise,
}	

#[derive(Clone,Copy)]
pub struct StepperSpeed{
	pub step_cycle_duration: u64	
}

impl StepperSpeed{
	pub fn new(speed: f32) -> Result<Self, &'static str>{
		if speed < 0.0 {
			Err("speed must be positive")
		} else if speed == 0.0 {
			Ok(StepperSpeed { step_cycle_duration: <u64>::MAX })
		} else {
			Ok( StepperSpeed{ step_cycle_duration: (1_000_000.0 / speed) as u64 })			
		}
	}
}

pub struct StepperDriver<'a>{
	pub parms: 				StepperParm,
	pub status_node: 		NodeRef<'a, StepperStatus>,
	pub step_signal_node: 	NodeRef<'a, DigitalValue>,
	pub dir_signal_node:    NodeRef<'a, DigitalValue>,
}

#[derive(Clone,  Debug )]
pub struct StepperStatus{
	pub position: 				StepperPosition,
    pub stepper_step_status: 	StepperStepPinStatus,
    pub status_last_update:     Time,
}

pub struct StepperParm {
	pub min_upstep_time_us:		 Duration,
	pub min_downstep_time_us:    Duration,
	pub microstepping_factor: 	 u8,
} 


impl Default for StepperParm{
    fn default() -> Self {
        Self { 
			min_upstep_time_us:   500_u64,
			min_downstep_time_us: 500_u64,
			microstepping_factor: 1,
		}
    }
}

impl<'a> StepperDriver<'a>{
	pub fn new(step_signal_node: NodeRef<'a, DigitalValue>, dir_signal_node: NodeRef<'a, DigitalValue>, status_node: NodeRef<'a, StepperStatus>, parms: StepperParm ) -> Self {
		StepperDriver{
			parms,
			status_node,
			step_signal_node,
			dir_signal_node
		}
	}
}

