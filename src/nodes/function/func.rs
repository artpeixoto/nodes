pub mod fnmut_procs{
    
    

	pub mod unary_func{
		use core::{error::Error};
		use alloc::boxed::Box;
		use crate::nodes::base::NodeRef;
		use crate::nodes::base::TimedProcess;
		use crate::common_types::timing::Time;

		pub struct UnaryFunc<'a, TInput, TOutput, TFunc: FnMut(&TInput) -> TOutput> {
			pub output:  	NodeRef<'a, TOutput>,
			pub input:   	NodeRef<'a, TInput>,
			pub func: 		TFunc,
		}

		impl<TInput, TOutput, TFunc: FnMut(&TInput) -> TOutput> TimedProcess for UnaryFunc<'_, TInput, TOutput, TFunc> {
			fn next(&mut self, _current_time: &Time) -> Result<(), Box<dyn Error>>{
				let input_ref  =	 self.input.try_borrow()?;
				let mut output_ref = self.output.try_borrow_mut()?;

				*output_ref = (self.func)(&input_ref);
				
				Ok(())
		   }
		}
	}
	pub use unary_func::*;

	pub mod binary_func{

		use core::ops::Deref;

		use crate::nodes::base::{NodeRef, TimedProcess};
		use crate::common_types::timing::Time;

		pub struct BinaryFunc<'a, TInput1, TInput2, TOutput, TFunc: FnMut(&TInput1, &TInput2) -> TOutput> {
			pub output:  	NodeRef<'a, TOutput>,
			pub input_1:   	NodeRef<'a, TInput1>,
			pub input_2:   	NodeRef<'a, TInput2>,
			pub func: 		TFunc,
		}

		impl<TInput1, TInput2, TOutput, TFunc> 
			TimedProcess 
			for 	BinaryFunc<'_, TInput1, TInput2, TOutput, TFunc>
			where 	TFunc: FnMut(&TInput1, &TInput2) -> TOutput {
			fn next(&mut self, _current_time: &Time) -> Result<(), alloc::boxed::Box<dyn core::error::Error>> {
				let input_1_ref  = self.input_1.try_borrow()     ?;
				let input_2_ref  = self.input_2.try_borrow()     ?;
				let mut output_ref = self.output.try_borrow_mut()?;

				*output_ref = (self.func)(input_1_ref.deref(), input_2_ref.deref() );
				
				Ok(())
			}
		}
	}
	pub use binary_func::*;
}
pub use fnmut_procs::*;

pub mod fn_procs{

	pub mod unary_func{
		use core::{error::Error, ops::Deref};
		use alloc::boxed::Box;
		use crate::{nodes::base::{NodeRef, TimedProcess}, common_types::timing::Time, extensions::replace_with::TryReplace};


		pub struct PureUnaryFunc<'a, TInput, TOutput, TFunc> 
			where 
				TInput: PartialEq + Clone,
				TFunc: Fn(&TInput) -> TOutput 
			{
			pub input										: NodeRef<'a, TInput>,
			pub output  									: NodeRef<'a, TOutput>,
			pub func 										: TFunc,
			pub(in super::super::super::func) 	input_cache	: Option<TInput>,
		}


		impl<'a, TInput: PartialEq + Clone, TOutput, TFunc: Fn(&TInput) -> TOutput> PureUnaryFunc<'a, TInput, TOutput, TFunc> {
				pub fn new(input: NodeRef<'a, TInput>, output: NodeRef<'a, TOutput>, func: TFunc) -> Self{
					Self { 
						input,
						output, 
						func, 
						input_cache: None
					}
				}	
		}

