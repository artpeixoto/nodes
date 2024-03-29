#![feature(corroutines, corroutine_trait)]
use core::{ops::{Coroutine, DerefMut}, pin::{Pin, self}, marker::PhantomData};
use crate::base::proc::Process;
pub struct CoroutineWrapper<TCorr, TCorrDeref, TArgs> 
	where TCorr: Coroutine<TArgs> , TCorrDeref: DerefMut<Target=TCorr>
{
	coroutine: 	Pin<TCorrDeref>,
	phantom: 	PhantomData<TArgs>
}

impl<TCorr, TCorrDeref, TArgs> 
	CoroutineWrapper<TCorr, TCorrDeref, TArgs>
	where 
		TCorr		: Coroutine<TArgs> ,
		TCorrDeref	: DerefMut<Target=TCorr>,
{
	pub fn new(corr: Pin<TCorrDeref>) -> Self{
		Self { 
			coroutine	: corr,
			phantom		: PhantomData
		}
	}
}


impl<TCorr, TCorrDeref, TArgs> 
	Process for CoroutineWrapper<TCorr, TCorrDeref, TArgs>
	where 
		TCorr: Coroutine<TArgs>,
		TCorrDeref: DerefMut<Target=TCorr>, 
{
    type TArgs<'a> = TArgs;
    fn resume<'a>(&mut self, args: Self::TArgs<'a>) {
		use core::ops::CoroutineState::*;
		match  self.coroutine.as_mut().resume(args){
    		Yielded(yielded) 	=> {},	
    		Complete(completed) => {},
		}
    }
}
