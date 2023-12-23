

pub trait Process{
    type TArgs<'a>;
    fn resume<'a>(&mut self, args: Self::TArgs<'a>);
}



