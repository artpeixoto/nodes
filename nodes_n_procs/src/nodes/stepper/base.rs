use core::cell::RefMut;
use core::cmp::{*};
use core::ops::{Deref, DerefMut};
use anyhow::Error;
use either::Either;
use crate::nodes::base::*;
use crate::common_types::digital_value::DigitalValue;
use crate::common_types::timing::*;
use crate::nodes::base::process_errors::NodeBorrowError;
use crate::nodes::timing::clock::{ClockNodeRef, ClockReading};

pub struct StepperPosition(
	pub i64,
);

pub struct StageTimeInfo {
	pub current_stage_start: Time,
	pub current_stage_end:   Option<Time>,
}

pub enum StepperStepCycleInCycleStage{
	High,
	Low,
}
pub struct Finished;

pub struct StepperStepCycleInCycleStatus{
	state: StepperStepCycleInCycleStage,
	until: Time,
}

pub struct StepperStepCycleInCycleParms{
	high_duration: Duration,
	low_duration:  Duration,
}

#[derive(Clone, PartialEq)]
pub enum StepperStepCycleState {
	InCycle(StepperStepCycleInCycleStatus),
	Awaiting,
}

pub fn update_stepper_step_cycle_status(stepper_step_cycle_status: &mut StepperStepCycleStatus, stepper_step_cycle_in_cycle_parms: &StepperStepCycleInCycleParms, current_time: &Time) {
	let new_stepper_step_cycle_status = {
		match &stepper_step_cycle_status.stage		{
			StepperStepCycleState::InCycle(in_cycle_status) => {
				if &in_cycle_status.until <= current_time{
					match in_cycle_status.state{
						StepperStepCycleInCycleStage::High => {
							Some(StepperStepCycleStatus{
								stage:  StepperStepCycleState::InCycle(
									StepperStepCycleInCycleStatus{
										state: ,
										until: 0,
									}
								),
								time_info: ,
							})
						}
						StepperStepCycleInCycleStage::Low => {}
					}
				}
			}
			StepperStepCycleState::Awaiting => {}
		}
	}
}

pub struct StepperStepCycleStatus {
	stage: StepperStepCycleState,
	time_info: StageTimeInfo,
}

#[derive(Clone, Copy)]
pub enum RotationDir{
	Clockwise, 
	CounterClockwise,
}

pub struct StepperDriver<'a>{
	stepper_data :	NodeRef<'a, StepperData>,
	clock: 			ClockNodeRef<'a>,
}

impl<'driver> StepperDriver<'driver>{
	fn try_get_write_lock<'driver_ref>(&'driver_ref mut self) -> Result<StepperDriverWriteLock<'driver_ref, 'driver>, NodeBorrowError>{
		let status_node_ref = self.stepper_status.try_borrow_mut()?;
		let parms_ref = &self.parms;

		Ok(StepperDriverWriteLock{
			parms_ref,
			status_node_ref,
		})
	}
}

pub struct StepperDriverWriteLock<'driver_ref, 'driver, ParmsRef, StatusNodeRefMut, ClockReadingRef>
	where
		'driver: 'driver_ref,
		ParmsRef: Deref<Target=StepperParm> + 'driver_ref,
		StatusNodeRefMut: DerefMut<Target=StepperStatus> + 'driver_ref,
		ClockReadingRef: Deref<Target=ClockReading> + 'driver_ref,
{
	parms_ref: 			ParmsRef,
	status_node_ref: 	StatusNodeRefMut,
	clock_reading_ref:	ClockReadingRef
}

