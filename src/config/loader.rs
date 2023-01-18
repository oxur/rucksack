use confyg::Confygery;

use super::{init, schema};

use crate::util;

pub fn load() -> schema::Config {
    let config_file = util::config_file();
    let defaults = schema::defaults();
    match init::config(config_file.clone()) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
    match Confygery::new()
        .add_file(&config_file)
        .add_struct(&defaults)
        .build()
    {
        Ok(cfg) => cfg,
        Err(e) => panic!("{}", e),
    }
}
