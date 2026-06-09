use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct AppConfig {
    // The delay between background API fetches
    pub fetch_interval_secs: u64,
    // The IP or hostname of your Fronius Symo
    pub fronius_url: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            // Default to 1 second as requested
            fetch_interval_secs: 1,
            fronius_url: "http://127.0.0.1/".to_string(),
        }
    }
}

pub fn load_config(path: &str) -> AppConfig {
    match fs::read_to_string(path) {
        Ok(content) => {
            match serde_ini::from_str::<AppConfig>(&content) {
                Ok(config) => {
                    println!("🚀 Application starting with config.ini settings:");
                    println!("   └─ Interval: {} second(s)", config.fetch_interval_secs);
                    println!("   └─ Fronius URL: {}", config.fronius_url);
                    config
                }
                Err(e) => {
                    eprintln!("⚠️ Failed to parse config.ini ({}), using system defaults.", e);
                    AppConfig::default()
                }
            }
        }
        Err(_) => {
            println!("ℹ️ config.ini not found at '{}', using system default settings.", path);
            AppConfig::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_config_missing_file_returns_default() {
        // Arrange: Use a path that definitely does not exist
        let non_existent_path = "this_file_should_not_exist_12345.ini";

        // Act
        let config = load_config(non_existent_path);

        // Assert
        assert_eq!(config, AppConfig::default());
        assert_eq!(config.fetch_interval_secs, 1);
        assert_eq!(config.fronius_url, "http://127.0.0.1/");
    }

    #[test]
    fn test_load_config_malformed_file_returns_default() {
        // Arrange: Create a temporary file with invalid INI syntax
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "[Invalid-Garbage-Data}}").unwrap();
        let path = temp_file.path().to_str().unwrap();

        // Act
        let config = load_config(path);

        // Assert
        assert_eq!(config, AppConfig::default());
    }

    #[test]
    fn test_load_config_valid_file_overrides_default() {
        // Arrange: Create a valid file with custom values
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            "fetch_interval_secs = 5\nfronius_url = http://192.168.1"
        )
            .unwrap();
        let path = temp_file.path().to_str().unwrap();

        // Act
        let config = load_config(path);

        // Assert
        assert_eq!(config.fetch_interval_secs, 5);
        assert_eq!(config.fronius_url, "http://192.168.1");
    }
}
