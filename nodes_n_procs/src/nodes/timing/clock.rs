use core::borrow::BorrowMut;
use crate::base::core::node::*;
use crate::base::core::proc::*;
use crate::timing::{Duration, Instant};


pub type ClockNode = Node<Instant>;
pub type ClockNMut<'a> = <ClockNode as TryDerefMut>::TMut<'a>;
pub type ClockNRef<'a> = <ClockNode as TryDeref>::TRef<'a>;

pub trait TimeGetter {
    fn get_current_time(&mut self) -> Instant;
}

impl<TSelf> TimeGetter for TSelf where TSelf: FnMut() -> Instant{
    fn get_current_time(&mut self) -> Instant {
        (self)()
    }
}

pub struct ClockProcess< TTimeGetter: TimeGetter> {
    time_getter:       TTimeGetter,
}

impl< TTimeGetter: TimeGetter> ClockProcess< TTimeGetter>
{
    pub fn new(time_getter: TTimeGetter) -> Self {
        Self{
            time_getter,
        }
    }
}


impl<'a, TTimeGetter: TimeGetter> Process<'a> for ClockProcess<TTimeGetter>
{
    type TArgs
        = ClockNMut<'a>;

    fn resume(&mut self, mut clock_reading: Self::TArgs)
    {
        let current_time = self.time_getter.borrow_mut().get_current_time();

        *clock_reading = current_time;
    }
}


pub struct DeltaTimeProcess {
    previous_time_reading: Option<Instant>,
}
impl DeltaTimeProcess {
    pub fn new() -> Self{
        Self{previous_time_reading: None}
    }
}

impl<'a> Process<'a> for DeltaTimeProcess{
    type TArgs  = (ClockNRef<'a>, NodeRefMut<'a, Duration>);

    fn resume(&mut self, (clock_node, mut output): Self::TArgs) {
        let current_time_reading : &Instant = clock_node.deref();
        if let Some(previous_time_reading) = &self.previous_time_reading{
            *output = (current_time_reading - previous_time_reading).to_num();
        }
        self.previous_time_reading = Some(current_time_reading.clone());
    }
}
