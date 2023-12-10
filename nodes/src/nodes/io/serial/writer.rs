use core::ops::DerefMut;

use embedded_hal::serial::Write;
use heapless::Deque;

use crate::nodes::base::{Node, NodeRef, SimpleProcess};
use crate::nodes::base::process_errors::NodeBorrowError;

pub struct WriterProc
    <'a, const input_buffer_size: usize, TWord:Clone, TWriter: Write<TWord>>
{
    writer:     TWriter,
    input:      NodeRef<'a, Deque<TWord, input_buffer_size>>,
}

impl<'a, const input_buffer_size: usize, TWord, TWriter>
    SimpleProcess for WriterProc<'a, input_buffer_size, TWord, TWriter>
    where
        TWord: Clone,
        TWriter: Write<TWord>
{
    fn next(&mut self) -> Result<(), Self::TError> {
        let mut input = self.input.try_borrow_mut()?;
        Self::write(input.deref_mut(), &mut self.writer);
        Ok(())
    }
}


impl<'a, const input_buffer_size: usize, TWord: Clone, TWriter: Write<TWord>>
    WriterProc<'a, input_buffer_size, TWord, TWriter>
{
    pub fn new(writer: TWriter, input: &'a Node<Deque<TWord, input_buffer_size>>) -> Self {
        Self{
            writer,
            input: input.make_ref(),
        }
    }
    pub fn try_write(&mut self) -> Result<(), NodeBorrowError>{
        let mut input = self.input.try_borrow_mut()?;
        Self::write(input.deref_mut(), &mut self.writer);
        Ok(())
    }

    fn write(
        inputs: &mut Deque<TWord, input_buffer_size>,
        writer: &mut TWriter
    ) -> usize {
        let initial_inputs_size = inputs.len();
        while let Some(input) = inputs.back(){
            match writer.write(input.clone()){
                Ok(_) =>  {
                    inputs.pop_back();
                }
                Err(_) => { break; }
            }
        }
        let final_inputs_size = inputs.len();
        initial_inputs_size - final_inputs_size
    }
}
