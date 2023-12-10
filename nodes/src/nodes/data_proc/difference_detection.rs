

use crate::nodes::sampling::sample_history::SampleData;

pub struct DataChange<T>{
    pub previous_value: Option<SampleData<T>>,
    pub current_value: 	SampleData<T>,
}

pub struct DataChangeDetector<T: Clone + PartialEq >{
    last_value: Option<T>
}
