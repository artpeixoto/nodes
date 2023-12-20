use core::cell::BorrowError;
use crate::base::*;
pub type SampleNode<'a, T>      = Node<Option<T>>;
pub type SampleNRef<'a, T>   = NodeRef<'a, Option<T>>;
pub type SampleNMut<'a, T> = NodeRefMut<'a, Option<T>>;



