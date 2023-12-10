pub type QueueNode<T, const SIZE: usize> = Node<heapless::HistoryBuffer<T, SIZE>>;
pub type QueueNodeRef<'a, T, const SIZE: usize> = NodeRef<'a, heapless::HistoryBuffer<T, SIZE>>;
