use super::column::Column;
use super::option::Opts;
use super::result::ResultRow;

#[derive(Clone, Debug, Default)]
pub struct Table {
    pub columns: Vec<Column>,
    pub opts: Opts,
    pub output: prettytable::Table,
    pub results: Vec<ResultRow>,
}

pub fn new(results: Vec<ResultRow>, opts: Opts) -> Table {
    let mut output = prettytable::Table::new();
    output.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    let mut t = Table {
        opts,
        results,
        output,

        ..Default::default()
    };
    t.set_columns();
    t
}

impl Table {
    pub fn results(&mut self) {
        for r in &self.results {
            let mut vals = Vec::<prettytable::Cell>::new();
            for c in &self.columns {
                vals.push(r.cell(c));
            }
            self.output.add_row(prettytable::Row::new(vals));
        }
    }

    pub fn display(&mut self) {
        println!();
        self.results();
        self.output.printstd();
    }

    fn set_columns(&mut self) {
        if self.opts.only_keys {
            self.columns = vec![Column::Key]
        } else if self.opts.decrypted {
            if self.opts.with_status {
                self.columns = vec![
                    Column::Name,
                    Column::Kind,
                    Column::Category,
                    Column::Password,
                    Column::Score,
                    Column::Status,
                    Column::Count,
                    Column::Url,
                ];
            } else {
                self.columns = vec![
                    Column::Name,
                    Column::Kind,
                    Column::Category,
                    Column::Password,
                    Column::Score,
                    Column::Count,
                    Column::Url,
                ];
            }
        } else if self.opts.password_history {
            self.columns = vec![
                Column::Password,
                Column::Created,
                Column::LastUpdated,
                Column::LastAccessed,
            ];
        } else if self.opts.kinds {
            self.columns = vec![Column::Kind];
        } else if self.opts.tags {
            self.columns = vec![Column::Tags];
        } else if self.opts.categories {
            self.columns = vec![Column::Category];
        } else if self.opts.with_status {
            self.columns = vec![
                Column::Name,
                Column::Kind,
                Column::Category,
                Column::Status,
                Column::Count,
                Column::Url,
            ];
        } else {
            self.columns = vec![
                Column::Name,
                Column::Kind,
                Column::Category,
                Column::Count,
                Column::Url,
            ];
        }
        self.set_headers();
    }

    fn set_headers(&mut self) {
        self.output.set_titles(prettytable::Row::new(
            self.columns
                .clone()
                .into_iter()
                .map(|c| c.header())
                .collect(),
        ));
    }
}
