use crate::address::Address;
use crate::data_description::DataDescription;
use crate::data_description::Value;
use crate::Visualize;

use std::ops::Deref;

impl Visualize for u8 {
    fn data(&self) -> Option<Value> {
        Some(Value::Owned(self.to_string()))
    }
}

impl Visualize for usize {
    fn data(&self) -> Option<Value> {
        Some(Value::Owned(self.to_string()))
    }
}

impl Visualize for String {
    fn data(&self) -> Option<Value> {
        Some(Value::Owned(self.clone()))
    }
}

impl Visualize for &String {
    fn data(&self) -> Option<Value> {
        Some(Value::referenced(
            Address::new(*self),
            DataDescription::from(*self),
        ))
    }
}

impl<T> Visualize for Option<T>
where
    T: Visualize,
{
    fn data(&self) -> Option<Value> {
        Some(Value::Owned(
            match self {
                Some(_) => "Some",
                None => "None",
            }
            .into(),
        ))
    }

    fn associated_data(&self) -> Option<Vec<DataDescription>> {
        self.as_ref().map(|x| vec![DataDescription::from(x)])
    }
}

impl<T> Visualize for Vec<T>
where
    T: Visualize,
{
    fn associated_data(&self) -> Option<Vec<DataDescription>> {
        Some(self.iter().map(DataDescription::from).collect())
    }
}

impl<T> Visualize for Box<T>
where
    T: Visualize,
{
    fn data(&self) -> Option<Value> {
        Some(Value::referenced(
            Address::new(self.deref()),
            DataDescription::from(self.deref()),
        ))
    }
}
