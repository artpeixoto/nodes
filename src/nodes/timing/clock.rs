use core::cell::BorrowMutError;
use crate::common_types::timing::{Duration, Time};
use crate::nodes::base::{Node, NodeRef};

#[derive(Clone, PartialEq, Eq)]
pub struct ClockReading {
    pub current_time: Time,
    pub delta_time:   Duration,
}

pub type ClockNode = Node<ClockReading>;
pub type ClockNodeRef<'a> = NodeRef<'a, ClockReading>;
pub trait TimeGetter {
    fn get_current_time(&mut self) -> Time;
}

impl<TSelf> TimeGetter for TSelf where TSelf: FnMut() -> Duration{
    fn get_current_time(&mut self) -> Time {
        (self)()
    }
}

pub struct ClockProcess<'a, TTimeGetter: TimeGetter> {
    time_getter:       TTimeGetter,
    clock_node_ref:    ClockNodeRef<'a>,
}

impl<'a, TTimeGetter: TimeGetter>
    ClockProcess<'a, TTimeGetter>
{
    pub fn new(time_getter: TTimeGetter, clock_node: &'a ClockNode) -> Self {
        let clock_node_ref = clock_node.make_ref();

        Self{
            time_getter,
            clock_node_ref
        }
    }

    pub fn try_update_clock_node(&mut self) -> Result<(), BorrowMutError>{
        let mut clock_reading = self.clock_node_ref.try_borrow_mut()?;
        let current_time = self.time_getter.get_current_time();
        let delta_time = current_time - clock_reading.current_time;

        clock_reading.current_time = current_time;
        clock_reading.delta_time = delta_time;

        Ok(())
    }
}





