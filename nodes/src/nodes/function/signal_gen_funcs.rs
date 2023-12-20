use crate::timing::{Time, Duration};

pub fn new_square_wave_generator (
        cycle_duration: f32,
        cycle_offset:   f32,
        amplitude:      f32,
        offset:         f32 
    ) -> impl Fn(&Time) -> f32
{
	let func = {
        let cycle_duration:  Duration = Duration::from_num( cycle_duration  ) ;
        let cycle_offset: 	 Duration = Duration::from_num( cycle_offset    ) ;

        let up_value 		    = amplitude + offset;
        let down_value 		= offset;

        move |time: &Time| -> f32
        {
            let time_mod = ( cycle_offset.add_unsigned(time.clone())) % cycle_duration;
            let value = 
                if time_mod <= (cycle_duration / 2) {
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
    ) -> impl Fn(&f32, &Time) -> f32
{
    let cycle_duration_us: 	Duration = Duration::from_num( cycle_duration ) ;
    let cycle_offset_us: 	Duration = Duration::from_num( cycle_offset    ) ;

    let up_value 		= amplitude + offset;
    let down_value 		= offset;

    move |current_val: &f32, time: &Time| -> f32 {
        let val_duration = (current_val * cycle_duration_us.to_num::<f32>()) as u64;

        let duration_since_cycle_start = ( cycle_offset_us.add_unsigned(time.clone())) % cycle_duration_us;

        if duration_since_cycle_start <= val_duration {
            up_value.clone().into()
        } else {
            down_value.clone().into()
        }
    }
}

// pub fn new_triangle_wave_generator<TOutput: From<f32> + Debug>
// 	(output: Node<TOutput>, cycle_duration: f32, cycle_offset: f32, amplitude: f32, offset: f32 ) 
// 	-> TimeFunc<TOutput, impl Fn(Time) -> TOutput> 
// 	{
// 		todo!()
//	}