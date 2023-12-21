use core::marker::PhantomData;
use embedded_hal::serial::Write;
use crate::base::Process;
use crate::queue::queue_node::QueueNMut;

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
        let _initial_inputs_size = inputs.len();
        while let Some(input) = inputs.peek(){
            match self.writer.write(input.clone()){
                Ok(_)   => unsafe { inputs.dequeue_unchecked(); }
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
