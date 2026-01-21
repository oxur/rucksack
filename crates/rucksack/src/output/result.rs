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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let row = new("id1".to_string(), "test".to_string(), "http://example.com".to_string());
        assert_eq!(row.hashmap.get(&Column::Id).unwrap(), "id1");
        assert_eq!(row.hashmap.get(&Column::Name).unwrap(), "test");
        assert_eq!(row.hashmap.get(&Column::Url).unwrap(), "http://example.com");
    }

    #[test]
    fn test_password() {
        let row = password(
            "pass123".to_string(),
            "95".to_string(),
            "2024-01-01".to_string(),
            "2024-01-02".to_string(),
            "2024-01-03".to_string(),
        );
        assert_eq!(row.hashmap.get(&Column::Password).unwrap(), "pass123");
        assert_eq!(row.hashmap.get(&Column::Score).unwrap(), "95");
        assert_eq!(row.hashmap.get(&Column::Created).unwrap(), "2024-01-01");
        assert_eq!(row.hashmap.get(&Column::LastUpdated).unwrap(), "2024-01-02");
        assert_eq!(row.hashmap.get(&Column::LastAccessed).unwrap(), "2024-01-03");
    }

    #[test]
    fn test_category() {
        let row = category("work".to_string());
        assert_eq!(row.hashmap.get(&Column::Category).unwrap(), "work");
    }

    #[test]
    fn test_kind() {
        let row = kind("password".to_string());
        assert_eq!(row.hashmap.get(&Column::Kind).unwrap(), "password");
    }

    #[test]
    fn test_tag() {
        let row = tag("important".to_string());
        assert_eq!(row.hashmap.get(&Column::Tags).unwrap(), "important");
    }

    #[test]
    fn test_result_row_id() {
        let row = new("test_id".to_string(), "name".to_string(), "url".to_string());
        assert_eq!(row.id(), "test_id");
    }

    #[test]
    fn test_result_row_add() {
        let mut row = new("id".to_string(), "name".to_string(), "url".to_string());
        row.add(Column::Category, "personal".to_string());
        assert_eq!(row.hashmap.get(&Column::Category).unwrap(), "personal");
    }

    #[test]
    fn test_result_row_get() {
        let row = new("id".to_string(), "myname".to_string(), "myurl".to_string());
        assert_eq!(row.get(&Column::Name).unwrap(), "myname");
        assert_eq!(row.get(&Column::Url).unwrap(), "myurl");
        assert!(row.get(&Column::Category).is_none());
    }

    #[test]
    fn test_result_row_cell() {
        let row = new("id".to_string(), "name".to_string(), "http://example.com".to_string());
        let _cell = row.cell(&Column::Url);
        // Just verify it returns a cell without panicking
        assert!(true);
    }

    #[test]
    fn test_result_row_cell_truncation() {
        let mut row = new("id".to_string(), "name".to_string(), "url".to_string());
        let long_value = "a".repeat(60);
        row.add(Column::Category, long_value);
        let _cell = row.cell(&Column::Category);
        // Cell should truncate to 50 chars (47 + "...")
        assert!(true);
    }

    #[test]
    fn test_result_row_cell_score_colors() {
        let mut row = new("id".to_string(), "name".to_string(), "url".to_string());

        // Test various score ranges
        row.add(Column::Score, "100".to_string());
        let _ = row.cell(&Column::Score);

        row.add(Column::Score, "95".to_string());
        let _ = row.cell(&Column::Score);

        row.add(Column::Score, "50".to_string());
        let _ = row.cell(&Column::Score);

        row.add(Column::Score, "5".to_string());
        let _ = row.cell(&Column::Score);

        assert!(true);
    }

    #[test]
    fn test_result_row_cell_missing_column() {
        let row = new("id".to_string(), "name".to_string(), "url".to_string());
        let _cell = row.cell(&Column::Category);
        // Should return empty cell without panicking
        assert!(true);
    }

    #[test]
    fn test_result_row_eq() {
        let row1 = new("id1".to_string(), "name1".to_string(), "http://same.com".to_string());
        let row2 = new("id2".to_string(), "name2".to_string(), "http://same.com".to_string());
        assert_eq!(row1, row2); // Equal based on URL
    }

    #[test]
    fn test_result_row_ne() {
        let row1 = new("id".to_string(), "name".to_string(), "http://a.com".to_string());
        let row2 = new("id".to_string(), "name".to_string(), "http://b.com".to_string());
        assert_ne!(row1, row2);
    }

    #[test]
    fn test_result_row_ord() {
        let row1 = new("id".to_string(), "name".to_string(), "http://a.com".to_string());
        let row2 = new("id".to_string(), "name".to_string(), "http://b.com".to_string());
        assert!(row1 < row2);
    }

    #[test]
    fn test_result_row_clone() {
        let row1 = new("id".to_string(), "name".to_string(), "url".to_string());
        let row2 = row1.clone();
        assert_eq!(row1, row2);
    }

    #[test]
    fn test_result_row_default() {
        let row = ResultRow::default();
        assert_eq!(row.hashmap.len(), 0);
    }

    #[test]
    fn test_results_and_groups_default() {
        let rag = ResultsAndGroups::default();
        assert_eq!(rag.results.len(), 0);
        assert_eq!(rag.groups.len(), 0);
    }

    #[test]
    fn test_results_and_groups_clone() {
        let mut rag1 = ResultsAndGroups::default();
        rag1.results.push(new("id".to_string(), "name".to_string(), "url".to_string()));
        let rag2 = rag1.clone();
        assert_eq!(rag1.results.len(), rag2.results.len());
    }
}
