use std::cmp::Ordering;
use std::cmp::Ordering::Less;
use nnp_base_core::extensions::used_in::UsedInTrait;
use proc_macro2::{Delimiter, Literal, TokenStream};
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter, Pointer};
use std::collections::hash_map::RandomState;
use quote::{format_ident, quote, ToTokens};
use syn::{braced, custom_keyword, Expr, ExprCall, ExprReference, Member, parse2, parse_macro_input, parse_quote, Token, Type, TypePath};
use syn::{Ident};
use syn::Expr::Field;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use crate::input::{BuildRunnerInput, NodeInput, NodesInput, ProcArgMutability, ProcsInput};
use crate::base::{NodeIndex, ProcIndex};

pub fn generate_runner_code(build_runner_input: BuildRunnerInput) -> TokenStream{
    let nodes = {
        let mut res = HashMap::<NodeIndex, NodeInput,RandomState>::new();
        res.extend(build_runner_input.nodes_input.nodes_defs );
        res
    };

    let nodes_indexes =
        nodes.keys().collect::<Vec<_>>();

    let nodes_definitions =
        nodes.values().collect::<Vec<_>>();

    let nodes_fields =
        nodes_indexes.iter()
        .map(|&index| (index, &nodes[index].node_name))
        .used_in(HashMap::<_,_,RandomState>::from_iter);

    let procs = &build_runner_input.procs_input.procs;

    let procs_indexes ={
        procs
        .iter()
        .enumerate()
        .map(|(index, proc)| index)
        .collect::<Vec<ProcIndex>>()
    };

    let procs_fields = {
        procs_indexes
        .iter()
        .map(|index| (index, syn::Index::from(*index) ))
        .used_in(HashMap::<_,_,RandomState>::from_iter)
    };

    pub struct ProcDependencies<'a> {
        args:       HashMap<&'a NodeIndex, ProcArgMutability>,
        args_order: Vec<(&'a NodeIndex, ProcArgMutability)>
    }

    let procs_dependencies : HashMap<ProcIndex, ProcDependencies> = {
        procs
        .iter()
        .map(|proc| {
            let mut args = HashMap::<&NodeIndex, ProcArgMutability>::new();
            let mut args_order = Vec::<(&NodeIndex, ProcArgMutability)>::new();
            for (_proc_arg_index, proc_arg) in proc.args.iter().enumerate() {
                let node_index ={
                    let node_name = proc_arg.get_node_name();
                    let Some((node_index, _)) = nodes.get_key_value(&node_name) else {
                        panic!("{node_name} is not a node that exists");
                    };
                    node_index
                };

                let proc_arg_mutability = &proc_arg.mutability;

                args_order.push((node_index, proc_arg_mutability.clone()));

                match args.get_mut(&node_index) {
                    None => {
                        args.insert(&node_index, proc_arg_mutability.clone());
                    }
                    Some(mutability) => {
                        if proc_arg_mutability > mutability {
                            *mutability = proc_arg_mutability.clone();
                        }
                    }
                }
            }
            ProcDependencies {
                args,
                args_order
            }
        })
        .enumerate()
        .used_in(HashMap::from_iter)
    };


    let starter_procs =
        procs_indexes
        .iter()
        .map(|&ix| (ix, &procs[ix]))
        .filter(|(_ix, proc)| proc.is_starter)
        .map(|(ix, proc)| ix)
        .collect::<Vec<_>>();

    let nodes_dependants = {
        let mut nodes_dependants : HashMap<&NodeIndex, HashSet<ProcIndex>> =
            nodes_indexes.iter().map(|node_index| (*node_index, HashSet::new())).used_in(HashMap::from_iter);

        for (proc_index, proc_dep) in procs_dependencies.iter(){
            for node_index in proc_dep.args.keys(){
                nodes_dependants.get_mut(node_index).unwrap().insert(*proc_index);
            }
        }
        nodes_dependants
    };

    let (nodes_ids_defn_ts, nodes_ids_construction_ts, nodes_ids_types, nodes_ids_keys) = {
        pub type NodeIdKey = u16;
        let nodes_ids_type_defn =
            quote! (
                type NodeIdKey = u16;

                #[derive(Clone, Copy, Default)]
                struct NodeId<const NODE_ID: NodeIdKey> {}

                impl<const NODE_ID: NodeIdKey> NodeId<NODE_ID>{
                    fn new() -> Self{
                        Self{}
                    }
                }

                fn make_node_id<const NODE_ID: NodeIdKey>() -> NodeId<NODE_ID>{
                    NodeId::new()
                }
            );

        let nodes_names_mod = {
            let individual_nodes_names =
                nodes_indexes
                .iter()
                .enumerate()
                .map(|(ix, node_ix )| {
                    let node_field = nodes_fields[node_ix];
                    let node_id_key = ix as NodeIdKey;
                    quote!(
                        pub const #node_field: NodeIdKey = #node_id_key;
                    )
                });

            quote!(
                struct NodesNames{}
                impl NodesNames{
                    #(#individual_nodes_names)*
                }
            )
        };

        let get_node_id_key = |node_def: &NodeInput| -> TokenStream{
            let node_ident = &node_def.node_name;
            quote!( {NodesNames::#node_ident} )
        };

        let get_node_id_type =
            |node_def: &NodeInput| -> TokenStream {
                let node_ident = &node_def.node_name;
                quote!(NodeId<{NodesNames::#node_ident}>)
            };

        let nodes_ids_struct = {
            let individual_nodes_ids =
                nodes_definitions
                .iter()
                .map(|node_defn| {
                    let node_id_type = get_node_id_type(node_defn);
                    let node_ident = &node_defn.node_name;
                    quote!(
                        #node_ident: #node_id_type
                    )
                });

            quote!(
                #[derive(Clone, Default)]
                pub struct NodesGetters{
                    #(#individual_nodes_ids, )*
                }
            )
        };

        let nodes_ids_construction_ts =
            quote!( NodesGetters::default() );

        let nodes_ids_defn_ts =
            TokenStream::from_iter([
                nodes_ids_type_defn,
                nodes_names_mod,
                nodes_ids_struct,
            ]);

        let nodes_ids_types =
            nodes
            .iter()
            .map(|pair| (pair.0, get_node_id_type(pair.1)))
            .used_in(HashMap::<_,_,RandomState>::from_iter);

        let nodes_ids_keys =
            nodes
            .iter()
            .map(|pair| (pair.0, get_node_id_key(pair.1)))
            .used_in(HashMap::<_,_,RandomState>::from_iter);

        (nodes_ids_defn_ts, nodes_ids_construction_ts, nodes_ids_types, nodes_ids_keys)
    };

    let (nodes_struct_defn_ts, nodes_construction_ts) = {
        let nodes_struct_defn = {
            let stuff =
                nodes_definitions
                .iter()
                .map(|node_def| {
                    let node_ident = &node_def.node_name;
                    let node_type = &node_def.node_type;
                    quote!( #node_ident: #node_type )
                });

            quote!(
                struct Nodes {
                    #(#stuff),*
                }
            )
        };

        let nodes_struct_gets_impls = {
            nodes
            .iter()
            .map(|(node_name, node_def)| {
                let node_id_type = nodes_ids_types.get(node_name).unwrap();
                let node_type = &node_def.node_type;
                let node_ident = &node_def.node_name;
                quote! {
                    impl OpensRef<#node_id_type> for Nodes{
                        type TRet = #node_type;

                        fn get_ref(&self, key: &#node_id_type) -> &Self::TRet{
                            &self.#node_ident
                        }
                    }

                    impl OpensMut<#node_id_type> for Nodes{
                        fn get_mut(&mut self, key: &#node_id_type) -> &mut Self::TRet{
                            &mut self.#node_ident
                        }
                    }
                }
            })
            .used_in(TokenStream::from_iter)
        };

        let nodes_struct_construction_ts = {
            let stuff =
            nodes_definitions
            .iter()
            .map(|node_def| {
                let node_name = &node_def.node_name;
                let node_value = &node_def.node_value;
                quote!( #node_name : Node::from(#node_value))
            });

            quote!(
                Nodes{
                    #(#stuff),*
                }
            )
        };


        (
            TokenStream::from_iter([nodes_struct_defn,  nodes_struct_gets_impls]),
            nodes_struct_construction_ts
        )
    };

    let (procs_ids_struct_defn_ts, procs_ids_types, procs_ids_keys) = {
        type ProcIdKey = u16;
        let proc_ids_struct_defn_ts =
            quote! {
                type ProcIdKey = u16;
                #[derive(Clone, Copy, Default)]
                struct ProcId<const KEY: ProcIdKey>{}
            };

        let procs_ids_types =
            procs_indexes
            .iter()
            .map(|index| {
                (index, {let u8_index = *index as ProcIdKey; quote! {ProcId<#u8_index>}})
            })
            .used_in(HashMap::<_, _, RandomState>::from_iter);

        let procs_ids_keys =
            procs
            .iter()
            .enumerate()
            .map(|(index, proc)| (index, {let u8_index = index as ProcIdKey; quote! {#u8_index}}))
            .used_in(HashMap::<_, _, RandomState>::from_iter);
        (proc_ids_struct_defn_ts, procs_ids_types, procs_ids_keys)
    };

    let (procs_getters_defn_ts, procs_getters_construction_ts)  = {
        let procs_individual_getters_ts =
            procs_indexes
            .iter()
            .map(|key|{
                let proc_id_type = procs_ids_types.get(&key).unwrap();
                quote!(
                    pub #proc_id_type
                )
            });

        let procs_getters_defn_ts =   quote!{
            #[derive(Clone, Default)]
            struct ProcsGetters(
                #(#procs_individual_getters_ts),*
            );
        };
        let procs_getters_construction_ts = quote!{
            ProcsGetters::default()
        };

        (procs_getters_defn_ts, procs_getters_construction_ts)
    };


    let (
        procs_generics_args,
        procs_generics_args_ts,
        procs_generics_wheres_ts,
    ) = {
        let (procs_generics_info) = {
            procs_indexes
            .iter()
            .map(|ix| {
                let proc_args = &procs_dependencies[ix];

                let proc_gen_name = format_ident!("TProc{}", ix);
                let proc_gen_args = {
                    proc_args.args_order
                    .iter()
                    .map(|(node_ix, mutability)| {
                        let node_type = &nodes[*node_ix].node_type;
                        match mutability{
                            ProcArgMutability::Read  => quote!{<#node_type as TryDeref>::TRef<'a>},
                            ProcArgMutability::Write => quote!{<#node_type as TryDerefMut>::TMut<'a>}
                        }
                    })
                };

                let proc_gen_cond = {
                    quote!{
                        #proc_gen_name: for<'a> Process<'a, TArgs = (#(#proc_gen_args),*)> 
                    }
                };
                (ix, (proc_gen_name, proc_gen_cond))
            })
            .used_in(HashMap::<_,_,RandomState>::from_iter)
        };

        let procs_generics_args = {
            procs_indexes.iter()
            .map(|proc_ix| {
                let proc_gen_name = procs_generics_info[proc_ix].0.clone();
                (proc_ix, proc_gen_name)
            })
            .used_in(HashMap::<_,_,RandomState>::from_iter)
        };

        let procs_generics_args_ts = {
            procs_indexes.iter()
            .map(|proc_ix| {
                &procs_generics_args[proc_ix]
            })
            .used_in(|procs_generics_args|quote!{
                #(#procs_generics_args),*
            })
        };


        let procs_generics_wheres_ts = {
            let procs_generics_cond =
                procs_indexes.iter()
                .map(|proc_ix| {
                    &procs_generics_info[proc_ix].1
                });
            quote!{
                #(#procs_generics_cond,)*
            }
        };
        (procs_generics_args, procs_generics_args_ts, procs_generics_wheres_ts)
    };

    let (procs_defn_ts, procs_construction_ts) = {
        let pins_procs_generics_args =
            procs_indexes.iter().map(|proc_ix|{
                let generic_arg = &procs_generics_args[proc_ix];
                quote!{#generic_arg}
            });
        let procs_struct_defn_ts = quote!{
            struct Procs<#procs_generics_args_ts> 
            (#(#pins_procs_generics_args),*)
            where #procs_generics_wheres_ts;
        };

        let procs_opens_impls_ts =
            procs_indexes.iter().map(|proc_ix|{
                let proc_id_type = &procs_ids_types[proc_ix];
                let proc_generic_arg = &procs_generics_args[proc_ix];
                let proc_field = &procs_fields[proc_ix];

                quote!{
                    impl<#procs_generics_args_ts>
                        OpensRef<#proc_id_type> for Procs<#procs_generics_args_ts>
                        where #procs_generics_wheres_ts
                    {
                        type TRet = #proc_generic_arg;
                        #[inline(always)]
                        fn get_ref(&self, key: &#proc_id_type) -> &Self::TRet{
                            &self.#proc_field
                        }
                    }

                    impl<#procs_generics_args_ts>
                        OpensMut<#proc_id_type> for Procs<#procs_generics_args_ts>
                        where #procs_generics_wheres_ts
                    {
                        #[inline(always)]
                        fn get_mut(&mut self, key: &#proc_id_type) -> &mut Self::TRet{
                            &mut self.#proc_field
                        }
                    }
                }
            });
        let procs_defn_ts =
            quote!{
                #procs_struct_defn_ts
                #(#procs_opens_impls_ts)*
            };
        let procs_construction_ts  = {
            let procs_construction_args = procs.iter().map(|proc| {
                let proc_func = &proc.func;
                quote!{#proc_func}
            });
            quote!{Procs(#(#procs_construction_args),*, )}
        };
        (procs_defn_ts, procs_construction_ts)
    };

    let (nodes_dependants_defn_ts, nodes_dependants_construction_ts) = {
        let nodes_dependants_struct_defn_ts = quote!{
            #[derive(Default)]
            struct NodesDependants{}
        };

        let nodes_dependants_impls_ts = {
            let procs_impls =
                nodes_dependants
                .iter()
                .map(|(node_key, procs_keys)| {
                    let node_name = &nodes.get(*node_key).unwrap().node_name;
                    let node_id_type = &nodes_ids_types[node_key];
                    let node_procs_ids_keys =
                        procs_keys
                        .iter()
                        .map(|proc_key| {
                            procs_ids_keys.get(proc_key).unwrap()
                        });

                    quote! {
                        impl NodesDependants{
                            const #node_name: &'static [ProcIdKey] = &[
                                #(#node_procs_ids_keys),*
                            ];
                        }

                        impl OpensRef<#node_id_type> for NodesDependants{
                            type TRet = &'static [ProcIdKey];
                            #[inline(always)]
                            fn get_ref(&self, key: &#node_id_type) -> &Self::TRet{
                                &NodesDependants::#node_name
                            }
                        }
                    }
                });
            TokenStream::from_iter(procs_impls)
        };

        let nodes_dependants_construction_ts = quote!{
            NodesDependants{}
        };

        (
            TokenStream::from_iter([
               nodes_dependants_struct_defn_ts,
               nodes_dependants_impls_ts
            ]),
            nodes_dependants_construction_ts
        )
    };

    let (runner_data_defn_ts, runner_data_construction_ts) = {

        let runner_data_defn_ts = quote!{
            struct RunnerData<#procs_generics_args_ts>
                where #procs_generics_wheres_ts
            {
                nodes:              Nodes,
                nodes_getters:      NodesGetters,
                nodes_dependants:   NodesDependants,
                procs:              Procs<#procs_generics_args_ts>,
                procs_getters:      ProcsGetters,
            }
        };

        let runner_data_construction_ts =
        |   nodes_construction_ts: TokenStream,
            nodes_getters_construction_ts: TokenStream,
            nodes_dependants_construction_ts: TokenStream,
            procs_construction_ts: TokenStream,
            procs_getters_construction_ts: TokenStream,
        | quote!{
            RunnerData{
                nodes:              #nodes_construction_ts,
                nodes_getters:      #nodes_getters_construction_ts,
                nodes_dependants:   #nodes_dependants_construction_ts,
                procs:              #procs_construction_ts,
                procs_getters:      #procs_getters_construction_ts,
            }
        };

        (runner_data_defn_ts, runner_data_construction_ts)
    };

    let (cycle_state_defn_ts, cycle_state_construction_ts) = {
        let cycle_state_defn_ts = {
            let fields_types  =
                procs.iter()
                .map(|a|{ quote!(ProcExecutionState) })
                .collect::<Vec<_>>();

            let fields = procs_indexes.iter().map(|proc_index| &procs_fields[proc_index]);

            let proc_count = procs.len();

            let cycle_execution_state_open_impls_ts = {
                procs_ids_types
                .iter()
                .map(|(proc_key, proc_id_type)| {
                    let proc_field = &procs_fields[proc_key];
                    quote!{
                        impl OpensRef<#proc_id_type> for  CycleExecutionState{
                            type TRet = ProcExecutionState;
                            fn get_ref(&self, key: &#proc_id_type) -> &Self::TRet{
                                &self.#proc_field
                            }
                        }
                        impl OpensMut<#proc_id_type> for  CycleExecutionState{
                            fn get_mut(&mut self, key: &#proc_id_type) -> &mut Self::TRet{
                                &mut self.#proc_field
                            }
                        }
                    }
                })
                .used_in(TokenStream::from_iter)
            };

            quote!(
                #[derive(PartialEq, Eq, Clone )]
                enum ProcExecutionState {
                    NotExecuted, Finished, ReQueued, Queued
                }

                impl Default for ProcExecutionState{
                    fn default() -> Self{
                        ProcExecutionState::NotExecuted
                    }
                }

                #[derive(Default)]
                struct CycleExecutionState(
                    #(#fields_types),*
                );

                impl CycleExecutionState{
                    fn clear(&mut self){
                        #(self.#fields = ProcExecutionState::NotExecuted);*
                    }
                    fn new() -> Self {
                        let mut res =  Self::default();
                        res.clear();
                        res
                    }
                }

                #cycle_execution_state_open_impls_ts

                struct CycleState{
                    execution_queue:    Deque<ProcIdKey, #proc_count>,
                    executed:           CycleExecutionState,
                }

                impl CycleState{
                    fn clear(&mut self){
                        self.execution_queue.clear();
                        self.executed.clear();
                    }
                }
            )
        };
        let cycle_state_construction_ts = {quote!{
            CycleState{
                execution_queue:    Deque::new(),
                executed:           CycleExecutionState::new(),
            }
        }};
        (cycle_state_defn_ts, cycle_state_construction_ts)
    };

    let (runner_defn_ts, runner_construction_ts) = {
        let runner_struct_defn_ts = {
            quote! {
                pub struct Runner<#procs_generics_args_ts>
                    where #procs_generics_wheres_ts
                {
                    current_cycle_state: CycleState,
                    runner_data:         RunnerData<#procs_generics_args_ts>,
                }
            }
        };

        let runner_execution_fn_defn_ts = {
            let run_next_proc_defn_ts =
                procs_indexes.iter()
                .map(|&proc_index|{
                    let proc = procs.get(proc_index).unwrap();
                    let proc_field = &procs_fields[&proc_index];
                    let proc_id_key = &procs_ids_keys[&proc_index];
                    let proc_dependencies = &procs_dependencies[&proc_index];
                    let args_values_tuple = {
                        proc_dependencies.args_order.iter()
                        .map(|(node_index, mutability)| {
                            let node_field = &nodes_fields[node_index];
                            quote! {
                                #node_field
                            }
                        })
                        .used_in(|nodes_fields_ts| quote! {
                            (#(#nodes_fields_ts),*)
                        })
                    };
                    let args_variables_tuple = {
                        proc_dependencies.args_order.iter()
                        .map(|(node_index, mutability)| {
                            let node_field = &nodes_fields[node_index];
                            if !mutability.is_mut() {
                                quote! {
                                    #node_field
                                }
                            } else {
                                quote! {
                                    mut #node_field
                                }
                            }
                        })
                        .used_in(|nodes_fields_ts| quote! {
                                       (#(#nodes_fields_ts),*)
                                       })
                    };
                    let args_type_tuple = {
                        proc_dependencies.args_order.iter()
                        .map(|(node_index, mutability)| {
                            let node_type = &nodes[*node_index].node_type;
                            if !mutability.is_mut() {
                                quote! {
                                       <#node_type as TryDeref>::TRef<'_>
                                       }
                            } else {
                                quote! {
                                       <#node_type as TryDerefMut>::TMut<'_>
                                       }
                            }
                        })
                        .used_in(|nodes_types_ts| quote! {
                                       (#(#nodes_types_ts),*)
                                       })
                    };
                    let nodes_ids_var_names = {
                        proc_dependencies.args.iter()
                        .map(|(&node_index, mutability)| {
                            let node = &nodes[node_index];
                            let node_id_var_name = format_ident!("{}_id", node.node_name);
                            (node_index, node_id_var_name)
                        })
                        .used_in(HashMap::<_, _, RandomState>::from_iter)
                    };
                    let get_nodes_ids_ts = {
                        proc_dependencies.args.iter()
                        .map(|(&node_index, mutability)| {
                            let node_field = &nodes_fields[node_index];
                            let node_id_var_name = &nodes_ids_var_names[node_index];
                            let get_node_id_ts = quote! {
                                       let #node_id_var_name = &self.runner_data.nodes_getters.#node_field;
                                       };
                            get_node_id_ts
                        })
                        .used_in(TokenStream::from_iter)
                    };
                    let get_nodes_references_ts = {
                        proc_dependencies.args.iter()
                        .map(|(&node_index, mutability)| {
                            let node_field = &nodes_fields[node_index];
                            let node_id_var_name = &nodes_ids_var_names[node_index];
                            if !mutability.is_mut() {
                                quote! {
                                   let #node_field = self.runner_data.nodes.get_ref
                                       (#node_id_var_name).try_deref()?;
                               }
                            } else {
                                quote! {
                                   let mut #node_field = self.runner_data.nodes.get_ref
                                       (#node_id_var_name).try_deref_mut()?;
                                   }
                            }
                        })
                        .used_in(|individual_gets| quote! {
                           || -> Result<#args_type_tuple, NodeBorrowError>{
                                   #(#individual_gets)*
                                   Ok(#args_values_tuple)
                               }
                        })
                    };

                    let nodes_changes_detectors_var_names = {
                        proc_dependencies.args.iter()
                        .filter(|(_, mutability)| mutability.is_mut())
                        .map(|(&node_index, mutability)| {
                            let node_name = &nodes[node_index].node_name;
                            let node_change_detector_var_name = format_ident!("{}_change_detector",
                                       node_name);
                            (node_index, node_change_detector_var_name)
                        })
                        .used_in(HashMap::<_, _, RandomState>::from_iter)
                    };

                    let nodes_changes_detectors_preparation_ts = {
                        nodes_changes_detectors_var_names.iter()
                        .map(|(&node_index, node_change_detector_var_name)| {
                            let node_field = nodes_fields[node_index];
                            quote! {
                                let mut #node_change_detector_var_name = ChangeDetector::new();
                                #node_field.add_change_detector(&mut #node_change_detector_var_name);
                            }
                        })
                        .used_in(TokenStream::from_iter)
                    };

                    let nodes_changes_detectors_evaluation_ts = {
                        nodes_changes_detectors_var_names.iter()
                        .map(|(&node_index, node_change_detector_var_name)| {
                            let node_id_var_name = &nodes_ids_var_names[node_index];
                            let node_dependants = &nodes_dependants[node_index];
                            let node_dependants_fields =
                            node_dependants.iter()
                            .map(|proc_index| (proc_index, &procs_fields[proc_index]))
                            .used_in(HashMap::<_, _, RandomState>::from_iter);

                            let add_node_dependants_to_execution_queue_ts = {
                                node_dependants.iter()
                                .map(|proc_index| {
                                    let proc_field = node_dependants_fields[proc_index];
                                    let proc_id_key = &procs_ids_keys[proc_index];
                                    quote! { {
                                        let proc_id = &self.runner_data.procs_getters.#proc_field;
                                        if self.current_cycle_state.executed.get_ref(proc_id) == &ProcExecutionState::NotExecuted{
                                            self.current_cycle_state.execution_queue.push_back(#proc_id_key).unwrap();
                                            *(self.current_cycle_state.executed.get_mut(proc_id))  = ProcExecutionState::Queued;
                                        }
                                    } }
                                })
                                .used_in(TokenStream::from_iter)
                            };
                            quote! {
                               if #node_change_detector_var_name.has_changed(){
                                   #add_node_dependants_to_execution_queue_ts
                               }
                           }
                        })
                        .used_in(TokenStream::from_iter)
                    };

                    let proc_func = &proc.func;

                    quote! {
                       #proc_id_key => {
                           let proc_id = &self.runner_data.procs_getters.#proc_field;
                           let proc_execution_state = self.current_cycle_state.executed.get_mut(proc_id);
                           if proc_execution_state != &ProcExecutionState::Finished{
                               #get_nodes_ids_ts
                               let get_nodes_references = #get_nodes_references_ts;
                               match get_nodes_references() {
                                   Ok(#args_variables_tuple) => {
                                       #nodes_changes_detectors_preparation_ts

                                       self.runner_data.procs.get_mut(proc_id).resume(
                                            #args_values_tuple
                                       );

                                       *proc_execution_state = ProcExecutionState::Finished;

                                       #nodes_changes_detectors_evaluation_ts
                                   }
                                   Err(_) => {
                                       if proc_execution_state == &ProcExecutionState::ReQueued{
                                           *proc_execution_state = ProcExecutionState::Finished;
                                       } else {
                                           *proc_execution_state = ProcExecutionState::ReQueued;
                                           self.current_cycle_state.execution_queue.push_back(#proc_id_key).unwrap();
                                       }
                                   }
                               }
                           }
                       }
                    }
                })
                .used_in(|individual_executions|{ quote!{
                        match self.current_cycle_state.execution_queue.pop_front(){
                            Some(proc_id_key) => {
                                match proc_id_key{
                                    #(#individual_executions),*
                                    _ => {panic!("impossible shit");}
                                };
                                true
                            }
                            None => {
                                false
                            }
                        }
                }});

            let add_initial_procs_defn_ts = {
                starter_procs.iter().map(|proc_ix|{
                    let proc_id_key = &procs_ids_keys[proc_ix];
                    quote!{
                        self.current_cycle_state.execution_queue.push_back(#proc_id_key).unwrap();
                    }
                })
                .used_in(TokenStream::from_iter)
            };
            quote!{
                impl<#procs_generics_args_ts> Runner<#procs_generics_args_ts> where
                #procs_generics_wheres_ts{
                    pub fn add_initial_procs(&mut self) {
                        #add_initial_procs_defn_ts
                    }
                    pub fn run_next_proc(&mut self) -> bool{
                        #run_next_proc_defn_ts
                    }
                    
                    pub fn run_forever(&mut self) -> !{
                        loop{self.run_once();}
                    }
             
                    pub fn prepare_to_run_cycle(&mut self) {
                        self.current_cycle_state.clear();
                        self.add_initial_procs();
                    }

                    pub fn run_once(&mut self){
                        self.prepare_to_run_cycle();
                        loop {
                            if !self.run_next_proc() { break; }
                        }
                    }
                }
            }
        };

        let runner_defn_ts = TokenStream::from_iter([runner_struct_defn_ts,
            runner_execution_fn_defn_ts]);

        let runner_construction_ts = {
            |cycle_state_construction_ts: TokenStream, runner_data_construction_ts: TokenStream|
            quote!{ Runner{
                current_cycle_state: #cycle_state_construction_ts,
                runner_data:         #runner_data_construction_ts,
            } }
        };

        (runner_defn_ts, runner_construction_ts)
    };

    let static_defns_ts = quote!{
        #nodes_ids_defn_ts
        #nodes_struct_defn_ts
        #procs_ids_struct_defn_ts
        #procs_getters_defn_ts
        #procs_defn_ts
        #nodes_dependants_defn_ts
        #runner_data_defn_ts
        #cycle_state_defn_ts
        #runner_defn_ts
    };

    let construction_ts = {
        runner_construction_ts(
            cycle_state_construction_ts,
            runner_data_construction_ts(
                nodes_construction_ts,
                nodes_ids_construction_ts,
                nodes_dependants_construction_ts,
                procs_construction_ts,
                procs_getters_construction_ts,
            )
        )
    };

    let res = quote!{
        #static_defns_ts
        let mut runner = #construction_ts;
    };
    res
}


macro_rules! print_val {
        ($val_name: expr) => {println!("{} = {}", stringify!($val_name), &$val_name );};
}
macro_rules! debug_val {
        ($val_name: expr) => {println!("{} = {:#?}", stringify!($val_name), &$val_name );};
}
