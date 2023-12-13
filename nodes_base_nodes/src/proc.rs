use core::{error::Error};
use core::cell::{BorrowError, BorrowMutError};
use core::ops::Coroutine;
use crate::extensions::used_in::UsedInTrait;
use crate::process_errors::NodeBorrowError;


pub trait Process: Coroutine<Self::TArgs, Yield=(), Return=()>{
    type TArgs;
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
