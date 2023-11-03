use core::{fmt::Debug, error::Error};
use alloc::boxed::Box;
use crate::{common_types::timing::Time, nodes::base::{NodeRef}, extensions::replace_with::TryReplace};
use crate::nodes::base::SimpleProcess;


pub fn new_square_wave_generator (
        cycle_duration: f32,
        cycle_offset:   f32,
        amplitude:      f32,
        offset:         f32 
    ) -> impl Fn(&Time) -> f32
{
	let func = {
        let cycle_duration_us: 	u64 = ( cycle_duration * 1_000_000_f32 ) as u64;
        let cycle_offset_us: 	u64 = ( cycle_offset   * 1_000_000_f32 ) as u64;

        let up_value 		    = amplitude + offset;
        let down_value 		= offset;

        move |time: &u64| -> f32
        {
            let time_mod = (time + cycle_offset_us) % cycle_duration_us;
            let value = 
                if time_mod <= (cycle_duration_us / 2) {
                    up_value.clone().into()
                } else {
                    down_value.clone().into()
                } ;
            value
        }
    };

    func
}
pub fn new_pwm_wave_generator (
        cycle_duration: f32,
        cycle_offset:   f32,
        amplitude:      f32,
        offset:         f32,
    ) -> impl Fn(&Time) -> impl Fn(&f32, &Time) -> f32
{
    let func = {
        let cycle_duration_us: 	u64 = ( cycle_duration * 1_000_000_f32 ) as u64;
        let cycle_offset_us: 	u64 = ( cycle_offset   * 1_000_000_f32 ) as u64;

        let up_value 		    = amplitude + offset;
        let down_value 		= offset;

        move |current_val: &f32, time: &u64| -> f32 {
            let val_duration = (current_val * (cycle_duration_us as f32)) as u64;
            let duration_since_cycle_start = (time + cycle_offset_us) % cycle_duration_us;

            if duration_since_cycle_start <= val_duration {
                up_value.clone().into()
            } else {
                down_value.clone().into()
            }
        }
    };

    func
}

// pub fn new_triangle_wave_generator<TOutput: From<f32> + Debug>
// 	(output: Node<TOutput>, cycle_duration: f32, cycle_offset: f32, amplitude: f32, offset: f32 ) 
// 	-> TimeFunc<TOutput, impl Fn(Time) -> TOutput> 
// 	{
// 		todo!()
//	}