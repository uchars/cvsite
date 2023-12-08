#[derive(Clone)]
pub struct Project {
    pub name: String,
    pub color: String,
    pub url: String,
}

impl Project {
    pub fn new(name: &str, color: &str, url: &str) -> Self {
        Self {
            name: String::from(name),
            color: String::from(color),
            url: String::from(url),
        }
    }
}
