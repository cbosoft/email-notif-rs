use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::Read;

/// Configuration object. Stores information about the email account used to
/// send mail. This struct is deserialised from the configuration file in
/// your home directory at "$HOME/.email_notifier.json".
#[derive(Deserialize)]
pub struct Config {
    pub smtp_server: String,
    pub sender_email: String,
    pub password: String,
    pub recipient_email: String,
    pub port: u16,
}

impl Config {
    /// Returns "$HOME/.email_notifier.json"
    fn config_path() -> String {
        let home = env::var("HOME").unwrap();
        format!("{}/{}", home, ".email_notifier.json")
    }

    /// Load configuration from config file and return config struct.
    pub fn load() -> Self {
        let config_path = Self::config_path();
        let mut f = File::open(config_path.clone())
            .expect(format!("error opening config file at {}", config_path).as_str());
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
