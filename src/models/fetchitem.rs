#[derive(Clone)]
pub enum FetchValueType {
    Link(String),
    Text,
    HoverText(String),
}

#[derive(Clone)]
pub struct FetchItem {
    pub name: String,
    pub value: String,
    pub value_type: FetchValueType,
}
