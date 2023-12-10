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

	pub fn make_ref<'a>(&'a self) -> NodeRef<'a, T>{
		NodeRef::new(self)
	}
}

pub struct NodeRefLock<'a, T>{
	cell_ref: Ref<'a, T>
}

impl<'a, T> Deref for NodeRefLock<'a, T>{
	type Target = T;

	fn deref(&self) -> &Self::Target {
		self.cell_ref.deref()
	}
}

pub struct ChangeDetector{
	has_changed: bool
}
impl ChangeDetector{
	pub fn new() -> Self{
		Self{has_changed: false}
	}
	pub fn has_changed(&self) -> bool{
		self.has_changed
	}
}
pub struct NodeRefMutLock<'a, T>{
	cell_ref: 	 	 RefMut<'a, T>,
	change_detector: Option<&'a mut ChangeDetector>,
}
impl<'a, T> NodeRefMutLock<'a, T>{
	pub fn add_change_detector(&mut self, change_detector: &'a mut ChangeDetector){
		self.change_detector = Some(change_detector);
	}
}

impl<'a, T> Deref for NodeRefMutLock<'a, T>{
	type Target = T;

	fn deref(&self) -> &Self::Target {
		self.cell_ref.deref()
	}
}

impl<'a, T> DerefMut for NodeRefMutLock<'a, T>{
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.change_detector.as_deref_mut().map(|change_detector| {change_detector.has_changed =
		true;});
		self.cell_ref.deref_mut()
	}
}

impl<T> TryDeref for Node<T>{
	type TRef<'a> where Self: 'a = NodeRefLock<'a, T>;
	type TTryDerefError = NodeBorrowError;

	fn try_deref<'a>(&'a self) -> Result<Self::TRef<'a>, Self::TTryDerefError> {
		let cell_ref = self.ref_item.try_borrow()?;
		Ok(NodeRefLock{cell_ref})
	}
}
impl<T> TryDerefMut for Node<T>{
	type TMut<'a> where Self: 'a = NodeRefMutLock<'a, T>;
	type TTryDerefMutError = NodeBorrowError;

	fn try_deref_mut<'a>(&'a self) -> Result<Self::TMut<'a>, Self::TTryDerefMutError> {
		let cell_ref = self.ref_item.try_borrow_mut()?;
		Ok(NodeRefMutLock{cell_ref, change_detector: None})
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

pub struct NodeRef<'a, T>
{
	kernel_ref: &'a Node<T>,
}

impl<'node_ref, T> NodeRef<'node_ref, T>{

    #[inline]
	const fn get_kernel(&self) -> &Node<T> {
		&self.kernel_ref
	}

	pub fn new<'node_kernel: 'node_ref>(kernel_ref: &'node_kernel Node<T>) -> Self{
		Self{kernel_ref}
	}

	#[inline]
	pub fn try_borrow<'a>(&'a self) -> Result<Ref<'a, T>, BorrowError> {
		self.get_kernel().ref_item.try_borrow()
	}

	#[inline]
	pub fn try_borrow_mut<'a>(&'a mut self) -> Result<RefMut<'a, T>, BorrowMutError> {
		self.get_kernel().ref_item.try_borrow_mut()
	}
}


pub trait TryBorrowMut<T>: TryBorrow<T>

{
	type TMutRef<'a>: DerefMut<Target=T> + 'a where Self: 'a;
	type TBorrowMutError: Error = BorrowMutError;
	fn try_borrow_mut<'a>(&'a self) -> Result<Self::TMutRef<'a>, Self::TBorrowMutError>;
}

pub trait TryBorrow<T>{
	type TRef<'a>: 	 Deref<Target=T> + 'a where Self: 'a;
	type TBorrowError: Error = BorrowError;
	fn try_borrow<'a>(&'a self) -> Result<Self::TRef<'a>, Self::TBorrowError>;
}


impl<T:Sized> TryReplace<T> for NodeRef<'_, T>{
    fn try_replace(&mut self, mut val: T) -> Result<T, ReplaceErr<T>> {
        match self.try_borrow_mut() {
            Ok(mut ref_cell) => {
                let mut val_ref = ref_cell; 
                val = replace(val_ref.deref_mut(), val);
                Ok(val)
            },
            Err(_err) => {
                Err(ReplaceErr::new_generic(val))
            },
        }
    }
}

impl<T> Clone for NodeRef<'_, T>{
    fn clone(&self) -> Self {
        Self{
			kernel_ref: self.kernel_ref.clone()
		}
    }
}

