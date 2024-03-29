
use heapless::spsc::Queue;
use crate::base::node::{Node, TryDeref, TryDerefMut};

pub type HistoryNode<T, const SIZE: usize> = Node<heapless::HistoryBuffer<T, SIZE>>;
pub type HistoryNRef<'a, T, const SIZE: usize> = <HistoryNode<T, SIZE> as TryDeref>::TRef<'a>;
pub type HistoryNMut<'a, T, const SIZE: usize> = <HistoryNode<T, SIZE> as TryDerefMut>::TMut<'a>;

pub type QueueNode<T, const SIZE: usize> = Node<Queue<T, SIZE>>;
pub type QueueNRef<'a, T, const SIZE: usize> = <QueueNode<T, SIZE> as TryDeref>::TRef<'a>;
pub type QueueNMut<'a, T, const SIZE: usize> = <QueueNode<T, SIZE> as TryDerefMut>::TMut<'a>;

pub type BytesQueueNode<const SIZE: usize> 		= QueueNode<u8, SIZE>;
pub type BytesQueueNRef<'a, const SIZE: usize> 	= QueueNRef<'a, u8, SIZE>;
pub type BytesQueueNMut<'a, const SIZE: usize> 	= QueueNMut<'a, u8, SIZE>;