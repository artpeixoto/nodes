#![no_std]
#![feature(
    associated_type_defaults,
    impl_trait_in_fn_trait_return,
    impl_trait_in_assoc_type,
    error_in_core,
    adt_const_params,
    type_name_of_val
)]

extern crate alloc;
extern crate nodes_base_nodes;
extern crate nodes_base_runner_deps;
extern crate nodes_base_runner_macro;
pub mod common_types;
pub mod nodes;
pub mod base;

pub use nodes::*;
