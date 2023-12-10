#![no_std]
#![feature(coroutines, coroutine_trait )]
pub mod build_runner_deps{

    pub mod proc{
        use core::ops::Coroutine;
        pub trait Process<TArg>: Coroutine<TArg, Yield=(), Return=()> {}

    }

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
    pub use heapless::Deque;
}