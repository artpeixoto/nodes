use core::ops::{DerefMut, Deref};
use core::fmt::Debug;

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