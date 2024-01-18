use core::marker::PhantomData;

use nnp_base::runner::Process;

pub struct FnProc<TFn, TArgs>
	where TFn: FnMut(TArgs)
{
	pub fun: TFn,
	_args_phantom: PhantomData<TArgs>	
}

impl<TFn, TArgs> FnProc<TFn, TArgs> 
	where TFn: FnMut(TArgs)
{
	pub fn new(fun: TFn) -> Self{
		Self { 
			fun: fun,
			_args_phantom: PhantomData{}
		}
	}
}

impl<'a, TFn, TArgs> 
	Process<'a> for FnProc<TFn, TArgs> 
	where 
		TFn: FnMut(TArgs),
		TArgs: 'a

{
    type TArgs = TArgs;
    fn resume(&mut self, args: Self::TArgs) {
		  (self.fun)(args)
    }
}
