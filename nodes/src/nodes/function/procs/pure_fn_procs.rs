pub mod unary_func{
    use core::{error::Error, ops::Deref};
    use alloc::boxed::Box;
    use crate::{nodes::base::{NodeRef}, common_types::timing::Time, extensions::replace_with::TryReplace};
    use crate::nodes::base::process_errors::NodeBorrowError;
    use crate::nodes::base::SimpleProcess;

    pub struct PureUnaryFunc<'a, TInput, TOutput, TFunc>
        where
        TInput: PartialEq + Clone,
        TFunc: Fn(&TInput) -> TOutput
    {
        pub input										: NodeRef<'a, TInput>,
        pub output  									: NodeRef<'a, TOutput>,
        pub func 										: TFunc,
        pub(in super::super::super::procs) 	input_cache	: Option<TInput>,
    }


    impl<'a, TInput, TOutput, TFunc>
    PureUnaryFunc<'a, TInput, TOutput, TFunc>
        where
        TInput: PartialEq + Clone,
        TFunc: Fn(&TInput) -> TOutput
    {
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
    SimpleProcess
    for PureUnaryFunc<'_, TInput, TOutput, TFunc> {
        fn next(&mut self) -> Result<(), NodeBorrowError> {

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
    use crate::{nodes::{base::{NodeRef}}, common_types::timing::Time, extensions::replace_with::TryReplace};
    use crate::nodes::base::process_errors::NodeBorrowError;
    use crate::nodes::base::SimpleProcess;

    pub struct PureBinaryFunc<'a, TInput1: PartialEq + Clone, TInput2: PartialEq + Clone, TOutput, TFunc: Fn(&TInput1, &TInput2) -> TOutput>{
        pub input_1			: NodeRef<'a, TInput1>,
        pub input_2			: NodeRef<'a, TInput2>,
        pub output  		: NodeRef<'a, TOutput>,
        pub func 			: TFunc,
        pub(super) input_cache			: Option<(TInput1, TInput2)>,
    }


    impl<'a, TInput1, TInput2, TOutput, TFunc>
        PureBinaryFunc<'a, TInput1, TInput2, TOutput, TFunc>
        where
            TInput1: PartialEq + Clone,
            TInput2: PartialEq + Clone,
            TFunc: Fn(&TInput1, &TInput2) -> TOutput
    {
        pub fn new(input_1: NodeRef<'a, TInput1>, input_2: NodeRef<'a, TInput2>,output: NodeRef<'a, TOutput>, func: TFunc) -> Self{
            Self {
                input_1, input_2,
                output,
                func,
                input_cache: None
            }
        }
    }

    impl<TInput1, TInput2, TOutput, TFunc>
        SimpleProcess for PureBinaryFunc<'_, TInput1, TInput2, TOutput, TFunc>
        where
            TInput1: PartialEq + Clone,
            TInput2: PartialEq + Clone,
            TFunc: Fn(&TInput1, &TInput2) -> TOutput
    {
        fn next(&mut self) -> Result<(), NodeBorrowError>{
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
    use crate::nodes::base::SimpleProcess;
    use crate::nodes::function::fn_procs::{BinaryFunc, UnaryFunc};

    use super::{PureUnaryFunc, PureBinaryFunc};

    pub trait IntoPure: SimpleProcess{
        type PureProc: SimpleProcess;
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

    impl<'a, TInput0, TInput2, TOutput, TFunc>
        IntoPure for BinaryFunc<'a, TInput0, TInput2, TOutput, TFunc>
        where
            TInput0: PartialEq + Clone,
            TInput2: PartialEq + Clone,
            TFunc: Fn(&TInput0, &TInput2) -> TOutput
    {
        type PureProc = PureBinaryFunc<'a, TInput0, TInput2, TOutput, TFunc>;
        fn into_pure(self) -> Self::PureProc {
            PureBinaryFunc{
                input_1: 		self.input.0,
                input_2: 		self.input.1,
                output: 		self.output,
                func:   		self.func,
                input_cache: 	None
            }
        }
    }
}
pub use into_pure::*;