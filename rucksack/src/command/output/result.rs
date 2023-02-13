use std::collections::HashMap;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct ListResult {
    pub id: String,
    pub name: String,
    pub url: String,
    pub pwd: String,
    pub access_count: u64,
    pub score: i64,
    pub status: String,
}

pub fn new(id: String, name: String, url: String) -> ListResult {
    ListResult {
        id,
        name,
        url,

        ..Default::default()
    }
}

impl ListResult {
    pub fn id(&self) -> String {
        self.id.clone()
    }
}

pub type GroupByString = HashMap<String, Vec<ListResult>>;
