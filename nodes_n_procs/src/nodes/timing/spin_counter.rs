
use crate::base::core::node::*;
use crate::base::core::proc::*;
pub struct SpinCounter {}
impl SpinCounter{
    pub fn new() -> Self{Self{}}
}
impl Process for SpinCounter{
    type TArgs<'args>  = NodeRefMut<'args, u64>;
    fn resume<'args>(&mut self, mut cycles_count: Self::TArgs<'args>) {
        *cycles_count += 1;
    }
}

