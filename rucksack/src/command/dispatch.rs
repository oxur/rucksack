use anyhow::Result;
use clap::ArgMatches;

use crate::app::App;

use super::handlers::{add, backup, config, dedupe, delete, export, gen, import, list, set, show};

pub fn run(app: &App, matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("add", add_matches)) => add::new(add_matches, app),
        Some(("backup", backup_matches)) => match backup_matches.subcommand() {
            Some(("delete", delete_matches)) => backup::delete(delete_matches, app),
            Some(("restore", restore_matches)) => backup::restore(restore_matches, app),
            Some((&_, _)) => todo!(),
            None => backup::run(backup_matches, app),
        },
        Some(("backups", backup_matches)) => match backup_matches.subcommand() {
            Some(("list", list_matches)) => backup::list(list_matches, app),
            Some((&_, _)) => todo!(),
            None => todo!(),
        },
        Some(("config", config_matches)) => match config_matches.subcommand() {
            Some(("re-init", init_matches)) => config::re_init(init_matches, app),
            Some((&_, _)) => todo!(),
            None => todo!(),
        },
        Some(("dedupe", dedupe_matches)) => dedupe::new(dedupe_matches, app),
        Some(("delete", delete_matches)) => delete::one(delete_matches, app),
        Some(("export", export_matches)) => export::new(export_matches, app),
        Some(("gen", gen_matches)) => gen::new(gen_matches),
        Some(("import", import_matches)) => import::new(import_matches, app),
        Some(("list", list_matches)) => match list_matches.subcommand() {
            Some(("backups", backups_matches)) => list::backups(backups_matches, app),
            Some(("deleted", deleted_matches)) => list::deleted(deleted_matches, app),
            Some(("duplicates", dupe_matches)) => list::duplicates(dupe_matches, app),
            Some(("keys", key_matches)) => list::keys(key_matches, app),
            Some(("passwords", passwords_matches)) => list::passwords(passwords_matches, app),
            Some((&_, _)) => todo!(),
            None => list::all(list_matches, app),
        },
        Some(("set", set_matches)) => match set_matches.subcommand() {
            Some(("password", password_matches)) => set::password(password_matches, app),
            Some(("status", status_matches)) => set::status(status_matches, app),
            Some(("url", url_matches)) => set::url(url_matches, app),
            Some(("user", user_matches)) => set::user(user_matches, app),
            Some(("type", type_matches)) => set::record_type(type_matches, app),
            Some((&_, _)) => todo!(),
            None => todo!(),
        },
        Some(("show", show_matches)) => match show_matches.subcommand() {
            Some(("backup-dir", bud_matches)) => show::backup_dir(bud_matches, app),
            Some(("categories", cat_matches)) => show::categories(cat_matches, app),
            Some(("config-file", cfgfile_matches)) => show::config_file(cfgfile_matches, app),
            Some(("config", cfg_matches)) => show::config(cfg_matches, app),
            Some(("data-dir", datadir_matches)) => show::data_dir(datadir_matches, app),
            Some(("db-file", dbfile_matches)) => show::db_file(dbfile_matches, app),
            Some(("db-version", dbvsn_matches)) => show::db_version(dbvsn_matches, app),
            Some(("tags", tag_matches)) => show::tags(tag_matches, app),
            Some(("types", type_matches)) => show::types(type_matches, app),
            Some((&_, _)) => todo!(),
            None => todo!(),
        },
        Some((cmd, _)) => {
            log::warn!("unknown command: {}", cmd);
            todo!()
        }
        None => todo!(),
    }
}
