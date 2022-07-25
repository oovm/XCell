use serde::{__private::de::Content, de::MapAccess, Deserialize};

pub fn read_map_next_key_lowercase<'de, M>(dict: &mut M) -> Result<Option<String>, M::Error>
where
    M: MapAccess<'de>,
{
    let key = dict.next_key::<&str>()?;
    Ok(key.map(|s| s.trim().to_ascii_lowercase()))
}

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

#[allow(unused_variables)]
pub fn read_map_next_extra<'de, M>(dict: &mut M, this: &str, key: &str)
where
    M: MapAccess<'de>,
{
    match dict.next_value::<Content>() {
        #[cfg(debug_assertions)]
        Ok(o) => eprintln!("{}.{} = {:?}", this, key, o),
        #[cfg(debug_assertions)]
        Err(_) => eprintln!("{}.{}", this, key),
        #[cfg(not(debug_assertions))]
        _ => {}
    }
}
