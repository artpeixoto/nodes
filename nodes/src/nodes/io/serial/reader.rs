use core::error::Error;
use core::iter::from_fn;
use core::ops::DerefMut;

use embedded_hal::serial::Read;
use heapless::Deque;

use crate::nodes::base::{Node, NodeRef, SimpleProcess};
use crate::nodes::base::process_errors::NodeBorrowError;

pub struct ReaderProc<'node, Word, Reader, const buffer_size: usize>
	where Reader: Read<Word>
{
	output: NodeRef<'node, Deque<Word, buffer_size>>,
	reader: Reader,
}


impl<'a, Word, TReader, const buffer_size: usize>
	ReaderProc<'a, Word, TReader, buffer_size>
	where
		TReader: Read<Word>,
		TReader::Error 	: Error + 'static
{
	pub fn new(output: &'a Node<Deque<Word, buffer_size>>, reader: TReader ) -> Self{
		Self{
			output: output.make_ref(),
			reader,
		}
	}
	fn read (
		reader: &mut TReader,
		output: &mut Deque<Word, buffer_size>,
	) -> usize {
		let input_iter =
			from_fn(|| reader.read().ok())
			.fuse()
			.take(output.capacity() - output.len());

		let mut read_count: usize = 0;
		for input in input_iter{
			read_count += 1;
			unsafe{ output.push_front_unchecked(input); }
		}
		read_count
	}
}

impl <'a, Word, Reader, const buffer_size: usize>
	SimpleProcess for ReaderProc<'a, Word, Reader, buffer_size>
	where
		Reader: 		Read<Word>,
		Reader::Error: 	Error + 'static
{
    fn next(&mut self) -> Result<(), NodeBorrowError> {
		let mut output = self.output.try_borrow_mut()?;

		Self::read(
			&mut self.reader,
			output.deref_mut(),
		);

		Ok(())
	}
}
