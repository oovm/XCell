#[serde(untagged)]
pub enum OneOrManyOrNull<T: Default> {
    One(T),
    Many(Vec<T>),
}

#[serde(untagged)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

impl OneOrManyOrNull {
    pub fn unwrap(self) {}
}

impl OneOrMany {
    pub fn unwrap(self) {}
}
