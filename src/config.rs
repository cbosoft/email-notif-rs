use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize)]
pub struct Config {
    pub smtp_server: String,
    pub sender_email: String,
    pub password: String,
    pub recipient_email: String,
    pub port: u16,
}

impl Config {
    fn config_path() -> String {
        let home = env::var("HOME").unwrap();
        let config_path = env::join_paths([home, ".email_notifier.json".to_string()]).unwrap();
        config_path.into_string().unwrap()
    }
    pub fn load() -> Self {
        let config_path = Self::config_path();
        let mut f = File::open(config_path).unwrap();
        let mut data = String::new();
        f.read_to_string(&mut data).unwrap();
        serde_json::from_str(data.as_str()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let _ = Config::load();
    }
}
