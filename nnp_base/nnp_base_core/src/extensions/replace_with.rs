
use core::{fmt::{Debug, Display}, error::Error, };
pub struct ReplaceErr<T>{
	value: T,
	msg: &'static str,
}

impl<T> ReplaceErr<T>{
	pub fn new_generic(value: T) -> ReplaceErr<T>{
		ReplaceErr::<T>{
			value, 
			msg: "error while trying to replace value in node"
		}
	}
	pub fn new(value: T, msg: &'static str) -> ReplaceErr<T>{
		ReplaceErr::<T>{
			value, msg
		}
	}
}


impl<T> Debug for ReplaceErr<T>{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ReplaceErr").field("msg", &self.msg).finish()
    }
}


impl<T> Display for ReplaceErr<T>{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}


impl<T> Error for ReplaceErr<T>{}

pub trait TryReplace<T: Sized>{
    fn try_replace(&mut self, val: T) -> Result<T, ReplaceErr<T>>;
}




