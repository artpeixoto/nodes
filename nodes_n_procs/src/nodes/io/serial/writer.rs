use core::marker::PhantomData;
use embedded_io::{Write, WriteReady};
use crate::base::proc::Process;
use crate::queue::queue_node::QueueNMut;

pub struct WriterProc<const input_buffer_size: usize, TWriter: Write + WriteReady>
{
    writer:     TWriter,
}

impl< const input_buffer_size: usize, TWriter>
    Process for WriterProc< input_buffer_size,  TWriter>
    where
        TWriter: Write + WriteReady,
{
    type TArgs<'args> = QueueNMut<'args, u8, input_buffer_size>;
    fn resume<'args>(&mut self, mut inputs: Self::TArgs<'args>) {
        let _initial_inputs_size = inputs.len();
        while let (Some(input), true) = (inputs.peek(), self.writer.write_ready().unwrap()){
            match self.writer.write(&[input.clone()]).unwrap(){
                1   => unsafe { inputs.dequeue_unchecked(); }
                _   => {break;}
            }
        }
    }
}


impl< const input_buffer_size: usize, TWriter: Write + WriteReady>
    WriterProc< input_buffer_size,  TWriter>
{
    pub fn new(writer: TWriter) -> Self {
        Self{
            writer,
        }
    }
}
