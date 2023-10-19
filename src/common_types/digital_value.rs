#[derive(PartialEq, Clone, Copy)]
pub enum DigitalValue {
	High, Low
}


impl From<bool> for DigitalValue{
    fn from(value: bool) -> Self {
        match value{
            true => DigitalValue::High,
            false => DigitalValue::Low,
        }
    }
} 

impl Into<bool> for DigitalValue{
    fn into(self) -> bool {
        match self{
            DigitalValue::High => true,
            DigitalValue::Low =>  false,
        }
    }
}

