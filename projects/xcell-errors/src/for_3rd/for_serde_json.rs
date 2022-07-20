use serde::de::MapAccess;
pub use serde_json::Value as Json;

#[allow(unused_variables)]
pub fn read_map_next_extra<'de, M>(dict: &mut M, this: &str, key: &str)
where
    M: MapAccess<'de>,
{
    match dict.next_value::<Json>() {
        #[cfg(debug_assertions)]
        Ok(o) => eprintln!("{}.{} = {:?}", this, key, o),
        #[cfg(debug_assertions)]
        Err(_) => eprintln!("{}.{}", this, key),
        #[cfg(not(debug_assertions))]
        _ => {}
    }
}
