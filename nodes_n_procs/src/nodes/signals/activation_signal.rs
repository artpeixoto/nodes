use crate::base::node::{Node, TryDeref, TryDerefMut};

pub type ActivationSignal = Option<()>;

pub type ActivationSignalNode       = Node<ActivationSignal>;
pub type ActivationSignalNRef<'a>   = <Node<ActivationSignal> as TryDeref>::TRef<'a>;
pub type ActivationSignalNMut<'a>   = <Node<ActivationSignal> as TryDerefMut>::TMut<'a>;
