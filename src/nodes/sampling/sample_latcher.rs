use alloc::boxed::Box;
use core::error::Error;
use core::ops::{Deref, DerefMut};

use crate::nodes::base::{Node, NodeRef, SimpleProcess};
use crate::nodes::base::process_errors::NodeBorrowError;
use super::{SampleNode, SampleNodeRef};

pub struct SampleLatcher<'node_ref, T:Clone> {
	pub sample_input: 	SampleNodeRef<'node_ref, T>,
	pub latch_output: 	NodeRef<'node_ref, T>
}

impl<'node_ref, T: Clone> SampleLatcher<'node_ref, T> {
	pub fn new(latch_output: &'node_ref Node<T>, sample_input: &'node_ref SampleNode<T>) -> Self{
		let latch_output = latch_output.make_ref();
		let sample_input = sample_input.make_ref();

		Self {
			sample_input,
			latch_output,
		}
	}
}

impl<'node_ref, TValue:Clone>
	SimpleProcess for SampleLatcher<'node_ref, TValue>
{
    fn next(&mut self) -> Result<(), NodeBorrowError> {
		let sample_ref = self.sample_input.try_borrow()?;

		if let Some(sample_value) = sample_ref.deref(){
			*(self.latch_output.try_borrow_mut()?.deref_mut()) = sample_value.clone();
		}

		Ok(())
    }
}