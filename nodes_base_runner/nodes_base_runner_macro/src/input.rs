use std::cmp::Ordering;
use std::collections::HashMap;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{braced, Expr, parenthesized, parse2, Token, Type};
use syn::Fields::Unit;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use crate::base::NodeIndex;

macro_rules! parse_kwargs  {
    (let { $kw_first:ident : $kw_arg_ty_first:ty $(, $kw:ident : $kw_arg_ty:ty)* } = $input:expr )
        => {{
            syn::custom_keyword![$kw_first];
            $(
                syn::custom_keyword![$kw];
            )*

            let res = (
                parse_kw::<$kw_arg_ty_first, $kw_first>($input)?
                $(,{
                    $input.parse::<Token![,]>()?;
                    parse_kw::<$kw_arg_ty, $kw>($input)?
                })*
            );

            res
        }}
}
pub fn parse_kw<TParse: Parse , TKw: Parse>(input: ParseStream) -> syn::Result<TParse>
{
    let _kw_token = input.parse::<TKw>()?;
    let _def_sep  = input.parse::<Token![:]>()?;
    let res = input.parse::<TParse>()?;
    Ok(res)
}

#[derive(Debug, Clone)]
pub(crate) struct BuildRunnerInput {
    pub nodes_input: NodesInput,
    pub procs_input: ProcsInput,
}

impl Parse for BuildRunnerInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let (nodes_input, procs_input) =
            parse_kwargs!(
                let {
                    nodes:      NodesInput,
                    processes:  ProcsInput
                } = input
            );

        Ok(Self{nodes_input, procs_input})
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum ProcArgMutability{
    Read,
    Write,
}
impl PartialOrd for ProcArgMutability{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use ProcArgMutability::*;
        use Ordering::*;
        match (self, other){
            (Read, Read) | (Write, Write) => Some(Equal),
            (Read, Write)                 => Some(Less),
            (Write, Read)                 => Some(Greater),
        }
    }
}

impl ProcArgMutability{
    pub fn is_mut(&self) -> bool {
        match self{
            ProcArgMutability::Read =>  {false}
            ProcArgMutability::Write => {true}
        }
    }
}

pub(crate) enum SpecialProcArg{
    Unit
}
impl Parse for SpecialProcArg{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![()]>()?;
    }
}


pub(crate) enum ProcArg{
    NodeProcArg(NodeProcArg),
    SpecialProcArg
}
#[derive(Debug, Clone)]
pub(crate) struct NodeProcArg {
    pub arg:        Ident,
    pub mutability: ProcArgMutability,
}

impl NodeProcArg {
    pub fn get_node_name(&self) -> String {
        self.arg.to_string()
    }
}

impl Parse for NodeProcArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mutability = if input.peek(Token![mut]){
            input.parse::<Token![mut]>()?;
            ProcArgMutability::Write
        } else{
            ProcArgMutability::Read
        };

        let ident = input.parse::<Ident>()?;
        Ok(Self{arg: ident, mutability })
    }
}


#[derive(Debug, Clone)]
pub(crate) struct Proc{
    pub func:       Box<Ident>,
    pub args:       Vec<NodeProcArg>,
}

impl Proc{
    fn get_args_names(&self) -> impl Iterator<Item = NodeIndex> + '_ {
        let selected_uses_exprs =
            self.args
            .iter()
            .map(|arg_expr| arg_expr.arg.to_string())        ;
        selected_uses_exprs
    }
}



impl Parse for Proc{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        println!("proc input is {:#?}", &input);
        let func  = Box::new(input.parse::<Ident>()?);

        let args_parenthesized;
        parenthesized!(args_parenthesized in input);

        let args_terminated = args_parenthesized.parse_terminated(NodeProcArg::parse, Token![,])?;

        let mut args: Vec<NodeProcArg> = args_terminated.into_iter().collect();


        Ok( Self { func, args })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ProcsInput {
    pub procs: Vec<Proc>
}

impl Parse for ProcsInput{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        println!("input is {:#?}", &input);

        let braced_input ;
        braced!(braced_input in input);

        println!("braced_input is {:#?}", &braced_input);

        let procs =  {
            let procs = braced_input.parse_terminated(
                Proc::parse,
                Token![,]
            )?;

            procs.into_iter().collect::<Vec<_>>()
        };
        Ok(Self{procs})
    }
}

#[derive(Debug, Clone)]
pub(crate) struct NodesInput{
    pub nodes_defs:    HashMap<String, NodeDefinition>
}

impl Parse for NodesInput{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let nodes_input;
        let _braces = braced!(nodes_input in input);

        let mut reses : HashMap<String, NodeDefinition> = HashMap::new();
        let parse_term =
        nodes_input
        .parse_terminated(
            NodeDefinition::parse,
            Token![,],
        )?
        .into_iter();

        for node_defn in parse_term{
            let node_name = (&node_defn).node_name.to_string();
            reses.insert(
                node_name,
                node_defn
            );
        }

        Ok(NodesInput{
            nodes_defs: reses
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct NodeDefinition{
    pub node_name:      Ident,
    pub node_type:      Type,
    pub node_value:     Expr,
}

impl Parse for NodeDefinition{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let node_name = input.parse::<Ident>()?;
        let _type_sep_token = input.parse::<Token![:]>()?;
        let node_type = input.parse::<Type>()?;
        let _val_sep_token  = input.parse::<Token![=]>()?;
        let node_value = input.parse::<Expr>()?;

        Ok(NodeDefinition{
            node_name,
            node_type,
            node_value,
        })
    }
}