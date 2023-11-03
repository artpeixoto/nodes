use alloc::collections::VecDeque;


pub trait RollingDeque {
	type Element;
	fn push_roll_forward(&mut self, value: Self::Element) -> Option<Self::Element>;
	fn push_back_dropping(&mut self, value: Self::Element) -> Option<Self::Element>;
}


impl<T> RollingDeque for VecDeque<T> {

    type Element = T;

    fn push_roll_forward(&mut self, value: Self::Element) -> Option<Self::Element> {
		if self.capacity() == 0 { 
			panic!("Cant push taking on an empty VecDeque") 
		}

		let taken =
			if self.capacity() == self.len() {
				Some(self.pop_front().unwrap())	
			} else {
				None
			};

		self.push_back(value);

		taken
    }

    fn push_back_dropping(&mut self, value: Self::Element) -> Option<Self::Element> {
		if self.capacity() == 0 { 
			panic!("Cant push taking on an empty VecDeque") 
		}

		let taken =
			if self.capacity() == self.len() {
				Some( self.pop_back().unwrap() )	
			} else {
				None
			};

		self.push_front(value);

		taken
    }
}

