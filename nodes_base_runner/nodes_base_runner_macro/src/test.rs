use quote::quote;
use syn::parse2;
use crate::input::{BuildRunnerInput, NodesInput, ProcsInput};
use crate::output;

#[test]
fn test_builder(){
    let nodes_input = quote!(
        {
            int_node: Node<i32> = 0,
            string_node: Node<String> = "String".to_owned(),
        }
    );

    let nodes_res : NodesInput = parse2(nodes_input.clone()).expect("Couldn't parse nodes_input");

    let procs_input = quote!(
        {
            count_cycles((), mut int_node),
            make_log_msg(int_node, mut string_node),
            write_log(string_node),
        }
    );

    let test_procs : ProcsInput = parse2(procs_input.clone()).expect("Couldn't parse procs_input");
    //debug_val!(test_procs);

    let full_input= quote!{
        nodes: #nodes_input,
        processes: #procs_input
    };

    let build_runner_input =
        parse2::<BuildRunnerInput>(full_input)
        .expect("Something went wrong while trying to parse this bitchass motherfucker as a \
    BuildRunnerInput");

    let output = output::generate_runner_code(build_runner_input);
    println!("{output}");
}