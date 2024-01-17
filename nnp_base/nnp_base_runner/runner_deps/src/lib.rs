#![no_std]
extern crate nnp_base_core;

pub mod build_runner_deps{
    pub use heapless::Deque;
    pub use nnp_base_core::{node::*, proc::*, extensions::*};

    pub mod opens {
        pub trait OpensMut<TKey>: OpensRef<TKey> {

            #[inline(always)]
            fn get_mut(&mut self, key: &TKey) -> &mut Self::TRet;
        }

        pub trait OpensRef<TKey> {
            type TRet;

            #[inline(always)]
            fn get_ref(&self, key: &TKey) -> &Self::TRet;
        }
    }


    pub use opens::*;
}