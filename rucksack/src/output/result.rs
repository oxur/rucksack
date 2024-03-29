use std::cmp::Ordering;
use std::collections::HashMap;

use prettytable::color::{BRIGHT_GREEN, BRIGHT_RED, BRIGHT_YELLOW, CYAN, GREEN, RED, YELLOW};
use prettytable::format::Alignment;
use prettytable::{Attr, Cell};

use super::column::Column;

#[derive(Clone, Debug, Default, Eq)]
pub struct ResultRow {
    pub hashmap: HashMap<Column, String>,
}

// This function is used for creating results rows that hold common
// record data and are needed by several rucksack commands.
pub fn new(id: String, name: String, url: String) -> ResultRow {
    let hashmap: HashMap<Column, String> =
        HashMap::from([(Column::Id, id), (Column::Name, name), (Column::Url, url)]);
    ResultRow { hashmap }
}

// This function is used for creating results rows that are needed by
// the `list passwords` command. The columns below are the only columns
// needed by that command.
pub fn password(
    pwd: String,
    score: String,
    created: String,
    updated: String,
    last_accessed: String,
) -> ResultRow {
    let hashmap: HashMap<Column, String> = HashMap::from([
        (Column::Password, pwd),
        (Column::Score, score),
        (Column::Created, created),
        (Column::LastUpdated, updated),
        (Column::LastAccessed, last_accessed),
    ]);
    ResultRow { hashmap }
}

// This function is used for creating results rows that are needed by
// the `show categories` command. The columns below are the only columns
// needed by that command.
pub fn category(cat: String) -> ResultRow {
    let hashmap: HashMap<Column, String> = HashMap::from([(Column::Category, cat)]);
    ResultRow { hashmap }
}

// This function is used for creating results rows that are needed by
// the `show types` command. The columns below are the only columns
// needed by that command.
pub fn kind(kind: String) -> ResultRow {
    let hashmap: HashMap<Column, String> = HashMap::from([(Column::Kind, kind)]);
    ResultRow { hashmap }
}

// This function is used for creating results rows that are needed by
// the `show tags` command. The columns below are the only columns
// needed by that command.
pub fn tag(tag: String) -> ResultRow {
    let hashmap: HashMap<Column, String> = HashMap::from([(Column::Tags, tag)]);
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
        let col = match self.get(column) {
            Some(c) => c.to_string(),
            None => {
                log::warn!("Key {} has no value for column {}", self.id(), column);
                "".to_string()
            }
        };
        let mut val = col;
        if val.len() > 50 {
            val.truncate(47);
            val.push_str("...");
        }
        let mut c = Cell::new(&val);
        match column {
            Column::Count => c.align(Alignment::RIGHT),
            Column::Score => {
                c.align(Alignment::RIGHT);
                if val.is_empty() {
                    val = "0".to_string();
                }
                match val.parse::<i32>().unwrap() {
                    100 => {
                        c = c
                            .with_style(Attr::ForegroundColor(BRIGHT_GREEN))
                            .with_style(Attr::Bold)
                    }
                    x if x >= 95 => c = c.with_style(Attr::ForegroundColor(BRIGHT_GREEN)),
                    x if x >= 90 => c = c.with_style(Attr::ForegroundColor(GREEN)),
                    x if x >= 85 => c = c.with_style(Attr::ForegroundColor(BRIGHT_YELLOW)),
                    x if x >= 80 => c = c.with_style(Attr::ForegroundColor(YELLOW)),
                    x if x >= 40 => c = c.with_style(Attr::ForegroundColor(RED)),
                    x if x >= 10 => {
                        c = c
                            .with_style(Attr::ForegroundColor(BRIGHT_RED))
                            .with_style(Attr::Bold)
                    }
                    _ => {
                        c = c
                            .with_style(Attr::ForegroundColor(BRIGHT_RED))
                            .with_style(Attr::Bold)
                            .with_style(Attr::Blink)
                    }
                }
            }
            Column::Url => c = c.with_style(Attr::ForegroundColor(CYAN)),
            _ => (),
        };
        c
    }
}

pub type GroupByString = HashMap<String, Vec<ResultRow>>;

#[derive(Clone, Debug, Default)]
pub struct ResultsAndGroups {
    pub results: Vec<ResultRow>,
    pub groups: GroupByString,
}
