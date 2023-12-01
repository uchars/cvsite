#[derive(Clone)]
pub enum FetchValueType {
    Link,
    Text,
}

#[derive(Clone)]
pub struct FetchItem {
    pub name: String,
    pub value: String,
    pub value_type: FetchValueType,
}
