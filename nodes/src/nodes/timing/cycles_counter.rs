use core::pin::Pin;
use crate::base::{NodeRef, NodeRefMut, Process};
pub struct SpinCounter {}
impl Process for SpinCounter{
    type TArgs<'args> where Self: 'args = (NodeRefMut<'args, u64>);
    fn resume<'args>(&mut self, mut cycles_count: Self::TArgs<'args>) {
        *cycles_count += 1;
    }
}

