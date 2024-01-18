
pub mod unary_func{
    use core::marker::PhantomData;
    use crate::base::node::*;
    use crate::base::proc::*;

    pub struct UnaryFunc<TInput, TOutput, TFunc>
        where TFunc: FnMut(&TInput) -> TOutput
    {
        pub func: 		TFunc,
        io_phantom: PhantomData<(TInput, TOutput)>
    }

    impl<'a, TInput, TOutput, TFunc>
        Process<'a> for UnaryFunc<TInput, TOutput, TFunc>
        where 
            TFunc: FnMut(&TInput) -> TOutput,
            TInput: 'a,
            TOutput: 'a,
    {
        type TArgs = (
            NodeRef<'a, TInput>,
            NodeRefMut<'a, TOutput>
        );

        fn resume(
            &mut self, 
            (input, mut output): Self::TArgs
        ) {
            let new_output = (self.func)(&input);
            *output = new_output;
        }
    }
}

pub use unary_func::*;
pub mod binary_func {
    use core::marker::PhantomData;
    use crate::base::node::*;
    use crate::base::proc::*;

    pub struct
        BinaryFunc<TInput0, TInput1, TOutput, TFunc>
        where TFunc: FnMut(&TInput0, &TInput1) -> TOutput
    {
        pub func    : TFunc,
        io_phantom  : PhantomData<(TInput0, TInput1, TOutput)>
    }


    impl<'a, TInput0, TInput1, TOutput, TFunc> Process<'a> for BinaryFunc<TInput0, TInput1, TOutput, TFunc>
        where 	
            TFunc: FnMut(&TInput0, &TInput1) -> TOutput, 
            TInput0: 'a,
            TInput1: 'a,
            TOutput: 'a,
    {
        type TArgs = (
            NodeRef<'a, TInput0>,
            NodeRef<'a, TInput1>, 
            NodeRefMut<'a, TOutput>
        );

        fn resume(
            &mut self,
            (input_0, input_1, mut output_dest): Self::TArgs
        ) {
            *output_dest = (self.func)(&input_0, &input_1);
        }
    }
}
pub use binary_func::*;