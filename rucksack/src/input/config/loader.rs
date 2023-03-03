use confyg::Confygery;

use super::{init, schema};

pub fn load(config_file: String, log_level: String, name: String) -> schema::Config {
    let defaults = schema::defaults();
    match init::config(config_file.clone()) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
    match Confygery::new()
        .add_file(&config_file)
        .add_struct(&defaults)
        .build::<schema::Config>()
    {
        Ok(mut cfg) => {
            if !log_level.is_empty() {
                cfg.logging.level = log_level;
            }
            cfg.rucksack.cfg_file = config_file;
            cfg.rucksack.name = name;
            cfg
        }
        Err(e) => panic!("{}", e),
    }
}
