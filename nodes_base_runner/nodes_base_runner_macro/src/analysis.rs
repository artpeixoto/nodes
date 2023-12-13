use core::ops::Coroutine;
use core::pin::Pin;

pub use build_runner_deps::*;
use nodes_base_nodes::{ChangeDetector, Node, Process, TryDeref, TryDerefMut};
use nodes_base_nodes::process_errors::NodeBorrowError;

pub mod build_runner_deps {
    pub use heapless::Deque;

    pub use opens::*;

    pub mod proc {}

    pub mod opens {
        pub trait OpensMut<TKey>: OpensRef<TKey> {
            fn get_mut(&mut self, key: &TKey) -> &mut Self::TRet;
        }

        pub trait OpensRef<TKey> {
            type TRet;
            fn get_ref(&self, key: &TKey) -> &Self::TRet;
        }
    }
}

pub fn do_shid() {

}
