use confyg::Confygery;

use super::schema;

pub fn load() -> schema::Config {
    let cfg: schema::Config = Confygery::new().add_file("./config.toml").build().unwrap();
    cfg
}
