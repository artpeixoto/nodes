use core::cell::BorrowError;

use crate::nodes::base::{NodeRef, Node};

pub type SampleNode<'a, T>      = Node<Option<T>>;
pub type SampleNodeRef<'a, T>   = NodeRef<'a, Option<T>>;

impl<T> SampleNodeRef<'_, T>{
    pub fn try_has_value(&self) -> Result<bool,BorrowError> { 
        let is_some = 
            self
            .try_borrow()?
            .is_some();

        Ok(is_some)
    }
}
