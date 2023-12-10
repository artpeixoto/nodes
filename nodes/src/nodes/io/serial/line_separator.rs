use alloc::string::String;
use core::mem;
use core::ops::DerefMut;
use embedded_hal::serial::Read;
use heapless::Deque;
use crate::nodes::base::{NodeRef, SimpleProcess};

pub struct LineSeparator<'a, const output_buffer_size: usize, const input_buffer_size: usize> {
	char_input: 	NodeRef<'a, Deque<char, input_buffer_size >>,
	lines_output: 	NodeRef<'a, Deque<String, output_buffer_size>>,
	current_string: String,
}

impl<'a, const output_buffer_size: usize, const input_buffer_size: usize> SimpleProcess
	for LineSeparator<'a, output_buffer_size, input_buffer_size>
{
	fn next(&mut self) -> Result<(), Self::TError> {
		let mut char_input = self.char_input.try_borrow_mut()?;
		let mut lines_output= self.lines_output.try_borrow_mut()?;
		Self::read_many(
			char_input.deref_mut(),
			lines_output.deref_mut(),
			&mut self.current_string
		);
		Ok(())
	}
}

#[derive(Clone, PartialEq)]
pub enum ReadFinished {
	LineFinished,
	InputDepleted,
}

impl<'a, const output_buffer_size: usize, const input_buffer_size: usize>
	LineSeparator<'a, output_buffer_size, input_buffer_size>
{
	pub fn read(char_input: &mut Deque<char, input_buffer_size>, current_string: &mut String) -> ReadFinished {
		while let Some(new_char) = char_input.pop_back(){
			match new_char{
				'\n' | '\r' => return ReadFinished::LineFinished,
				_ 			=> current_string.push(new_char),
			}
		}
		return ReadFinished::InputDepleted;
	}

	pub fn read_many(
		char_input: 	&mut Deque<char, input_buffer_size>,
		lines_output: 	&mut Deque<String, output_buffer_size>,
		string_buffer: 	&mut String,
	) {
		while (!lines_output.is_full()) && (Self::read(char_input, string_buffer) == ReadFinished::LineFinished) {
			string_buffer.shrink_to_fit();
			let new_string = mem::replace(string_buffer , String::new());
			lines_output.push_front(new_string).unwrap();
		}
	}
}






