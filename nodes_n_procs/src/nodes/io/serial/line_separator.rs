use alloc::string::String;
use core::mem;
use core::ops::DerefMut;
use heapless::{Deque, spsc::Queue};
use crate::{base::{proc::Process}, queue::queue_node::QueueNMut};

pub struct LineSeparator<const OUTPUT_BUFFER_SIZE: usize, const INPUT_BUFFER_SIZE: usize> {
	string_buffer: String,
}

impl<const OUTPUT_BUFFER_SIZE: usize, const INPUT_BUFFER_SIZE: usize> 
	Process for LineSeparator<OUTPUT_BUFFER_SIZE, INPUT_BUFFER_SIZE>
{
	type TArgs<'args>  = (QueueNMut<'args, char, INPUT_BUFFER_SIZE>, QueueNMut<'args, String, OUTPUT_BUFFER_SIZE>) where Self: 'args;

	fn resume<'args>(&mut self, (chars_input, lines_output): Self::TArgs<'args>) {
		self.read_many(chars_input, lines_output);
	}
}

#[derive(Clone, PartialEq)]
pub enum ReadFinished {
	LineFinished,
	InputDepleted,
}

impl<const output_buffer_size: usize, const input_buffer_size: usize>
	LineSeparator< output_buffer_size, input_buffer_size>
{
	pub fn read(
		&mut self,
		mut char_input: impl DerefMut<Target=Queue<char, input_buffer_size>>,
	) -> ReadFinished {
		while let Some(new_char) = char_input.dequeue(){
			match new_char{
				'\n' | '\r' => return ReadFinished::LineFinished,
				_ 			=> self.string_buffer.push(new_char),
			}
		}
		return ReadFinished::InputDepleted;
	}

	pub fn read_many(
		&mut self,
		mut char_input: 	impl DerefMut<Target = Queue<char, input_buffer_size>>,
		mut lines_output: 	impl DerefMut<Target = Queue<String, output_buffer_size>>,
	) {
		while (!lines_output.is_full()) && (self.read(&mut *char_input) == ReadFinished::LineFinished) {
			self.string_buffer.shrink_to_fit();
			let new_string = mem::replace(&mut self.string_buffer , String::new());
			lines_output.enqueue(new_string).unwrap();
		}
	}
}






