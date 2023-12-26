#![no_std]
#![feature(
    associated_type_defaults,
    impl_trait_in_fn_trait_return,
    impl_trait_in_assoc_type,
    error_in_core,
)]
#![feature(inherent_associated_types)]
// #![feature(coroutine_trait)]
// #![feature(coroutines)]
extern crate alloc;

pub mod base{
    extern crate nnp_base;
    pub use nnp_base::*;
    pub use nnp_base::core::*;
}
pub mod prelude{
    pub use super::base::prelude::*;
}

pub mod common_types;
pub mod nodes;
pub use nodes::*;