pub trait Process<'a>{
    type TArgs: 'a;
    fn resume(&mut self, args: Self::TArgs);
}