use core::marker::PhantomData;
use core::ops::DerefMut;
use alloc::boxed::Box;
use alloc::string::String;
use embedded_hal_nb::serial::{Read, Write};
use heapless::spsc::Queue;
use crate::base::proc::Process;
use crate::queue::QueueNMut;

pub struct WriterProc<const input_buffer_size: usize, TWriter: Write>
{
    writer:     TWriter,
}

impl<'a, const INPUT_BUFFER_SIZE: usize, TWriter>
    Process<'a> for WriterProc<INPUT_BUFFER_SIZE,  TWriter>
    where
        TWriter: Write
{
    type TArgs = QueueNMut<'a, u8, INPUT_BUFFER_SIZE>;
    fn resume(&mut self, mut inputs: Self::TArgs) {
        let _initial_inputs_size = inputs.len();

        loop{
            let Some(word) = inputs.peek() else {break};
            match self.writer.write(*word){
                Ok(()) => { inputs.dequeue().unwrap(); },
                Err(nb::Error::WouldBlock) => {break;} ,
                Err(nb::Error::Other(_)) => panic!(),
            }
        }
    }
}

pub struct LineJoiner<const INPUT_BUFFER_SIZE: usize, const OUTPUT_BUFFER_SIZE: usize>{
    current_line: Option<(Box<[u8]>, usize)>
}

impl<const INPUT_BUFFER_SIZE:usize, const OUTPUT_BUFFER_SIZE: usize> 
    LineJoiner<INPUT_BUFFER_SIZE, OUTPUT_BUFFER_SIZE> 
{
    pub fn new() -> Self{
        Self{
            current_line: None
        }
    }
}
impl<'a, const INPUT_BUFFER_SIZE:usize, const OUTPUT_BUFFER_SIZE: usize> 
    Process<'a> for LineJoiner<INPUT_BUFFER_SIZE, OUTPUT_BUFFER_SIZE> 
{
    type TArgs = (
        QueueNMut<'a, String, INPUT_BUFFER_SIZE>,
        QueueNMut<'a, u8    , OUTPUT_BUFFER_SIZE>,
    );

    fn resume(&mut self, (mut input_queue, mut output_queue): Self::TArgs) {
        if self.current_line.is_none(){
            let Some(next_line) = input_queue.dequeue() else {return};
            self.current_line = Some((next_line.into_bytes().into_boxed_slice(), 0));
        }
        if let Some((line, caret)) = &mut self.current_line {
            let remaining = &line[*caret..];
            let mut words_sent_count = None ;
            for (index, word) in remaining.iter().enumerate(){
                let enqueue_res = output_queue.enqueue(*word);
                if enqueue_res.is_err(){
                    words_sent_count = Some(index); 
                    break;
                }
            }
            match words_sent_count{
                None => /* foi tudo adicionado */ {
                    self.current_line = None;
                }
                Some(count) => { /* adicionou uma parcela. Avancamos o fea da puta aqui  */
                    *caret = *caret + count;
                }
            }
        }
    }
}

impl< const OUTPUT_BUFFER_SIZE: usize, TWriter: Write>
    WriterProc< OUTPUT_BUFFER_SIZE,  TWriter>
{
    pub fn new(writer: TWriter) -> Self {
        Self{
            writer,
        }
    }
}

pub struct ReaderWriterProc<TReadWrite, const INPUT_BUFFER_SIZE: usize, const OUTPUT_BUFFER_SIZE: usize> where TReadWrite: Read + Write{
    rw: TReadWrite
}

impl<TReadWrite, const INPUT_BUFFER_SIZE: usize, const OUTPUT_BUFFER_SIZE: usize>
    ReaderWriterProc<TReadWrite, INPUT_BUFFER_SIZE, OUTPUT_BUFFER_SIZE>
    where TReadWrite: Read + Write
{
    pub const fn new(rw: TReadWrite) -> Self{
        Self{rw}   
    }
    pub fn execute_read(&mut self, mut read_queue: impl DerefMut<Target=Queue<u8, INPUT_BUFFER_SIZE>>){
        let open_input_space = read_queue.capacity() - read_queue.len() ;

        for _ in 0..open_input_space{
            match self.rw.read(){
                Ok(word) => {
                    read_queue.enqueue(word).unwrap();
                },
                Err(nb::Error::WouldBlock) => {
                    break;
                },
                _ => { panic!(); }
            }
        }
    }

    pub fn execute_write(&mut self, mut output_queue: impl DerefMut<Target=Queue<u8, OUTPUT_BUFFER_SIZE>>){
        loop{
            let Some(word) = output_queue.peek() else { break };
            match self.rw.write(*word) {
                Ok(()) => { output_queue.dequeue().unwrap(); },
                Err(nb::Error::WouldBlock) => {break;} ,
                Err(nb::Error::Other(_)) => panic!(),
            }
        }
    }
}
impl<'a, TReadWrite, const INPUT_BUFFER_SIZE: usize, const OUTPUT_BUFFER_SIZE: usize> 
    Process<'a> for ReaderWriterProc<TReadWrite, INPUT_BUFFER_SIZE, OUTPUT_BUFFER_SIZE> 
    where TReadWrite: Read+Write+'a
{
    type TArgs = (
        /*read_queue*/  QueueNMut<'a, u8, INPUT_BUFFER_SIZE>,
        /*write_queue*/ QueueNMut<'a, u8, OUTPUT_BUFFER_SIZE>,
    );

    fn resume(&mut self, (read_queue, write_queue): Self::TArgs) {
        if !read_queue.is_full(){
            self.execute_read(read_queue)
        }
        if !write_queue.is_empty(){
            self.execute_write(write_queue);
        }
    }
}