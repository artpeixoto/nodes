use core::marker::PhantomData;
use core::ops::DerefMut;
use core::pin::Pin;

use embedded_hal::serial::Write;
use heapless::Deque;
use crate::base::Process;
use crate::queue::queue_node::QueueNMut;

use crate::base::{Node, NodeRef};
use crate::base::process_errors::NodeBorrowError;

pub struct WriterProc<const input_buffer_size: usize, TWord:Clone, TWriter: Write<TWord>>
{
    writer:     TWriter,
    _phantom:   PhantomData<TWord>,
}

impl< const input_buffer_size: usize, TWord, TWriter>
    Process for WriterProc< input_buffer_size, TWord, TWriter>
    where
        TWord: Clone,
        TWriter: Write<TWord>,
        for<'a> TWord: 'a,
{
    type TArgs<'args>
        = QueueNMut<'args, TWord, input_buffer_size>
        ;

    fn resume<'args>(&mut self, mut inputs: Self::TArgs<'args>) {
        let initial_inputs_size = inputs.len();
        while let Some(input) = inputs.front(){
            match self.writer.write(input.clone()){
                Ok(_)   => { inputs.pop_front(); }
                Err(_)  => { break; }
            }
        }
    }
}


impl< const input_buffer_size: usize, TWord: Clone, TWriter: Write<TWord>>
    WriterProc< input_buffer_size, TWord, TWriter>
{
    pub fn new(writer: TWriter) -> Self {
        Self{
            writer,
            _phantom: PhantomData{}
        }
    }
}
