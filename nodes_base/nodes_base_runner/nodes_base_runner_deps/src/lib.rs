#![feature(coroutine_trait)]
#![no_std]
extern crate nodes_base_nodes;
pub mod build_runner_deps{
    pub use core::pin::Pin;
    pub use heapless::Deque;
    pub use nodes_base_nodes::*;
    pub use nodes_base_nodes::Process;
    pub use nodes_base_nodes::process_errors::*;
    pub mod opens {
        pub trait OpensMut<TKey>: OpensRef<TKey> {
            fn get_mut(&mut self, key: &TKey) -> &mut Self::TRet;
        }

        pub trait OpensRef<TKey> {
            type TRet;
            fn get_ref(&self, key: &TKey) -> &Self::TRet;
        }
    }
    pub use opens::*;
}