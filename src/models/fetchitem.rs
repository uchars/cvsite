pub enum FetchValueType {
    Link,
    Text,
}

pub struct FetchItem {
    pub name: String,
    pub value: String,
    pub value_type: FetchValueType,
}
