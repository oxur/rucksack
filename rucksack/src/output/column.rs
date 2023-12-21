use std::fmt;

use prettytable::color::BLUE;
use prettytable::{Attr, Cell};

use super::Opts;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Column {
    Category,
    Count,
    Created,
    DupeInfo,
    Hash,
    HistoryCount,
    Id,
    Imported,
    Key,
    Kind,
    LastAccessed,
    LastUpdated,
    Name,
    Password,
    Permissions,
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
            Column::DupeInfo => "Duplicate Info".to_string(),
            Column::HistoryCount => "History Count".to_string(),
            Column::Kind => "Type".to_string(),
            Column::LastUpdated => "Last Updated".to_string(),
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

pub trait Columns {
    fn gen(&self, opts: &Opts) -> Vec<Column> {
        let mut cols = self.pre(opts);
        cols = self.passwd(opts, cols);
        cols = self.status(opts, cols);
        cols = self.post(opts, cols);
        cols.clone()
    }

    fn pre(&self, _opts: &Opts) -> Vec<Column> {
        vec![]
    }

    fn status(&self, opts: &Opts, mut cols: Vec<Column>) -> Vec<Column> {
        if opts.with_status {
            cols.push(Column::Status);
        }
        cols
    }

    fn passwd(&self, opts: &Opts, mut cols: Vec<Column>) -> Vec<Column> {
        if opts.with_passwd {
            cols.push(Column::Password);
            if opts.reveal && opts.decrypted {
                cols.push(Column::Score);
            }
        }
        cols
    }

    fn post(&self, _opts: &Opts, cols: Vec<Column>) -> Vec<Column> {
        cols
    }
}

pub struct ColsOnlyKey;

impl Columns for ColsOnlyKey {
    fn pre(&self, _opts: &Opts) -> Vec<Column> {
        vec![Column::Key]
    }
}

pub struct ColsOnlyKind;

impl Columns for ColsOnlyKind {
    fn pre(&self, _opts: &Opts) -> Vec<Column> {
        vec![Column::Kind]
    }
}

pub struct ColsOnlyTags;

impl Columns for ColsOnlyTags {
    fn pre(&self, _opts: &Opts) -> Vec<Column> {
        vec![Column::Tags]
    }
}

pub struct ColsOnlyCat;

impl Columns for ColsOnlyCat {
    fn pre(&self, _opts: &Opts) -> Vec<Column> {
        vec![Column::Category]
    }
}

pub struct ColsBackupFiles;

impl Columns for ColsBackupFiles {
    fn pre(&self, _opts: &Opts) -> Vec<Column> {
        vec![Column::Name, Column::Permissions]
    }
}

pub struct ColsGroupByName;

impl Columns for ColsGroupByName {
    fn post(&self, _opts: &Opts, mut cols: Vec<Column>) -> Vec<Column> {
        cols.append(&mut vec![Column::Count, Column::Url]);
        cols
    }
}

#[derive(Eq, PartialEq, PartialOrd)]
pub struct ColsGroupByHash;

impl Columns for ColsGroupByHash {
    fn pre(&self, _opts: &Opts) -> Vec<Column> {
        vec![Column::Kind, Column::Category, Column::DupeInfo]
    }

    fn post(&self, _opts: &Opts, mut cols: Vec<Column>) -> Vec<Column> {
        cols.append(&mut vec![
            Column::Count,
            Column::LastUpdated,
            Column::HistoryCount,
        ]);
        cols
    }
}

pub struct ColsGroupByPasswd;

impl Columns for ColsGroupByPasswd {
    fn pre(&self, _opts: &Opts) -> Vec<Column> {
        vec![Column::Name, Column::Kind, Column::Category]
    }

    fn post(&self, _opts: &Opts, mut cols: Vec<Column>) -> Vec<Column> {
        cols.append(&mut vec![Column::Count, Column::Url]);
        cols
    }
}

pub struct ColsGroupByKind;

impl Columns for ColsGroupByKind {
    fn pre(&self, _opts: &Opts) -> Vec<Column> {
        vec![Column::Name, Column::Category]
    }

    fn post(&self, _opts: &Opts, mut cols: Vec<Column>) -> Vec<Column> {
        cols.append(&mut vec![Column::Count, Column::Url]);
        cols
    }
}

pub struct ColsGroupByCat;

impl Columns for ColsGroupByCat {
    fn pre(&self, _opts: &Opts) -> Vec<Column> {
        vec![Column::Name, Column::Kind]
    }

    fn post(&self, _opts: &Opts, mut cols: Vec<Column>) -> Vec<Column> {
        cols.append(&mut vec![Column::Count, Column::Url]);
        cols
    }
}

pub struct ColsDefault;

impl Columns for ColsDefault {
    fn pre(&self, _opts: &Opts) -> Vec<Column> {
        vec![Column::Name, Column::Kind, Column::Category]
    }

    fn post(&self, _opts: &Opts, mut cols: Vec<Column>) -> Vec<Column> {
        cols.append(&mut vec![Column::Count, Column::Url]);
        cols
    }
}

pub struct ColsPasswdHist;

impl Columns for ColsPasswdHist {
    fn post(&self, _opts: &Opts, mut cols: Vec<Column>) -> Vec<Column> {
        cols.append(&mut vec![
            Column::Created,
            Column::LastUpdated,
            Column::LastAccessed,
        ]);
        cols
    }
}

#[cfg(test)]
mod tests {
    use super::Column;

    #[test]
    fn column_display() {
        assert_eq!(format!("{}", Column::Count), "Count");
        assert_eq!(format!("{}", Column::Kind), "Kind");
        assert_eq!(format!("{}", Column::Name), "Name");
        assert_eq!(format!("{}", Column::Score), "Score");
        assert_eq!(format!("{}", Column::Url), "Url");
    }

    #[test]
    fn column_name() {
        assert_eq!(Column::Count.name(), "Access Count");
        assert_eq!(Column::Kind.name(), "Type");
        assert_eq!(Column::Name.name(), "Name");
        assert_eq!(Column::Score.name(), "Score / Strength");
        assert_eq!(Column::Url.name(), "URL");
    }
}
