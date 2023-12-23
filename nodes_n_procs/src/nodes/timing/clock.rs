use core::borrow::BorrowMut;
use crate::base::core::node::*;
use crate::base::core::proc::*;
use crate::timing::{Duration, Time};


pub type ClockNode = Node<Time>;
pub type ClockNMut<'a> = <ClockNode as TryDerefMut>::TMut<'a>;
pub type ClockNRef<'a> = <ClockNode as TryDeref>::TRef<'a>;

pub trait TimeGetter {
    fn get_current_time(&mut self) -> Time;
}

impl<TSelf> TimeGetter for TSelf where TSelf: FnMut() -> Time{
    fn get_current_time(&mut self) -> Time {
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


impl<TTimeGetter: TimeGetter> Process for ClockProcess<TTimeGetter>
{
    type TArgs<'args>
        = ClockNMut<'args>;

    fn resume<'a>(&mut self, mut clock_reading: Self::TArgs<'a>)
    {
        let current_time = self.time_getter.borrow_mut().get_current_time();

        *clock_reading = current_time;
    }
}


pub struct DeltaTimeProcess {
    previous_time_reading: Option<Time>,
}
impl DeltaTimeProcess {
    pub fn new() -> Self{
        Self{previous_time_reading: None}
    }
}

impl Process for DeltaTimeProcess{
    type TArgs<'args>  = (ClockNRef<'args>, NodeRefMut<'args, Duration>) where Self: 'args;

    fn resume<'args>(&mut self, (clock_node, mut output): Self::TArgs<'args>) {
        let current_time_reading : &Time = clock_node.deref();
        if let Some(previous_time_reading) = &self.previous_time_reading{
            *output = (current_time_reading - previous_time_reading).to_num();
        }
        self.previous_time_reading = Some(current_time_reading.clone());
    }
}
