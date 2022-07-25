use serde::{Serialize, Serializer};

use crate::OneOrMany;

impl<T> Serialize for OneOrMany<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.inner.len() {
            1 => unsafe { T::serialize(self.inner.first().unwrap_unchecked(), serializer) },
            _ => Vec::<T>::serialize(&self.inner, serializer),
        }
    }
}
