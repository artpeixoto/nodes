use core::{error::Error};
use core::cell::{BorrowError, BorrowMutError};
use crate::extensions::used_in::UsedInTrait;
use crate::process_errors::NodeBorrowError;


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

    pub enum NodeBorrowError{
        BorrowError(BorrowError),
        BorrowMutError(BorrowMutError),
    }

    impl Debug for NodeBorrowError {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            match self{
                NodeBorrowError::BorrowError(_) => f.write_str(stringify!(BorrowError)),
                NodeBorrowError::BorrowMutError(borrow_error) => f.write_str(stringify!
                (BorrowMutError)),
            }
        }
    }

    impl Display for NodeBorrowError {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            match self{
                NodeBorrowError::BorrowError(_) => f.write_str(stringify!(BorrowError)),
                NodeBorrowError::BorrowMutError(borrow_error) => f.write_str(stringify!
                (BorrowMutError)),
            }
        }
    }

    impl Error for NodeBorrowError{

    }
    impl From<BorrowMutError> for NodeBorrowError{
        fn from(value: BorrowMutError) -> Self {
            NodeBorrowError::BorrowMutError(value)
        }
    }

    impl From<BorrowError> for NodeBorrowError{
        fn from(value: BorrowError) -> Self {
            NodeBorrowError::BorrowError(value)
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

