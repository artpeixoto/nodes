

use crate::nodes::sampling::sample_history::SampleHistoryDataPoint;

pub struct DataChange<T>{
    pub previous_value: Option<SampleHistoryDataPoint<T>>,
    pub current_value: 	SampleHistoryDataPoint<T>,
}

pub struct DataChangeDetector<T: Clone + PartialEq >{
    last_value: Option<T>
}
