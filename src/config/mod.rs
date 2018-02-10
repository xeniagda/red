use std::sync::Mutex;

pub struct Config {
    pub silent: bool
}

lazy_static! {
    pub static ref CONF: Mutex<Config> = Mutex::new(Config { silent: false });
}
