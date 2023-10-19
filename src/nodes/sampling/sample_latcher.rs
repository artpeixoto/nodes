use core::ops::{Deref, DerefMut};

use crate::nodes::base::{NodeRef, TimedProcess};
use super::SampleNodeRef;

pub struct SampleLatcher<'node_ref, T:Clone> {
	pub sample_node: 	SampleNodeRef<'node_ref, T>,
	pub latch_node: 	NodeRef<'node_ref, T>
}
impl<'node_ref, TValue:Clone> TimedProcess for SampleLatcher<'node_ref, TValue>{
    fn next(&mut self, current_time: &crate::common_types::timing::Time) -> Result<(), alloc::boxed::Box<dyn core::error::Error>> {
		let sample_ref = self.sample_node.try_borrow()?;

		if let Some(sample_value) = sample_ref.deref(){
			*(self.latch_node.try_borrow_mut()?.deref_mut()) = sample_value.clone();
		}

		Ok(())
    }
}