
use crate::base::core::node::*;
use crate::base::core::proc::*;
pub struct SpinCounter {}
impl SpinCounter{
    pub fn new() -> Self{Self{}}
}
impl<'a> Process<'a> for SpinCounter{
    type TArgs  = NodeRefMut<'a, u64>;
    fn resume(&mut self, mut cycles_count: Self::TArgs) {
        *cycles_count += 1;
    }
}

