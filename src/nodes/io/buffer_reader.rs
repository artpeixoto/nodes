use core::error::Error;
use alloc::boxed::Box;
use embedded_hal::serial::Read;
use heapless::Vec;
use crate::nodes::base::{Node, NodeRef, TimedProcess};
use crate::common_types::timing;


pub type BufferNode<Word, const buffer_size: usize> 
	= Node<Vec<Word, buffer_size>>;
pub type BufferNodeRef<'node, Word, const buffer_size: usize> 
	= NodeRef<'node, Vec<Word, buffer_size>>;

pub struct 
	ReaderProc<'node, Word, Reader, const buffer_size: usize>
	where Reader: Read<Word>
{
	output: BufferNodeRef<'node, Word, buffer_size>,
	reader: Reader,
}

impl
	<'a, Word, Reader, const buffer_size: usize>
	ReaderProc<'a, Word, Reader, buffer_size> 
	where 
		Reader			: Read<Word>,
		Reader::Error 	: Error + 'static
{
	pub fn new() -> Self{
		todo!()
	}
	pub fn read(&mut self) -> Result<usize, Box<dyn Error>> {
 		let mut output = self.output.try_borrow_mut()?; 
		let mut read_count: usize = 0;

		loop {
			match self.reader.read() {
				Ok(word) => {
					output.push(word);
					read_count += 1;
				} 
				Err(nb::Error::Other(e)) => {
					let boxed_err = Box::new(e) as Box<dyn Error>;
					break Err(boxed_err);
				}
				Err(nb::Error::WouldBlock) 	=> {
					break Ok(read_count);
				}
			}
		}
	}
}

impl <'a, Word, Reader, const buffer_size: usize>
	TimedProcess 
	for ReaderProc <'a, Word, Reader, buffer_size> 
	where 
		Reader: Read<Word>,
		Reader::Error: Error + 'static
{
    fn next(&mut self, current_time: &timing::Time) -> Result<(), Box<dyn Error>> {
		self.read()?;
		Ok(())
    }
}

 