impl<'driver_ref, 'driver, ParmsRef, StatusNodeRefMut, ClockReadingRef>
	StepperDriverWriteLock<
		'driver_ref,
		'driver,
		ParmsRef,
		StatusNodeRefMut,
		ClockReadingRef
	>
	where
		'driver: 'driver_ref,
		ParmsRef: Deref<Target = StepperParm> + 'driver_ref,
		StatusNodeRefMut: DerefMut<Target = StepperStatus> + 'driver_ref,
		ClockReadingRef: Deref<Target = ClockReading> + 'driver_ref,
{
	fn update(&mut self){
		fn update_step(stepper_parms: &StepperParm, stepper_status: &mut StepperStatus, current_time: &Time){
			let new_status = {
				match &stepper_status.stepper_step_status {
					StepperStepCycleStatus::InCycle{stage: cycle_stage,  cycle_stage_time_info: cycle_stage_time_info} => {
						if (&cycle_stage_time_info.current_stage_end <= current_time) {
							Some(StepperStepPinStatus::Low( StageTimeInfo {
								cycle_start: step_cycle_time_info.cycle_start,
								current_stage_start: current_time.clone(),
								current_stage_end: current_time + stepper_parms.min_downstep_duration,
							}))
						} else {
							None
						}
					}
					StepperStepPinStatus::Low(step_cycle_time_info) => {
						if (&step_cycle_time_info.current_stage_end <= current_time) {
							Some(StepperStepPinStatus::Awaiting)
						} else {
							None
						}

					}
					StepperStepPinStatus::Awaiting => {None}
				}
			};

			if let Some(new_status) = new_status{
				*stepper_status.stepper_step_status = new_status;
			}
		}

		let current_time = &self.clock_reading_ref.current_time;
		update_step(self.parms_ref.deref(), self.status_node_ref.deref_mut(), current_time);

	}


	pub fn can_do_stuff(&self) -> bool{
		use StepperStepPinStatus::*;
		match &self.status_node_ref.stepper_step_status {
			High(_) | Low(_) 	=> {false},
			Awaiting 			=> {true}
		}
	}
}


impl StepperDriver<'_>{

	pub fn try_set_direction(&mut self, dir: RotationDir){

	}
	pub fn step(&mut self) -> Result<(), NodeBorrowError>{
		let mut status_node = self.stepper_status.try_borrow_mut()?;

		let can_step = {
			match status_node.stepper_step_status {
				StepperStepPinStatus::High(_) | StepperStepPinStatus::Low(_) => { false }
				StepperStepPinStatus::Awaiting => { true }
			}
		};

		fn add_step(stepper_pos: &mut StepperPosition, direction: &RotationDir, parms: &StepperParm){
			stepper_pos
		}

		if can_step {
			self.
		}

		Ok(())
	}
	pub fn update(&mut self, current_time: &Time) -> Result<(), NodeBorrowError> {
		let mut status_node = self.stepper_status.try_borrow_mut()?;

		fn update_step_cycle(){
			if current
		}
	}
}

pub struct StepperData{
	stepper_status: 	StepperStatus,
	stepper_parameters: StepperParm,
}

#[derive(Clone)]
pub struct StepperStatus{
	pub stepper_direction: 		RotationDir,
    pub stepper_step_status: StepperStepCycleStatus,
}

pub struct StepperStatusToIOWriter<'a> {
	pub status_node: 		NodeRef<'a, StepperStatus>,
	pub step_signal_node: 	NodeRef<'a, DigitalValue>,
	pub direction_signal:   NodeRef<'a, DigitalValue>,
}

pub struct StepperParm {
	pub min_upstep_duration:     Duration,
	pub min_downstep_duration:   Duration,
	pub steps_per_rev: 	 		 u16,
}


impl Default for StepperParm{
    fn default() -> Self {
        Self { 
			min_upstep_duration:   	500_u64,
			min_downstep_duration: 	500_u64,
			steps_per_rev: 			200_u16,
		}
    }
}

impl<'a> StepperDriver<'a>{
	pub fn new(
		step_signal_node: 	NodeRef<'a, DigitalValue>,
		dir_signal_node: 	NodeRef<'a, DigitalValue>,
		status_node: 		NodeRef<'a, StepperStatus>,
		parms: 				StepperParm
	) -> Self {
		StepperDriver {
			parms,
			stepper_status: status_node,
			step_signal_node,
			dir_signal_node
		}
	}
}

