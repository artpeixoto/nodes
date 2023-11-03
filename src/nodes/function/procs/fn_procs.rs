
pub mod unary_func{
    use core::{error::Error};
    use alloc::boxed::Box;
    use crate::nodes::base::{NodeRef, SimpleProcess};
    use crate::nodes::base::process_errors::NodeBorrowError;

    pub struct UnaryFunc<'a, TInput, TOutput, TFunc>
        where TFunc: FnMut(&TInput) -> TOutput
    {
        pub output:  	NodeRef<'a, TOutput>,
        pub input:   	NodeRef<'a, TInput>,
        pub func: 		TFunc,
    }

    impl<TInput, TOutput, TFunc>
    SimpleProcess for UnaryFunc<'_, TInput, TOutput, TFunc>
        where TFunc: FnMut(&TInput) -> TOutput
    {
        fn next(&mut self) -> Result<(), NodeBorrowError>{
            let input_ref  =	 self.input.try_borrow()?;
            let mut output_ref = self.output.try_borrow_mut()?;

            *output_ref = (self.func)(&input_ref);

            Ok(())
        }
    }
}
pub use unary_func::*;

pub mod binary_func {
    use alloc::boxed;
    use core::ops::Deref;
    use boxed::Box;
    use core::error::Error;
    use crate::nodes::base::{NodeRef, SimpleProcess};
    use crate::nodes::base::process_errors::NodeBorrowError;

    pub struct BinaryFunc<'a, TInput0, TInput1, TOutput, TFunc: FnMut(&TInput0, &TInput1) -> TOutput>
    {
        pub input:   	(NodeRef<'a, TInput0>, NodeRef<'a, TInput1>),
        pub func: 		TFunc,
        pub output:  	NodeRef<'a, TOutput>,
    }


    impl<TInput0, TInput1, TOutput, TFunc>
        SimpleProcess for BinaryFunc<'_, TInput0, TInput1, TOutput, TFunc>
        where 	TFunc: FnMut(&TInput0, &TInput1) -> TOutput
    {
        fn next(&mut self) -> Result<(), NodeBorrowError> {
            let mut output_ref = self.output.try_borrow_mut()?;
            let input_0_ref  = self.input.0.try_borrow()?;
            let input_1_ref  = self.input.1.try_borrow()?;

            *output_ref = (self.func)(input_0_ref.deref(), input_1_ref.deref());

            Ok(())
        }
    }
}
pub use binary_func::*;