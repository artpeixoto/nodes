
use core::marker::PhantomData;
use core::ops::{Deref};

use crate::base::node::*;
use crate::base::proc::*;
use crate::sampling::sample_node::SampleNRef;

pub struct SampleLatcher<T:Clone>(PhantomData<T>);

impl< T: Clone> SampleLatcher<T> {
	pub fn new() -> Self{
		Self(PhantomData{})
	}
}

impl<'a, TValue:Clone> Process<'a> for SampleLatcher<TValue> where TValue: 'a
{
	type TArgs = (SampleNRef<'a, TValue>, NodeRefMut<'a, TValue>);
    fn resume(&mut self, (sample_input,mut latch_output): Self::TArgs) {
		if let Some(sample_value) = sample_input.deref(){
			*latch_output = sample_value.clone()
		}
    }
}