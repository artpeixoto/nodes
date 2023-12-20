
pub mod unary_func{
    use core::ops::Deref;
    use core::pin::Pin;
    use crate::base::{NodeRef, NodeRefMut, Process};

    pub struct PureUnaryFunc<TInput, TOutput, TFunc>
        where
            TInput: PartialEq + Clone,
            TFunc: Fn(&TInput) -> TOutput
    {
        pub func 	: TFunc,
        input_cache	: Option<TInput>,
    }

    impl< TInput, TOutput, TFunc>
        PureUnaryFunc< TInput, TOutput, TFunc>
        where
            TInput: PartialEq + Clone,
            TFunc: Fn(&TInput) -> TOutput
    {
        pub fn new(func: TFunc) -> Self{
            Self {
                func,
                input_cache: None
            }
        }
    }

    impl<TInput, TOutput, TFunc>
        Process for PureUnaryFunc<TInput, TOutput, TFunc>
        where
            for<'a> TInput: PartialEq + Clone + 'a,
            for<'a> TOutput: 'a,
            TFunc:  Fn(&TInput) -> TOutput
    {
        type TArgs<'a> = (NodeRef<'a, TInput>, NodeRefMut<'a, TOutput>);

        fn resume<'args>(&mut self, (input, mut output_dest): Self::TArgs<'args>) {
            let should_update = match &self.input_cache {
                None => true,
                Some(cache_value) => input.deref() != cache_value
            };

            if should_update {
                let new_output = (self.func)(&input);
                self.input_cache = Some(input.deref().clone());
                *output_dest = new_output;
            }
        }
    }
}

pub use unary_func::*;

pub mod binary_func{
    use core::ops::Deref;
    use core::pin::Pin;
    use crate::base::{NodeRef, NodeRefMut, Process};

    pub struct PureBinaryFunc< TInput1: PartialEq + Clone, TInput2: PartialEq + Clone, TOutput, TFunc: Fn(&TInput1, &TInput2) -> TOutput>{
        pub func 			: TFunc,
        input_cache			: Option<(TInput1, TInput2)>,
    }


    impl< TInput1, TInput2, TOutput, TFunc>
        PureBinaryFunc< TInput1, TInput2, TOutput, TFunc>
        where
            TInput1: PartialEq + Clone,
            TInput2: PartialEq + Clone,
            TFunc: Fn(&TInput1, &TInput2) -> TOutput
    {
        pub fn new(func: TFunc) -> Self{
            Self {
                func,
                input_cache: None
            }
        }
    }

    impl<TInput0, TInput1, TOutput, TFunc>
        Process for PureBinaryFunc<TInput0, TInput1, TOutput, TFunc>
        where
            for<'a> TInput0: PartialEq + Clone + 'a,
            for<'a> TInput1: PartialEq + Clone + 'a,
            for<'a> TOutput:  'a,
            TFunc:   Fn(&TInput0, &TInput1) -> TOutput
    {
        type TArgs<'a> =
            (NodeRef<'a, TInput0>, NodeRef<'a, TInput1>, NodeRefMut<'a, TOutput> );

        fn resume<'args>(&mut self, (input_0_ref, input_1_ref, mut output): Self::TArgs<'args> ) {
            let (input_0, input_1) = (input_0_ref.deref(), input_1_ref.deref());
            let should_update = match &self.input_cache{
                None => true,
                Some((input_0_cache, input_1_cache)) => {
                    input_0_cache != input_0|| input_1_cache != input_1
                }
            };

            if should_update {
                let new_output = (self.func)(input_0, input_1);
                self.input_cache = Some((input_0.clone(), input_1.clone()));
                *output = new_output;
            }

        }
    }
}
pub use binary_func::*;

// pub mod into_pure{
//
//     pub trait IntoPure: SimpleProcess{
//         type PureProc: SimpleProcess;
//         fn into_pure(self) -> Self::PureProc;
//     }
//
//     impl<'a, TInput, TOutput, TFunc>
//         IntoPure
//         for UnaryFunc<'a, TInput, TOutput, TFunc>
//         where
//             TInput: PartialEq + Clone,
//             TFunc:  Fn(&TInput) -> TOutput
//     {
//         type PureProc = PureUnaryFunc<'a, TInput, TOutput, TFunc>;
//         fn into_pure(self) -> Self::PureProc {
//             PureUnaryFunc{
//                 input: 			self.input,
//                 output: 		self.output,
//                 func:   		self.func,
//                 input_cache: 	None
//             }
//         }
//     }
//
//     impl<'a, TInput0, TInput2, TOutput, TFunc>
//         IntoPure for BinaryFunc<'a, TInput0, TInput2, TOutput, TFunc>
//         where
//             TInput0: PartialEq + Clone,
//             TInput2: PartialEq + Clone,
//             TFunc: Fn(&TInput0, &TInput2) -> TOutput
//     {
//         type PureProc = PureBinaryFunc<'a, TInput0, TInput2, TOutput, TFunc>;
//         fn into_pure(self) -> Self::PureProc {
//             PureBinaryFunc{
//                 input_1: 		self.input.0,
//                 input_2: 		self.input.1,
//                 output: 		self.output,
//                 func:   		self.func,
//                 input_cache: 	None
//             }
//         }
//     }
// }
// pub use into_pure::*;