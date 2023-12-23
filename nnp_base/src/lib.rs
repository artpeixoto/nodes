#![no_std]
#![no_implicit_prelude]
#![no_core]
#![feature(no_core)]

extern crate nnp_base_core;
extern crate nnp_base_runner;


pub mod core{
    pub use nnp_base_core::*;
}
pub mod runner {
    pub use nnp_base_runner::deps::build_runner_deps::*;
    pub use nnp_base_runner::macros::build_runner;
}

pub mod prelude{
    pub use crate::core::node::*;
    pub use crate::core::proc::*;
    pub use crate::runner::*;
}