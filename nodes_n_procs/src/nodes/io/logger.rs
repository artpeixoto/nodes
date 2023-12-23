use alloc::string::{String};
use core::{fmt::{Write}, marker::PhantomData};
use core::ops::{DerefMut, Not};
use heapless::{Deque, spsc::Queue};
use crate::base::node::*;
use crate::base::proc::*;
use crate::signals::activation_signal::ActivationSignalNRef;


pub type LogQueue<const QUEUE_SIZE: usize> = Queue<String, QUEUE_SIZE>;

pub type LogQueueNode<const MSG_QUEUE_SIZE: usize> = Node<LogQueue<MSG_QUEUE_SIZE>>;

pub type LogQueueNodeRef<'a, const MSG_QUEUE_SIZE: usize> = <LogQueueNode<MSG_QUEUE_SIZE> as TryDeref>::TRef<'a>;

pub type LogQueueNMut<'a, const MSG_QUEUE_SIZE: usize> = <LogQueueNode<MSG_QUEUE_SIZE> as TryDerefMut>::TMut<'a>;


pub struct LogWriter<TWrite: Write, const msg_queue_size: usize>{
	writer: 			TWrite,
	current_string:		Option<String>,
}


impl<TWrite: Write, const msg_queue_size: usize> Process for LogWriter<TWrite, msg_queue_size>
{
	type TArgs<'args>  = LogQueueNMut<'args, msg_queue_size>;
    fn resume<'a>(&mut self, _msg_queue: Self::TArgs<'a>) 
	{
		let can_take_string = {
			if self.current_string.is_some() {
				if unsafe{ self.write_string_unchecked() }{
					true
				} else {
					false 
				}
			} else { 
				false 
			}
		};
		if can_take_string{
			unsafe{ if self.get_string_unchecked(_msg_queue) {
				self.write_string_unchecked();
			} }
		}
    }
}


impl<TWrite: Write, const msg_queue_size: usize> LogWriter<TWrite, msg_queue_size> 
{
	pub fn new(writer: TWrite) -> Self{
		Self{
			writer			: writer, 
			current_string	: None
		}
	} 
	unsafe fn write_string_unchecked(&mut self) -> bool {
			
		let has_written = unsafe{ 
			let string = self.current_string.as_deref().unwrap_unchecked();
			self.writer.write_str(string).is_ok()
		};
		if has_written {
			self.current_string = None;
		}
		has_written
	}
	unsafe fn get_string_unchecked(
		&mut self, 
		mut queue: impl DerefMut<Target=LogQueue<msg_queue_size>>
	) -> bool {
		if !queue.is_empty(){
			self.current_string = Some(unsafe{queue.dequeue_unchecked()});
			true
		} else {
			false
		}
	}
}

pub struct Logger<TValue, TMsgMaker, const msg_queue_size: usize>
	where 
		TMsgMaker: FnMut(&TValue) -> String 
{
	msg_maker	 	: TMsgMaker,
	value_phantom	: PhantomData<TValue>,
}

impl< TValue, TMsgMaker, const msg_queue_size: usize> Logger<TValue, TMsgMaker, msg_queue_size>
	where TMsgMaker: FnMut(&TValue) -> String
{
	pub fn new(msg_maker: TMsgMaker) -> Self {
		Self { msg_maker , value_phantom: PhantomData{}}
	}
}

impl<TValue, TMsgMaker, const msg_queue_size: usize>
	Process for Logger< TValue, TMsgMaker, msg_queue_size>
	where 
	 	for<'a> TValue: 'a,
		TMsgMaker: FnMut(&TValue) -> String
{
	type TArgs<'args> = (
		NodeRef<'args, TValue>,
		ActivationSignalNRef<'args>,
		LogQueueNMut<'args, msg_queue_size>
	) ;

    fn resume<'a>(&mut self, (value, activation_signal, mut log_queue): Self::TArgs<'a> ) {
		if activation_signal.is_some() && log_queue.is_full().not(){
			let new_message = (self.msg_maker)(&value);
			log_queue.enqueue(new_message).unwrap();
		}
   	}
}