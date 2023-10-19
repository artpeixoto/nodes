use core::{alloc::{GlobalAlloc, Layout}, cell::{RefCell, Ref, BorrowError, BorrowMutError, RefMut}, borrow::{Borrow}, ops::{Deref, DerefMut}, mem::{replace}, fmt::{Debug}, ptr::NonNull, pin::Pin, marker::PhantomData};

use crate::{ extensions::replace_with::{TryReplace, ReplaceErr}};



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

pub struct NodeRef<'a, T: 'a> ( 
	&'a Node<T>
);

impl<'node_ref, T> NodeRef<'node_ref, T>{

    #[inline]
	const fn get_kernel(&self) -> &Node<T> {
		&self.0
	}

	pub fn new<'node_kernel: 'node_ref>(kernel_ref: &'node_kernel Node<T>) -> Self{
		Self(kernel_ref)
	}			

	#[inline]
	pub fn try_borrow<'a>(&'a self) -> Result<Ref<'a, T>,BorrowError> { 
		self.get_kernel().ref_item.try_borrow()
	}
	
	#[inline]
	pub fn try_borrow_mut<'a>(&'a self) -> Result<RefMut<'a, T>, BorrowMutError> {
		self.get_kernel().ref_item.try_borrow_mut()	
	}
}

impl<T:Sized> TryReplace<T> for NodeRef<'_, T>{
    fn try_replace(&mut self, mut val: T) -> Result<T, crate::extensions::replace_with::ReplaceErr<T>> {
        match self.try_borrow_mut() {
            Ok(mut ref_cell) => {
                let mut val_ref = ref_cell; 
                val =  replace(val_ref.deref_mut(), val);
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
        Self(self.0.clone())
    }
}