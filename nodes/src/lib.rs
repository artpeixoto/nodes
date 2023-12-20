#![no_std]
#![feature(
    associated_type_defaults,
    impl_trait_in_fn_trait_return,
    impl_trait_in_assoc_type,
    error_in_core,
    type_name_of_val
)]
#![feature(coroutine_trait)]
#![feature(inherent_associated_types)]

extern crate nodes_base;
extern crate alloc;
extern crate heapless;
extern crate either;

pub mod base;
pub mod common_types;
pub mod nodes;
pub use nodes::*;