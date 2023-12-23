use core::{cell::{RefCell}};
use core::error::Error;

pub use crate::extensions::try_deref::TryDeref;
pub use crate::extensions::try_deref::TryDerefMut;
pub use core::ops::Deref;
pub use core::ops::DerefMut;


pub struct Node<T> {
	ref_item:	RefCell<T>,
}

impl<T> Node<T>{
	pub fn new(value: T) -> Self{
		Self {
			ref_item: RefCell::new(value) 
		}
	}
}

pub mod node_borrow_error{

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

pub use node_borrow_error::NodeBorrowError;

mod node_ref{
	use super::*;	
    use core::{cell::Ref};
	pub struct NodeRef<'a, T> where T: 'a{
		pub(super) cell_ref: Ref<'a, T>
	}

	impl<'a, T> Deref for NodeRef<'a, T> where T: 'a{
		type Target = T;

		fn deref(&self) -> &Self::Target {
			self.cell_ref.deref()
		}
	}
}

pub use node_ref::*;

pub mod node_ref_mut{
    use core::{cell::RefMut, ops::{self}};
	use super::*;

	pub struct ChangeDetector{
		has_changed: bool
	}
	impl ChangeDetector{
		pub fn new() -> Self{
			Self{has_changed: false}
		}
		pub fn inform_changes_happened(&mut self) {
			self.has_changed = true;
		}
		pub fn has_changed(&self) -> bool{
			self.has_changed
		}
	}
	pub struct NodeRefMut<'cell_ref, T> 
		where T: 'cell_ref
	{
		pub(super) cell_ref			: RefMut<'cell_ref, T>,
		pub(super) change_detector	: Option<&'cell_ref mut ChangeDetector>,
	}

	impl<'cell_ref, T> NodeRefMut<'cell_ref, T>{
		pub fn add_change_detector(&mut self, change_detector: &'cell_ref mut ChangeDetector)
		{
			self.change_detector = Some(change_detector)
		}
	}

	impl<'a, T> Deref for NodeRefMut<'a, T>{
		type Target = T;
		fn deref(&self) -> &Self::Target {
			self.cell_ref.deref()
		}
	}

	impl<'a, T> DerefMut for NodeRefMut<'a,T>{
		fn deref_mut(&mut self) -> &mut Self::Target {
			self.change_detector.as_deref_mut().map(|change_detector| change_detector.inform_changes_happened());
			self.cell_ref.deref_mut()
		}
	}
}

pub use node_ref_mut::*;

impl<T> TryDeref for Node<T>{
	type TRef<'a>  = NodeRef<'a, T> where T: 'a;
	type TTryDerefError = node_borrow_error::NodeBorrowError;

	fn try_deref<'a>(&'a self) -> Result<Self::TRef<'a>, Self::TTryDerefError> {
		let cell_ref = self.ref_item.try_borrow()?;
		Ok(NodeRef {cell_ref})
	}
}

impl<T> TryDerefMut for Node<T>{
	type TMut<'a> = NodeRefMut<'a, T> where T: 'a;
	type TTryDerefMutError = NodeBorrowError;

	fn try_deref_mut<'a>(&'a self) -> Result<Self::TMut<'a>, Self::TTryDerefMutError> {
		let cell_ref = self.ref_item.try_borrow_mut()?;
		Ok(NodeRefMut {cell_ref , change_detector: None})
	}
}


impl<T> From<T> for Node<T>{
	fn from(value: T) -> Self {
		Self::new(value)
	}
}