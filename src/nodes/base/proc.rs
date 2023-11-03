use alloc::boxed::Box;
use alloc::string::String;
use core::{error::Error};
use core::cell::{BorrowError, BorrowMutError};
use either::Either;
use embedded_hal::can::ErrorKind;
use crate::extensions::used_in::UsedInTrait;
use crate::nodes::base::process_errors::NodeBorrowError;


pub trait Process{

	type NextOutput<'output>;
	type NextInput<'input>;
	fn next<'input, 'output>(&mut self, input: Self::NextInput<'input>)
        -> Self::NextOutput<'output>;
}

pub mod process_errors{
    use core::cell::{BorrowError, BorrowMutError};
    use core::error::Error;
    use core::fmt::{Debug, Display, Formatter};
    use either::Either;

    pub struct NodeBorrowError(pub Either<BorrowError, BorrowMutError>);

    impl Debug for NodeBorrowError {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            match self.0{
                Either::Left(_) => f.write_str(stringify!(BorrowError)),
                Either::Right(_) => f.write_str(stringify!(BorrowMutError)),
            }
        }
    }

    impl Display for NodeBorrowError {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            match self.0{
                Either::Left(_) => f.write_str(stringify!(BorrowError)),
                Either::Right(_) => f.write_str(stringify!(BorrowMutError)),
            }
        }
    }

    impl Error for NodeBorrowError{

    }
    impl From<BorrowMutError> for NodeBorrowError{
        fn from(value: BorrowMutError) -> Self {
            NodeBorrowError(Either::Right(value))
        }
    }

    impl From<BorrowError> for NodeBorrowError{
        fn from(value: BorrowError) -> Self {
            NodeBorrowError(Either::Left(value))
        }
    }
}
pub trait SimpleProcess{
    type TError: Error = NodeBorrowError;
    fn next(&mut self) -> Result<(), Self::TError>;
}

impl<TSelf> Process for TSelf
    where TSelf: SimpleProcess
{
    type NextOutput<'output> = Result<(), <Self as SimpleProcess>::TError>;
    type NextInput<'input>   = ();
    fn next<'input, 'output>(&mut self, input: Self::NextInput<'input>) -> Self::NextOutput<'output> {
        SimpleProcess::next(self)
    }
}

