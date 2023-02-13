use std::fmt;

use super::option::Opts;
use super::result::ListResult;

#[derive(Clone, Debug)]
pub enum Column {
    Category,
    Count,
    Created,
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
}

#[derive(Clone, Debug, Default)]
pub struct Formatter {
    pub columns: Vec<Column>,
    pub field_sep: String,
    pub header_sep: String,
    pub results: Vec<ListResult>,
    pub opts: Opts,
}

pub fn new(results: Vec<ListResult>, opts: Opts) -> Formatter {
    let mut f = Formatter {
        field_sep: " | ".to_string(),
        header_sep: "-+-".to_string(),
        opts,
        results,

        ..Default::default()
    };
    f.set_columns();
    f
}

impl Formatter {
    pub fn header(&self) {
        match self.columns[..] {
            // Decrypted with status
            [Column::Name, Column::Password, Column::Score, Column::Status, Column::Count, Column::Url] =>
            {
                let field_sep = &self.field_sep;
                println!(
                    "\n{: <30}{field_sep}{: <20}{field_sep}{: <15}{field_sep}{: <8}{field_sep}{: <12}{field_sep}{}",
                    Column::Name,
                    Column::Password,
                    Column::Score.name(),
                    Column::Status,
                    Column::Count.name(),
                    Column::Url.name(),
                );
                let header_sep = &self.header_sep;
                println!(
                    "{: <30}{header_sep}{: <20}{header_sep}{: <16}{header_sep}{: <12}{header_sep}{: <8}{header_sep}{}",
                    "-".repeat(30),
                    "-".repeat(20),
                    "-".repeat(16),
                    "-".repeat(12),
                    "-".repeat(8),
                    "-".repeat(40),
                )
            }
            // Decrypted w/o status
            [Column::Name, Column::Password, Column::Score, Column::Count, Column::Url] => {
                println!(
                    "\n{: <30} | {: <20} | {: <15} | {: <12} | {: <40}",
                    Column::Name,
                    Column::Password,
                    Column::Score.name(),
                    Column::Count.name(),
                    Column::Url.name(),
                );
                println!(
                    "{: <30}-+-{: <20}-+-{: <15}-+-{: <12}-+-{}",
                    "-".repeat(30),
                    "-".repeat(20),
                    "-".repeat(16),
                    "-".repeat(12),
                    "-".repeat(40),
                )
            }
            [_, ..] => todo!(),
            [] => todo!(),
        }
    }

    pub fn results(&self) {
        for r in &self.results {
            self.row(r)
        }
    }

    pub fn row(&self, r: &ListResult) {
        match self.columns[..] {
            // Decrypted with status
            [Column::Name, Column::Password, Column::Score, Column::Status, Column::Count, Column::Url] =>
            {
                println!(
                    "{: <30} | {: <20} | {: ^16.2} | {: ^12} | {: ^8} | {} ",
                    r.name, r.pwd, r.score, r.access_count, r.status, r.url,
                )
            }
            [Column::Name, Column::Password, Column::Score, Column::Count, Column::Url] => {
                println!(
                    "{: <30} | {: <20} | {: ^16.2} | {: ^12} | {}",
                    r.name, r.pwd, r.score, r.access_count, r.url,
                )
            }
            [_, ..] => todo!(),
            [] => todo!(),
        }
    }

    pub fn display(&self) {
        self.header();
        self.results();
    }

    fn set_columns(&mut self) {
        if self.opts.decrypted {
            if self.opts.with_status {
                self.columns = vec![
                    Column::Name,
                    Column::Password,
                    Column::Score,
                    Column::Status,
                    Column::Count,
                    Column::Url,
                ];
            } else {
                self.columns = vec![
                    Column::Name,
                    Column::Password,
                    Column::Score,
                    Column::Count,
                    Column::Url,
                ];
            }
        }
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
