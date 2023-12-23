
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

    impl<TInput, TOutput, TFunc>
        Process for UnaryFunc<TInput, TOutput, TFunc>
        where 
            TFunc: FnMut(&TInput) -> TOutput,
            for<'a> TInput: 'a,
            for<'a> TOutput: 'a,
    {
        type TArgs<'a> = (
            NodeRef<'a, TInput>,
            NodeRefMut<'a, TOutput>
        );

        fn resume<'a>(
            &mut self, 
            (input, mut output): Self::TArgs<'a>
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


    impl<TInput0, TInput1, TOutput, TFunc> Process for BinaryFunc<TInput0, TInput1, TOutput, TFunc>
        where 	
            TFunc: FnMut(&TInput0, &TInput1) -> TOutput, 
            for<'a> TInput0: 'a,
            for<'a> TInput1: 'a,
            for<'a> TOutput: 'a,
    {
        type TArgs<'a> = (NodeRef<'a, TInput0>, NodeRef<'a, TInput1>, NodeRefMut<'a, TOutput>);

        fn resume<'a>(
            &mut self,
            (input_0, input_1, mut output_dest): Self::TArgs<'a>
        ) {
            *output_dest = (self.func)(&input_0, &input_1);
        }
    }
}
pub use binary_func::*;