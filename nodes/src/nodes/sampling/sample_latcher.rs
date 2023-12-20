use alloc::boxed::Box;
use core::error::Error;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use core::pin::Pin;
use crate::base::{NodeRefMut, Process};
use crate::sampling::sample_node::SampleNRef;

pub struct SampleLatcher<T:Clone>(PhantomData<T>);

impl< T: Clone> SampleLatcher<T> {
	pub fn new() -> Self{
		Self(PhantomData{})
	}
}

impl<TValue:Clone> Process for SampleLatcher<TValue> where for<'a> TValue: 'a
{
	type TArgs<'a> = (SampleNRef<'a, TValue>, NodeRefMut<'a, TValue>);
    fn resume<'a>(&mut self, (sample_input,mut latch_output): Self::TArgs<'a>) {
		if let Some(sample_value) = sample_input.deref(){
			*latch_output = sample_value.clone()
		}
    }
}