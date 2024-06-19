pub trait UsedInTrait: Sized{
    #[inline(always)]
	fn used_in<T, TFn: FnOnce(Self) -> T>(self, func: TFn) -> T{
		func(self)
	}

    #[inline(always)]
	fn ref_used_in<T, TFn: FnOnce(&Self) -> T>(&self, func: TFn) -> T {
		func(self)
	}

    #[inline(always)]
	fn mut_used_in<T, TFn: FnOnce(&mut Self) -> T>(&mut self, func: TFn) -> T{
		func(self)
	}
}
impl<T:Sized> UsedInTrait for T {}