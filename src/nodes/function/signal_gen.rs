use core::{fmt::Debug, error::Error};
use alloc::boxed::Box;

use crate::{common_types::timing::Time, nodes::base::{NodeRef, TimedProcess}, extensions::replace_with::TryReplace};


pub struct TimeFunc<'a, TOutput, TFunc: FnMut(&Time) -> TOutput>{
	output: NodeRef<'a, TOutput>,
	func: TFunc,
}

impl<'node, TOutput, TFunc: FnMut(&Time) -> TOutput + 'node> TimeFunc<'node, TOutput, TFunc>{
	pub fn new(func: TFunc, output: NodeRef<'node, TOutput>) -> Self{
		Self{
            output,
            func
        }
	}
}

impl<TOutput, TFunc: FnMut(&Time) -> TOutput> TimedProcess for TimeFunc<'_, TOutput, TFunc>{
    fn next(&mut self, input: &Time) -> Result<(), Box<dyn Error>> {
		let res = (&mut self.func)(input);
		self.output.try_replace(res).unwrap();
		Ok(())
    }
}

pub fn new_square_wave_generator<TOutput> ( 
        output:         NodeRef<TOutput>,
        cycle_duration: f32, 
        cycle_offset:   f32,
        amplitude:      f32,
        offset:         f32 
    ) 
    ->      TimeFunc<TOutput, impl Fn(&Time) -> TOutput> 
    where   TOutput: From<f32> 
    {

	let func = {
        let cycle_duration_us: 	u64 = ( cycle_duration * 1_000_000_f32 ) as u64;
        let cycle_offset_us: 	u64 = ( cycle_offset   * 1_000_000_f32 ) as u64; 
        let up_value 				= amplitude + offset;
        let down_value 				= offset;

        move |time: &u64| -> TOutput {
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

	TimeFunc::new(func, output)
} 

// pub fn new_triangle_wave_generator<TOutput: From<f32> + Debug>
// 	(output: Node<TOutput>, cycle_duration: f32, cycle_offset: f32, amplitude: f32, offset: f32 ) 
// 	-> TimeFunc<TOutput, impl Fn(Time) -> TOutput> 
// 	{
// 		todo!()
//	}