		impl<TInput: PartialEq + Clone, TOutput, TFunc: Fn(&TInput) -> TOutput> 
			TimedProcess 
			for PureUnaryFunc<'_, TInput, TOutput, TFunc> {
			fn next(&mut self, _input: &Time) -> Result<(), Box<dyn Error>> {

				let input_value_ref = self.input.try_borrow()?;

				let should_update = match &self.input_cache{
					None 				=> true,
					Some(cache_value) 	=> input_value_ref.deref() != cache_value
				};

				if should_update {
					let new_output = (self.func)(&input_value_ref);
					if self.output.try_replace(new_output).is_ok(){
						self.input_cache = Some(input_value_ref.clone());
					}
				}

				Ok(())
			}
		}
	}
	pub use unary_func::*;

	pub mod binary_func{
		use core::{error::Error, ops::Deref};

use alloc::boxed::Box;

use crate::{nodes::{base::{NodeRef, TimedProcess}}, common_types::timing::Time, extensions::replace_with::TryReplace};
				
		pub struct PureBinaryFunc<'a, TInput1: PartialEq + Clone, TInput2: PartialEq + Clone, TOutput, TFunc: Fn(&TInput1, &TInput2) -> TOutput>{
			pub input_1			: NodeRef<'a, TInput1>,
			pub input_2			: NodeRef<'a, TInput2>,
			pub output  		: NodeRef<'a, TOutput>,
			pub func 			: TFunc,
			pub(super) input_cache			: Option<(TInput1, TInput2)>,
		}


		impl<'a, TInput1: PartialEq + Clone, TInput2: PartialEq + Clone, TOutput, TFunc: Fn(&TInput1, &TInput2) -> TOutput> PureBinaryFunc<'a, TInput1, TInput2, TOutput, TFunc> {
			pub fn new(input_1: NodeRef<'a, TInput1>, input_2: NodeRef<'a, TInput2>,output: NodeRef<'a, TOutput>, func: TFunc) -> Self{
				Self { 
					input_1, input_2,
					output, 
					func, 
					input_cache: None
				}
			}	
		}



		impl<TInput1: PartialEq + Clone, TInput2: PartialEq + Clone, TOutput, TFunc: Fn(&TInput1, &TInput2) -> TOutput> TimedProcess for PureBinaryFunc<'_, TInput1, TInput2, TOutput, TFunc> {

			fn next(&mut self, _input: &Time) -> Result<(), Box<dyn Error>>{

				let input_1_value_ref = self.input_1.try_borrow()?;
				let input_2_value_ref = self.input_2.try_borrow()?;

				let should_update = match &self.input_cache{
					None => true,
					Some((input_1, input_2)) => {
						input_1 != input_1_value_ref.deref() || input_2 != input_2_value_ref.deref()
					}
				};

				if should_update {
					let new_output = (self.func)(input_1_value_ref.deref(), input_2_value_ref.deref());
					if self.output.try_replace(new_output).is_ok(){
						self.input_cache = Some((input_1_value_ref.clone(), input_2_value_ref.clone()));
					}
				}

				Ok(())
			}
		}
	}
	pub use binary_func::*;
	
	pub mod into_pure{
		use crate::nodes::{base::TimedProcess, function::{UnaryFunc, BinaryFunc}};

		use super::{PureUnaryFunc, PureBinaryFunc};

		pub trait IntoPure: TimedProcess{
			type PureProc: TimedProcess;
			fn into_pure(self) -> Self::PureProc;
		}

		impl<'a, TInput, TOutput, TFunc>
			IntoPure 
			for UnaryFunc<'a, TInput, TOutput, TFunc> 
			where 
				TInput: PartialEq + Clone,
				TFunc:  Fn(&TInput) -> TOutput 
			{
				type PureProc = PureUnaryFunc<'a, TInput, TOutput, TFunc>;
				fn into_pure(self) -> Self::PureProc {
					PureUnaryFunc{ 
						input: 			self.input,
						output: 		self.output,
						func:   		self.func,
						input_cache: 	None
					}
				}
		}

		impl<'a, TInput1, TInput2, TOutput, TFunc>
			IntoPure 
			for BinaryFunc<'a, TInput1, TInput2, TOutput, TFunc> 
			where 
				TInput1: PartialEq + Clone,
				TInput2: PartialEq + Clone,
				TFunc: Fn(&TInput1, &TInput2) -> TOutput 
			{
				type PureProc = PureBinaryFunc<'a, TInput1, TInput2, TOutput, TFunc>;
				fn into_pure(self) -> Self::PureProc {
					PureBinaryFunc{ 
						input_1: 		self.input_1,
						input_2: 		self.input_2,
						output: 		self.output,
						func:   		self.func,
						input_cache: 	None
					}
				}
		}
	}
}
pub use fn_procs::*;
