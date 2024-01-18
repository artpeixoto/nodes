
use crate::sampling::sample_node::{SampleNMut};
use crate::signals::activation_signal::ActivationSignalNRef;
use crate::base::node::*;
use crate::base::proc::*;

pub struct Sampler<T: Clone>{
    current_sample: Option<T>,
}

impl<T:Clone> Sampler<T>{}


impl<'a, T:Clone > Process<'a> for Sampler<T> where T: 'a{
    type TArgs = (ActivationSignalNRef<'a>, NodeRef<'a, T>, SampleNMut<'a, T>) ;
    fn resume(&mut self, (activation_signal, value_source, mut sample_output): Self::TArgs){
        if activation_signal.is_some(){
            let value = value_source.deref().clone() ;
            *sample_output = Some(value);
        } else if sample_output.is_some() {
            *sample_output = None;
        }
    }
}

fn test<TProc>(proc: &mut TProc) 
    where TProc: for<'a> Process<'a, TArgs =NodeRefMut<'a,usize>>,  
{

}