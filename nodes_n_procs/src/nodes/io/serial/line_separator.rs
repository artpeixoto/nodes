use alloc::string::String;
use core::mem;
use core::ops::DerefMut;
use heapless::{Deque, spsc::Queue};
use crate::{base::{proc::Process}, queue::QueueNMut};

pub struct LineSeparator<const OUTPUT_BUFFER_SIZE: usize, const INPUT_BUFFER_SIZE: usize> {
	string_buffer: String,
}

impl<'a, const OUTPUT_BUFFER_SIZE: usize, const INPUT_BUFFER_SIZE: usize> 
	Process<'a> for LineSeparator<OUTPUT_BUFFER_SIZE, INPUT_BUFFER_SIZE>
{
	type TArgs  = (QueueNMut<'a, char, INPUT_BUFFER_SIZE>, QueueNMut<'a, String, OUTPUT_BUFFER_SIZE>) ;

	fn resume(&mut self, (chars_input, lines_output): Self::TArgs) {
		self.read_many(chars_input, lines_output);
	}
}

#[derive(Clone, PartialEq)]
pub enum ReadFinished {
	LineFinished,
	InputDepleted,
}

impl<const OUTPUT_BUFFER_SIZE: usize, const INPUT_BUFFER_SIZE: usize>
	LineSeparator< OUTPUT_BUFFER_SIZE, INPUT_BUFFER_SIZE>
{
	pub fn new() -> Self {
		Self { string_buffer: String::new() }
	}
	
	pub fn read(
		&mut self,
		mut char_input: impl DerefMut<Target=Queue<char, INPUT_BUFFER_SIZE>>,
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
		mut char_input: 	impl DerefMut<Target = Queue<char, INPUT_BUFFER_SIZE>>,
		mut lines_output: 	impl DerefMut<Target = Queue<String, OUTPUT_BUFFER_SIZE>>,
	) {
		while (!lines_output.is_full()) && (self.read(&mut *char_input) == ReadFinished::LineFinished) {
			self.string_buffer.shrink_to_fit();
			let new_string = mem::replace(&mut self.string_buffer , String::new());
			lines_output.enqueue(new_string).unwrap();
		}
	}
}






