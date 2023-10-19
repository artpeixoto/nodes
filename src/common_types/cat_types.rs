
pub trait Functor{
    type T;
    type OtherFunctor<U>;

    fn f_map<U>(self, func: impl FnOnce(Self::T) -> U) -> Self::OtherFunctor<U>;
}

pub trait Monad{
    type T;
    type OtherMonad<U>: Monad<T=U>;

    fn m_return(value: Self::T) -> Self;
    fn m_bind<U>(self, func: impl FnOnce(Self::T) -> Self::OtherMonad<U>) -> Self::OtherMonad<U>;
}

impl<T> Functor for Option<T>{
    type T = T;

    type OtherFunctor<U> = Option<U>;

    fn f_map<U>(self, func: impl FnOnce(Self::T) -> U) -> Self::OtherFunctor<U> {
        match self{
            Some(val) => Some(func(val)),
            None => None,
        }
    }
}

impl<T> Monad for Option<T>{
    type T = T;
    type OtherMonad<U> = Option<U>;

    fn m_return(value: Self::T) -> Self {
        Some(value)
    }

    fn m_bind<U>(self, func: impl FnOnce(Self::T) -> Self::OtherMonad<U>) -> Self::OtherMonad<U> {
        match self{
            Some(value) => func(value),
            None => None,
        }
    }
}