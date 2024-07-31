use serde::{Deserialize, Serialize};
use crate::pattern_recognition::Shape;


/// Configuration struct for the Emergency Backup, JSON serializable.
/// The configuration stores the shape, source path, destination path, and optional extension filter.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Configuration {
    // Store the configuration parameters: shape, source path, destination path, optional extension filter
    pub shape: Shape,
    pub source_path: String,
    pub destination_path: String,
    pub extension_filter: Option<String>,
}

impl Configuration {
    pub fn new(shape: Shape, source_path: String, destination_path: String, extension_filter: Option<String>) -> Configuration {
        Configuration { shape, source_path, destination_path, extension_filter }
    }

    /// Save the configuration to a JSON file
    pub fn save(&self, path: &str) {
        let json = serde_json::to_string_pretty(&self);
        match json {
            Ok(json) => {
                std::fs::write(path, json).expect("Could not write the configuration file");
                println!("Configuration saved to {}", path);
            }
            Err(e) => { println!("Error: Could not serialize the configuration: {}", e); }
        }
    }

    /// Load the configuration from a JSON file
    pub fn load(path: &str) -> Configuration {
        let json = std::fs::read_to_string(path).expect("Could not read the configuration file");
        let config: Configuration = serde_json::from_str(&json).expect("Could not parse the configuration file");
        config
    }
}

// TESTS
#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_configuration_equality() {
        let config1 = Configuration::new(Shape::Circle, "source".to_string(), "destination".to_string(), Some("jpg".to_string()));
        let config2 = Configuration::new(Shape::Circle, "source".to_string(), "destination".to_string(), Some("jpg".to_string()));
        assert_eq!(config1, config2);
    }

    #[test]
    fn test_configuration_inequality() {
        let config1 = Configuration::new(Shape::Circle, "source".to_string(), "destination".to_string(), Some("jpg".to_string()));
        let config2 = Configuration::new(Shape::Square, "source".to_string(), "destination".to_string(), Some("jpg".to_string()));
        assert_ne!(config1, config2);
    }

    #[test]
    fn test_configuration_save() {
        let config = Configuration::new(Shape::Circle, "source".to_string(), "destination".to_string(), Some("jpg".to_string()));
        let path = "test_config.json";
        config.save(path);
        let json = fs::read_to_string(path).expect("Could not read the configuration file");
        let loaded_config: Configuration = serde_json::from_str(&json).expect("Could not parse the configuration file");
        assert_eq!(config, loaded_config);
        println!("Configuration saved to {}", path);
        fs::remove_file(path);  // Remove the test file
    }

    #[test]
    fn test_configuration_load() {
        let config = Configuration::new(Shape::Circle, "source".to_string(), "destination".to_string(), Some("jpg".to_string()));
        let path = "test_config.json";
        config.save(path);
        let loaded_config = Configuration::load(path);
        assert_eq!(config, loaded_config);
        println!("Configuration loaded from {}", path);
        fs::remove_file(path);  // Remove the test file
    }
}