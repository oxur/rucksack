use std::cmp::Ordering;
use std::collections::HashMap;

use prettytable::color::{CYAN, GREEN, RED, YELLOW};
use prettytable::format::Alignment;
use prettytable::{Attr, Cell};

use super::column::Column;

#[derive(Clone, Debug, Default, Eq)]
pub struct ResultRow {
    pub hashmap: HashMap<Column, String>,
}

pub fn new(id: String, name: String, url: String) -> ResultRow {
    let hashmap: HashMap<Column, String> =
        HashMap::from([(Column::Id, id), (Column::Name, name), (Column::Url, url)]);
    // let mut hashmap: HashMap<Column, String> = HashMap::new();
    ResultRow { hashmap }
}

impl PartialOrd for ResultRow {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ResultRow {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hashmap
            .get(&Column::Url)
            .unwrap()
            .cmp(other.hashmap.get(&Column::Url).unwrap())
    }
}

impl PartialEq for ResultRow {
    fn eq(&self, other: &Self) -> bool {
        self.hashmap.get(&Column::Url).unwrap() == other.hashmap.get(&Column::Url).unwrap()
    }
}

impl ResultRow {
    pub fn id(&self) -> String {
        self.hashmap.get(&Column::Id).unwrap().to_string()
    }

    pub fn add(&mut self, column: Column, value: String) {
        self.hashmap.insert(column, value);
    }

    pub fn get(&self, column: &Column) -> Option<&String> {
        self.hashmap.get(column)
    }

    pub fn cell(&self, column: &Column) -> Cell {
        let mut val = self.get(column).unwrap().clone();
        if val.len() > 50 {
            val.truncate(47);
            val.push_str("...");
        }
        let mut c = Cell::new(&val);
        match column {
            Column::Count => c.align(Alignment::RIGHT),
            Column::Score => {
                c.align(Alignment::RIGHT);
                match val.parse::<i32>().unwrap() {
                    x if x >= 90 => c = c.with_style(Attr::ForegroundColor(GREEN)),
                    x if x >= 80 => c = c.with_style(Attr::ForegroundColor(YELLOW)),
                    _ => c = c.with_style(Attr::ForegroundColor(RED)),
                }
            }
            Column::Url => c = c.with_style(Attr::ForegroundColor(CYAN)),
            _ => (),
        };
        c
    }
}

pub type GroupByString = HashMap<String, Vec<ResultRow>>;
