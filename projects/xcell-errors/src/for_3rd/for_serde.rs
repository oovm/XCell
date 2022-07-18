use serde::de::MapAccess;
use serde::Deserialize;

#[allow(unused_variables)]
pub fn read_map_next_value<'de, M, F, T>(dict: &mut M, mut handler: F)
    where
        M: MapAccess<'de>,
        F: FnMut(T),
        T: Deserialize<'de>,
{
    match dict.next_value::<T>() {
        Ok(o) => handler(o),
        #[cfg(debug_assertions)]
        Err(e) => eprintln!("MapAccess::next_value: {}", e),
        #[cfg(not(debug_assertions))]
        _ => {}
    }
}
