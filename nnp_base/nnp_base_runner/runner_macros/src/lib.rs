#![feature(proc_macro_quote)]
mod output;
mod input;
mod test;
mod base;

use proc_macro;
use syn::parse2;

type TokenStream1 = proc_macro::TokenStream;
type TokenStream2 = proc_macro2::TokenStream;

#[proc_macro]
pub fn build_runner(ts: TokenStream1) -> TokenStream1{
    use input::*;

    let build_runner_input = parse2::<BuildRunnerInput>(ts.into()).unwrap();

    let runner_code = output::generate_runner_code(build_runner_input);

    //println!("{runner_code}");

    runner_code.into()
}