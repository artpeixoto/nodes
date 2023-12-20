use core::pin::Pin;
use core::ops::DerefMut;
use crate::base::{NodeRef, NodeRefMut, Process};
use crate::sampling::sample_history::Direction;
use crate::signals::activation_signal::ActivationSignal;
use crate::timing::{Duration, Time};
use crate::timing::periodic::cycles_keeper::CyclesKeeper;

pub mod cycles_keeper{
    use core::pin::Pin;
    use fixed::traits::Fixed;

    use crate::base::{NodeRef, NodeRefMut, Process};
    use crate::timing::{Duration, Time};

    pub struct CyclesKeeper{
        cycle_duration:         Duration,
        current_cycle:    CycleTimeRange,
    }
    struct CycleTimeRange {
        start:  Time,
        end:    Time
    }
    impl CyclesKeeper{
        pub fn new(cycle_duration: Duration) -> Self{
            Self{
                cycle_duration: cycle_duration.clone(),
                current_cycle: CycleTimeRange{
                    start: Time::from_num(0),
                    end: Time::ZERO.add_signed(  cycle_duration),
                }
            }
        }
        pub fn add_cycles(&mut self, cycle_count: i32) {
            let new_range = {
                let next_cycle_start = self.current_cycle.start.add_signed(self.cycle_duration * Duration::from_num( cycle_count));
                let next_cycle_end = next_cycle_start.add_signed(self.cycle_duration);

                CycleTimeRange{
                    start: next_cycle_start,
                    end:   next_cycle_end
                }
            };
            self.current_cycle =  new_range;
        }

        pub fn get_cycles_distance(&self, current_time: &Time) -> i32{
            if &self.current_cycle.end < current_time || (&self.current_cycle.start) > current_time {
                ((current_time - self.current_cycle.start).get_signed().unwrap() / (self.cycle_duration)).floor().to_num()
            } else {
                0
            }
        }

        pub fn update(&mut self, current_time: &Time) -> i32{
            let cycles_count = self.get_cycles_distance(current_time);
            self.add_cycles(cycles_count as i32);
            cycles_count
        }
    }

}
pub struct PeriodicCyclesProcess(pub CyclesKeeper);
impl Process for PeriodicCyclesProcess {
    type TArgs<'args> where Self: 'args = (NodeRef<'args, Time>, NodeRefMut<'args, u64>);

    fn resume<'args>(&mut self, (current_time,mut cycles_count): Self::TArgs<'args>) {
        let cycles_counted = self.0.update(&current_time);
        if cycles_counted > 0 {
            *(cycles_count.deref_mut()) += cycles_counted as u64;
        }
    }
}
pub struct PeriodicSignalSourceProc(pub CyclesKeeper);

impl Process for PeriodicSignalSourceProc{
    type TArgs<'args> where Self: 'args = (NodeRef<'args, Time>, NodeRefMut<'args, ActivationSignal>);

    fn resume<'args>(&mut self, (current_time, mut activation_signal): Self::TArgs<'args>) {
        let cycles_count = self.0.update(&current_time);
        if cycles_count > 0{
            *activation_signal = Some(());
        } else if activation_signal.is_some() {
            *activation_signal = None;
        }
    }
}