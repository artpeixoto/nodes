use core::error::Error;
use core::iter::from_fn;
use core::marker::PhantomData;
use core::ops::DerefMut;

use embedded_hal::serial::Read;
use heapless::Deque;
use heapless::spsc::Queue;
use crate::base::Process;
use crate::queue::queue_node::QueueNMut;

pub struct ReaderProc<Word, Reader, const buffer_size: usize>
	where Reader: Read<Word>
{
	reader: Reader,
	output_phantom: PhantomData<Queue<Word, buffer_size>>,
}


impl< Word, TReader, const buffer_size: usize>
	ReaderProc< Word, TReader, buffer_size>
	where
		TReader: Read<Word>,
		TReader::Error 	: Error + 'static
{
	pub fn new( reader: TReader ) -> Self{
		Self{
			reader,
			output_phantom: PhantomData{}
		}
	}
	fn read (
		&mut self,
		mut output: impl DerefMut<Target=Queue<Word, buffer_size>>,
	) -> usize {
		let input_iter =
			from_fn(|| self.reader.read().ok())
			.fuse()
			.take(output.capacity() - output.len());

		let mut read_count: usize = 0;
		for input in input_iter{
			read_count += 1;
			unsafe{ output.enqueue_unchecked(input); }
		}
		read_count
	}
}

impl<Word, Reader, const buffer_size: usize>
	Process for ReaderProc< Word, Reader, buffer_size>
	where
	 	for<'a> Word	:'a,
		Reader			: Read<Word>,
		Reader::Error	: Error + 'static
{
	type TArgs<'args>  
		= QueueNMut<'args, Word, buffer_size>;

	fn resume<'args>(&mut self, output: Self::TArgs<'args>) 
	{
		self.read(
			output
		);
	}
}
