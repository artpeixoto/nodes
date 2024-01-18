use core::borrow::Borrow;
use core::error::Error;
use core::iter::from_fn;
use core::marker::PhantomData;
use core::ops::DerefMut;

use embedded_io::{ReadReady, Read};
use heapless::{Deque, Vec};
use heapless::spsc::Queue;
use crate::base::proc::Process;
use crate::queue::queue_node::QueueNMut;

pub struct ReaderProc<Reader, const BUFFER_SIZE: usize>
	where Reader: ReadReady + Read
{
	reader: Reader,
	output_phantom: PhantomData<Queue<u8, BUFFER_SIZE>>,
}


impl<TReader, const BUFFER_SIZE: usize>
	ReaderProc<TReader, BUFFER_SIZE>
	where
		TReader: ReadReady + Read,
		TReader::Error 	: Error + 'static
{
	pub fn new( reader: TReader ) -> Self{
		Self{
			reader,
			output_phantom: PhantomData{}
		}
	}

	pub fn read (
		&mut self,
		mut output: impl DerefMut<Target=Queue<u8, BUFFER_SIZE>>,
	) -> usize {
		if self.reader.read_ready().unwrap() && !output.is_full(){
			let mut buf = Vec::<u8, BUFFER_SIZE>::new();
			let buf_ref = &mut buf[0..(output.capacity() - output.len())];

			let read_count = self.reader.read(buf_ref).unwrap();
			let output_producer = output.split().0;

			for byte in &buf_ref[0..read_count]{
				unsafe{ output.enqueue_unchecked(*byte); }
			}

			read_count
		} else {
			0
		}
	}
}

impl<'a, Reader, const buffer_size: usize>
	Process<'a> for ReaderProc<  Reader, buffer_size>
	where
		Reader			: ReadReady + Read,
		Reader::Error	: Error + 'static
{
	type TArgs  
		= QueueNMut<'a, u8, buffer_size>;

	fn resume(&mut self, output: Self::TArgs) 
	{
		self.read(output);
	}
}
