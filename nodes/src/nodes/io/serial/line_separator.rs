use alloc::string::String;
use core::mem;
use core::ops::DerefMut;
use core::pin::Pin;
use heapless::Deque;
use crate::base::{NodeRef, Process};
use crate::queue::queue_node::{HistoryNode, QueueNMut, QueueNode};

pub struct LineSeparator< const output_buffer_size: usize, const input_buffer_size: usize> {
	string_buffer: String,
}
impl<const output_buffer_size: usize, const input_buffer_size: usize> Process for LineSeparator<output_buffer_size, input_buffer_size>
{
	type TArgs<'args> where Self: 'args = (QueueNMut<'args, char, input_buffer_size>, QueueNMut<'args, String, output_buffer_size>);

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
		mut char_input: impl DerefMut<Target=Deque<char, input_buffer_size>>,
	) -> ReadFinished {
		while let Some(new_char) = char_input.pop_front(){
			match new_char{
				'\n' | '\r' => return ReadFinished::LineFinished,
				_ 			=> self.string_buffer.push(new_char),
			}
		}
		return ReadFinished::InputDepleted;
	}

	pub fn read_many(
		&mut self,
		mut char_input: 	impl DerefMut<Target = Deque<char, input_buffer_size>>,
		mut lines_output: 	impl DerefMut<Target = Deque<String, output_buffer_size>>,
	) {
		while (!lines_output.is_full()) && (self.read(&mut *char_input) == ReadFinished::LineFinished) {
			self.string_buffer.shrink_to_fit();
			let new_string = mem::replace(&mut self.string_buffer , String::new());
			lines_output.push_back(new_string).unwrap();
		}
	}
}






