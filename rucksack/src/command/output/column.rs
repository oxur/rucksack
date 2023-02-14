use std::fmt;

use prettytable::color::BLUE;
use prettytable::{Attr, Cell};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Column {
    Category,
    Count,
    Created,
    Id,
    Imported,
    Kind,
    LastAccessed,
    LastUpdated,
    Name,
    Password,
    Score,
    Status,
    Synced,
    Tags,
    Url,
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Column {
    pub fn name(&self) -> String {
        match self {
            Column::Count => "Access Count".to_string(),
            Column::Score => "Score / Strength".to_string(),
            Column::Url => "URL".to_string(),
            _ => format!("{self}"),
        }
    }

    pub fn header(&self) -> Cell {
        Cell::new(self.name().as_str())
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(BLUE))
    }
}

#[cfg(test)]
mod tests {
    use super::Column;

    #[test]
    fn column_display() {
        assert_eq!(format!("{}", Column::Count), "Count");
        assert_eq!(format!("{}", Column::Name), "Name");
        assert_eq!(format!("{}", Column::Score), "Score");
        assert_eq!(format!("{}", Column::Url), "Url");
    }

    #[test]
    fn column_name() {
        assert_eq!(Column::Count.name(), "Access Count");
        assert_eq!(Column::Name.name(), "Name");
        assert_eq!(Column::Score.name(), "Score / Strength");
        assert_eq!(Column::Url.name(), "URL");
    }
}
