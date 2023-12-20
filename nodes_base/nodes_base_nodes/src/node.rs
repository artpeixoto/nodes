use core::{alloc::{GlobalAlloc, Layout}, cell::{RefCell, Ref, BorrowError, BorrowMutError, RefMut}, borrow::{Borrow}, ops::{Deref, DerefMut}, mem::{replace}, fmt::{Debug}, ptr::NonNull, pin::Pin, marker::PhantomData};
use core::error::Error;
use crate::extensions::replace_with::{ReplaceErr, TryReplace};
use crate::process_errors::NodeBorrowError;

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

pub struct NodeRef<'a, T> where T: 'a{
	cell_ref: Ref<'a, T>
}

impl<'a, T> Deref for NodeRef<'a, T> where T: 'a{
	type Target = T;

	fn deref(&self) -> &Self::Target {
		self.cell_ref.deref()
	}
}


pub mod node_ref_mut{
    use core::{cell::RefMut, ops::{Deref, DerefMut}};

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
		where T: 'cell_ref{
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
	type TTryDerefError = NodeBorrowError;

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

pub trait TryDeref{
	type TRef<'a>: Deref + 'a where Self: 'a;
	type TTryDerefError: Debug;
	fn try_deref<'a>(&'a self) -> Result<Self::TRef<'a>, Self::TTryDerefError>;
}

pub trait TryDerefMut : TryDeref{
	type TMut<'a> : DerefMut + 'a where Self: 'a;
	type TTryDerefMutError: Debug;
	fn try_deref_mut<'a>(&'a self) -> Result<Self::TMut<'a>, Self::TTryDerefMutError>;
}


impl<T> From<T> for Node<T>{
	fn from(value: T) -> Self {
		Self::new(value)
	}
}