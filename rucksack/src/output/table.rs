use super::column::{self, Column, Columns};
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
            self.columns = column::ColsOnlyKey {}.new(&self.opts);
        } else if self.opts.kinds {
            self.columns = column::ColsOnlyKind {}.new(&self.opts);
        } else if self.opts.tags {
            self.columns = column::ColsOnlyTags {}.new(&self.opts);
        } else if self.opts.categories {
            self.columns = column::ColsOnlyCat {}.new(&self.opts);
        } else if self.opts.backup_files {
            self.columns = column::ColsBackupFiles {}.new(&self.opts);
        } else if self.opts.group_by_name {
            self.opts.with_passwd = true;
            self.columns = column::ColsGroupByName {}.new(&self.opts);
        } else if self.opts.group_by_hash {
            self.columns = column::ColsGroupByHash {}.new(&self.opts);
        } else if self.opts.group_by_password {
            self.columns = column::ColsGroupByPasswd {}.new(&self.opts);
        } else if self.opts.group_by_kind {
            self.columns = column::ColsGroupByKind {}.new(&self.opts);
        } else if self.opts.group_by_category {
            self.columns = column::ColsGroupByCat {}.new(&self.opts);
        } else if self.opts.password_history {
            self.opts.with_passwd = true;
            self.columns = column::ColsPasswdHist {}.new(&self.opts);
        } else {
            self.columns = column::ColsDefault {}.new(&self.opts);
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
