use crate::data_description::DataDescription;
use crate::Visualize;

impl<T> Visualize for &T where T: Visualize {}

impl Visualize for u8 {
    fn data(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl Visualize for usize {
    fn data(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl Visualize for String {
    fn data(&self) -> Option<String> {
        Some(self.clone())
    }
}

impl<T> Visualize for Option<T>
where
    T: Visualize,
{
    fn data(&self) -> Option<String> {
        Some(
            match self {
                Some(_) => "Some",
                None => "None",
            }
            .into(),
        )
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
