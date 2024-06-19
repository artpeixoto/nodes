use core::{ops::{Coroutine, DerefMut}, pin::{Pin, self}, marker::PhantomData};
use crate::base::proc::Process;
pub struct CoroutineProcess<TCorr, TCorrDeref, TArgs> 
	where 
		for<'a> TArgs:  'a,
		TCorr: Coroutine<TArgs>,
		TCorrDeref: DerefMut<Target=TCorr>
{
	coroutine: 	Pin<TCorrDeref>,
	phantom: 	PhantomData<TArgs>
}

impl<TCorr, TCorrDeref, TArgs> 
CoroutineProcess<TCorr, TCorrDeref, TArgs>
where 
	for<'a> TArgs:  'a,
	TCorr: Coroutine<TArgs>,
	TCorrDeref	: DerefMut<Target=TCorr>,
{
	pub fn new(corr: Pin<TCorrDeref>) -> Self{
		Self { 
			coroutine	: corr,
			phantom		: PhantomData
		}
	}
}


impl<'process, TCorr, TCorrDeref, TArgs> 
	Process<'process> for CoroutineProcess<TCorr, TCorrDeref, TArgs>
where 
	for<'a> TArgs: 'process + 'a   ,
	for<'a> TCorr:  Coroutine<TArgs > + 'a + 'process,
	TCorrDeref	: DerefMut<Target=TCorr> + 'process,
{
    type TArgs = TArgs;
    fn resume(&mut self, args: Self::TArgs) {
		use core::ops::CoroutineState::*;
		match  self.coroutine.as_mut().resume(args){
    		Yielded(yielded) 	=> {},	
    		Complete(completed) => {},
		}
    }
}
