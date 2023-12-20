use alloc::string::{String};
use core::{fmt::{Write}, marker::PhantomData};
use core::ops::{DerefMut, Not};

use heapless::Deque;
use crate::base::{Node, NodeRef, Process, TryDeref, TryDerefMut};
use crate::signals::activation_signal::ActivationSignalNRef;





pub type LogQueue<const QUEUE_SIZE: usize> = Deque<String, QUEUE_SIZE>;
pub type LogQueueNode<const MSG_QUEUE_SIZE: usize> = Node<Deque<String, MSG_QUEUE_SIZE>>;
pub type LogQueueNodeRef<'a, const MSG_QUEUE_SIZE: usize> = <LogQueueNode<MSG_QUEUE_SIZE> as TryDeref>::TRef<'a>;
pub type LogQueueNMut<'a, const MSG_QUEUE_SIZE: usize> = <LogQueueNode<MSG_QUEUE_SIZE> as TryDerefMut>::TMut<'a>;


pub struct LogWriter<TWrite: Write, const msg_queue_size: usize>{
	writer: 			TWrite,
	current_string:		Option<String>,
}

impl<TWrite: Write, const msg_queue_size: usize> Process for LogWriter<TWrite, msg_queue_size>
{
	type TArgs<'args>  = LogQueueNMut<'args, msg_queue_size>;
    fn resume<'a>(&mut self, _msg_queue: Self::TArgs<'a>) {
    }
}

impl<TWrite: Write, const msg_queue_size: usize> LogWriter<TWrite, msg_queue_size> {
	pub fn try_write_all(&mut self , _queue: impl DerefMut<Target=LogQueue<msg_queue_size>>){

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
			log_queue.push_back(new_message).unwrap();
		}
   	}
}