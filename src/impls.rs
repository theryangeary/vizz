use crate::address::Address;
use crate::data_description::DataDescription;
use crate::data_description::Value;
use crate::Visualize;

use std::ops::Deref;

macro_rules! impl_visualize_num {
    ($ty:ident) => {
        impl Visualize for $ty {
            fn data(&self) -> Option<Value> {
                Some(Value::Owned(self.to_string()))
            }
        }
    };
}

impl_visualize_num!(u8);
impl_visualize_num!(u16);
impl_visualize_num!(u32);
impl_visualize_num!(u64);
impl_visualize_num!(u128);
impl_visualize_num!(usize);
impl_visualize_num!(i8);
impl_visualize_num!(i16);
impl_visualize_num!(i32);
impl_visualize_num!(i64);
impl_visualize_num!(i128);
impl_visualize_num!(isize);

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
