use core::borrow::Borrow;
use core::error::Error;
use core::iter::from_fn;
use core::marker::PhantomData;
use core::ops::DerefMut;
use core::str::from_utf8;

use heapless::{Deque, Vec};
use heapless::spsc::Queue;
use crate::base::proc::Process;
use crate::queue::QueueNMut;

pub struct ReaderProc<Reader, const BUFFER_SIZE: usize>
	where Reader: embedded_hal_nb::serial::Read<u8>,
{
	reader: Reader,
	output_phantom: PhantomData<Queue<char, BUFFER_SIZE>>,
}


impl<TReader, const BUFFER_SIZE: usize>
	ReaderProc<TReader, BUFFER_SIZE>
	where
		TReader: embedded_hal_nb::serial::Read<u8>,
{
	pub fn new( reader: TReader ) -> Self{
		Self{
			reader,
			output_phantom: PhantomData{}
		}
	}

	pub fn read (
		&mut self,
		mut output: impl DerefMut<Target=Queue<char, BUFFER_SIZE>>,
	) -> usize {
		if !output.is_full(){
			let open_output_space = output.capacity() - output.len() ;
			let mut read_count = 0;
			for i in 0..open_output_space{
				match self.reader.read(){
					Ok(word) => {
						output.enqueue(word as char).unwrap();
					},
					Err(nb::Error::WouldBlock) => {
						read_count = i;
						break;
					},
					_ => { panic!(); }
				}
			}
			read_count
		} else {
			0
		}
	}
}

impl<'a, Reader, const BUFFER_SIZE: usize>
	Process<'a> for ReaderProc<  Reader, BUFFER_SIZE>
	where
		Reader			: embedded_hal_nb::serial::Read,
{
	type TArgs  
		= QueueNMut<'a, char, BUFFER_SIZE>;

	fn resume(&mut self, output: Self::TArgs) 
	{
		self.read(output);
	}
}

pub struct ByteToCharConverter<const INPUT_QUEUE_SIZE: usize, const OUTPUT_BUFFER_SIZE: usize> {
	current_char: Option<heapless::Vec<u8, 8>>
}

impl<const INPUT_QUEUE_SIZE: usize, const OUTPUT_BUFFER_SIZE: usize> ByteToCharConverter<INPUT_QUEUE_SIZE, OUTPUT_BUFFER_SIZE> {
	pub fn new() -> Self{
		Self{current_char:None}
	}
	pub fn execute_convertion(
		&mut self,
		input_queue: 	&mut impl DerefMut<Target = heapless::spsc::Queue<u8, INPUT_QUEUE_SIZE>>, 
		output_queue: 	&mut impl DerefMut<Target = heapless::spsc::Queue<char, OUTPUT_BUFFER_SIZE>>
	) {
		if !input_queue.is_empty() && !output_queue.is_full(){
			let mut current_char = self.current_char.take().unwrap_or(heapless::Vec::new());
			while !input_queue.is_empty() && !output_queue.is_full(){
				if let Ok(current_str) = from_utf8(&current_char){
					let mut last_index_in_current_char = None;
					for (index, char) in current_str.char_indices(){
						if output_queue.enqueue(char).is_err(){
							last_index_in_current_char = Some(index);
							break;
						}
					}
					if let Some(last_index) = last_index_in_current_char{
						self.current_char = Some(heapless::Vec::from_slice(&current_char[last_index..]).unwrap());
						break;
					} else {
						current_char = heapless::Vec::new();
					};
				}	
				if let Some(byte) = input_queue.dequeue(){
					current_char.push(byte).unwrap();
				} else {
					if !current_char.is_empty(){
						self.current_char = Some(current_char);
					}
					break;	
				}
			}
		}
	}
}


impl<'a, const INPUT_BUFFER_SIZE: usize, const OUTPUT_BUFFER_SIZE: usize> 
	Process<'a> for ByteToCharConverter<INPUT_BUFFER_SIZE, OUTPUT_BUFFER_SIZE>
{
	type TArgs = (
		QueueNMut<'a, u8,   INPUT_BUFFER_SIZE>,
		QueueNMut<'a, char, OUTPUT_BUFFER_SIZE> ,
	);

	fn resume(&mut self, (mut input_queue, mut output_queue): Self::TArgs) {
		self.execute_convertion(&mut input_queue, &mut output_queue);
	}
} 


pub struct CharToByteConverter<const INPUT_BUFFER_SIZE: usize, const OUTPUT_BUFFER_SIZE: usize>
{
	current_char: Option<char>
}

impl<'a, const INPUT_BUFFER_SIZE: usize, const OUTPUT_BUFFER_SIZE: usize> Process<'a> for CharToByteConverter<INPUT_BUFFER_SIZE, OUTPUT_BUFFER_SIZE> {
	type TArgs = (
		QueueNMut<'a, char, INPUT_BUFFER_SIZE>,
		QueueNMut<'a, u8, OUTPUT_BUFFER_SIZE>
	);

	fn resume(&mut self, args: Self::TArgs) {
		todo!()
	}
}

impl<const INPUT_BUFFER_SIZE: usize, const OUTPUT_BUFFER_SIZE: usize> CharToByteConverter<INPUT_BUFFER_SIZE, OUTPUT_BUFFER_SIZE> {
	pub fn new() -> Self{
		Self{current_char: None}
	}
}
