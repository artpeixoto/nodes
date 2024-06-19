
use core::marker::PhantomData;
use core::ops::AddAssign;
use core::ops::DerefMut;
use crate::base::core::node::*;
use crate::base::core::proc::*;

use crate::signals::activation_signal::ActivationSignal;
use crate::timing::{Instant, Duration};
use crate::timing::periodic::cycles_keeper::CyclesKeeper;

pub mod cycles_keeper{
    use fixed::traits::{Fixed, FixedSigned};
    use crate::timing::{Duration, Instant};

    pub struct CyclesKeeper{
        cycle_duration:     Duration,
        current_cycle:      CycleTimeRange,
    }
    struct CycleTimeRange {
        start:  Instant,
        end:    Instant
    }
    impl CyclesKeeper{
        pub fn new(cycle_duration: Duration) -> Self{
            if cycle_duration.is_zero() {
                panic!("cycle_duration cannot be zero");
            }
            Self{
                cycle_duration: cycle_duration.clone(),
                current_cycle: CycleTimeRange{
                    start: Instant::from_num(0),
                    end: Instant::ZERO.add_signed(cycle_duration),
                }
            }
        }
        pub fn add_cycles(&mut self, cycle_count: i32) {
            if cycle_count == 0{ return }
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

        
        pub fn get_cycles_distance(&self, current_time: &Instant) -> i32{
            if current_time > &self.current_cycle.end {
                ((current_time.unwrapped_sub(self.current_cycle.start.clone())) / (self.cycle_duration.unsigned_abs())).floor().to_num()
            } else if current_time < &self.current_cycle.start {
                ((self.current_cycle.start.unwrapped_sub(current_time.clone())) / (self.cycle_duration.unsigned_abs())).floor().to_num::<i32>() * (-1_i32)
            } else {
                0
            }
        }
        pub fn update_cycle_duration(&mut self, new_duration: &Duration){
            if &self.cycle_duration != new_duration{
                self.current_cycle.end = self.current_cycle.start.add_signed(new_duration.clone()); 
                self.cycle_duration = *new_duration;
            }
        }
        pub fn update(&mut self, current_time: &Instant) -> i32{
            let cycles_count = self.get_cycles_distance(current_time);
            self.add_cycles(cycles_count as i32);
            cycles_count
        }
    }

}
pub struct PeriodicCyclesProcess<TOut>
    where TOut: From<i32> + AddAssign
{
    pub cycles_keeper: CyclesKeeper,
    __phantom: PhantomData<TOut>
}

impl<TOut> PeriodicCyclesProcess<TOut>
where TOut: From<i32> + AddAssign
{
    pub fn new_from_cycle_duration(cycle_duration: impl Into<Duration>) -> Self {
        let cycles_keeper = CyclesKeeper::new(cycle_duration.into());
        Self { cycles_keeper, __phantom: PhantomData }
    }
    pub fn new_from_keeper(cycles_keeper: CyclesKeeper) -> Self {
        Self { cycles_keeper, __phantom: PhantomData }
    }
}

impl<'a, TOut> Process<'a> for PeriodicCyclesProcess<TOut> 
where 
    TOut: From<i32> + AddAssign + 'a
{
    type TArgs  = (NodeRef<'a, Instant>, NodeRefMut<'a, TOut>);

    fn resume(&mut self, (current_time,mut cycles_count): Self::TArgs) {
        let cycles_counted = self.cycles_keeper.update(&current_time);
        if cycles_counted > 0 {
            (cycles_count.deref_mut()).add_assign(cycles_counted.into());
        }
    }
}
pub struct PeriodicImpulseProc(pub CyclesKeeper);

impl<'a> Process<'a> for PeriodicImpulseProc{
    type TArgs  = (NodeRef<'a, Duration>, NodeRef<'a, Instant>,  NodeRefMut<'a, ActivationSignal>);

    fn resume(&mut self, (duration, current_time, mut activation_signal): Self::TArgs) {
        self.0.update_cycle_duration(duration.deref());
        let cycles_count = self.0.update(&current_time);
        if cycles_count > 0{
            *activation_signal = Some(());
        } else if activation_signal.is_some() {
            *activation_signal = None;
        }
    }
}