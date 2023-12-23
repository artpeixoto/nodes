#![no_std]
#![no_implicit_prelude]

extern crate nnp_base_runner_deps;
extern crate nnp_base_runner_macros;

pub mod deps{
	pub use nnp_base_runner_deps::*;
}
pub mod macros{
	pub use nnp_base_runner_macros::*;
}