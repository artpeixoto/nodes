use core::{borrow::{Borrow}, error::Error, ops::Deref};
use alloc::boxed::Box;
use crate::{nodes::{base::node::NodeRef }, common_types::digital_value::DigitalValue, common_types::timing::{Time, Duration}};
use super::super::base::*;
use super::stepper::{StepperPosition, StepperStepPinStatus, StepperSpeed, StepperDriver};

pub struct StepperPositionController<'a> {
	pub stepper_driver		: StepperDriver<'a>,
	pub pos_setpoint_node	: NodeRef<'a, StepperPosition>,
	pub speed_node			: NodeRef<'a, StepperSpeed>,
    free_time_cache			: Option<(StepperSpeed, Duration)>,
}

impl<'a> StepperPositionController<'a>{
    pub fn new(
        stepper_driver      :  StepperDriver<'a>,
        pos_setpoint_node   :  NodeRef<'a, StepperPosition>,
        speed_node          :  NodeRef<'a, StepperSpeed>    
    ) -> Self { 
        Self{
            stepper_driver,
            pos_setpoint_node,
            speed_node,
            free_time_cache: None
        }
    }
}

impl TimedProcess for StepperPositionController<'_> {
    fn next(&mut self, input: &Time) -> Result<(), Box<dyn Error>>{
		let stepper_parms 	= &self.stepper_driver.parms;
		let mut status_node	= self.stepper_driver.status_node.try_borrow_mut()?;
		let speed_node 		= self.speed_node.try_borrow()?;

        let finish_time = 
            status_node.status_last_update + 
            match status_node.stepper_step_status.borrow(){
                StepperStepPinStatus::High 		=> 	stepper_parms.min_upstep_time_us,
                StepperStepPinStatus::Low 		=>	stepper_parms.min_downstep_time_us,
                StepperStepPinStatus::Awaiting 	=>	{speed_node.step_cycle_duration - (stepper_parms.min_upstep_time_us + stepper_parms.min_downstep_time_us)},
            
            };

        if !(&finish_time > input) {
            use StepperStepPinStatus::*;
            let mut step_node = self.stepper_driver.step_signal_node.try_borrow_mut()?;
            let mut dir_node  = self.stepper_driver.dir_signal_node .try_borrow_mut()?;

            match &status_node.stepper_step_status{
                High => {
                    *step_node = DigitalValue::Low;
                    status_node.status_last_update  = finish_time;
                    status_node.stepper_step_status = Low;
                },
                Low	=> {
                    status_node.status_last_update = finish_time;
                    status_node.position += match dir_node.deref() {
                            DigitalValue::Low  => 1,
                            DigitalValue::High => -1
                        };
                    status_node.stepper_step_status = Awaiting;
                },
                Awaiting => {
                    let setpoint_pos_ref = self.pos_setpoint_node.try_borrow().unwrap();
                    let current_pos  = &status_node.position;

                    if current_pos == setpoint_pos_ref.deref() {
                        status_node.stepper_step_status = Awaiting; 
                    } else {
                        *dir_node = if &status_node.position < setpoint_pos_ref.deref() {
                            DigitalValue::Low
                        } else {
                            DigitalValue::High	
                        };
                        *step_node 						= DigitalValue::High;
                        status_node.status_last_update 	= finish_time;
                        status_node.stepper_step_status = High;
                    }
                },
            };
        } ;

		Ok(())
	}
}



