pub trait TryAsRef<T>{
	type Error;
	type Ref: AsRef<T>;
	fn try_as_ref (&self) -> Result<Self::Ref, Self::Error> ;
}

pub trait TryAsMut<T>
	where Self: TryAsRef<T>
{
	type MutError;
	type Mut: AsMut<T>;
	fn try_as_mut(&mut self) -> Result<Self::Mut, Self::MutError>;
}