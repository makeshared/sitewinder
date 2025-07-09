use chrono::NaiveDate;

#[derive(Clone, Debug, Default)]
pub struct Metadata {
    pub title: String,
    pub group: Option<String>,
    pub tags: Option<Vec<String>>,
    pub date: Option<NaiveDate>,
    pub author: String,
    pub path: String,
}

impl Metadata {
    pub fn new(path: &str) -> Metadata {
        Metadata {
            title: String::new(),
            group: None,
            tags: None,
            date: None,
            author: String::new(),
            path: path.to_string(),
        }
    }
}
