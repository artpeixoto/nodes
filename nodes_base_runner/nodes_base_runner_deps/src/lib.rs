#![feature(coroutine_trait)]
#![no_std]
pub mod build_runner_deps{
    pub mod proc{

    }
    use core::ops::Coroutine;
    use core::pin::Pin;

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