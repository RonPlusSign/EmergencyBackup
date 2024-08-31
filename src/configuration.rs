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

    /// Save the configuration to a JSON file inside the "config" folder (next to the executable)
    pub fn save(&self) {
        let json = serde_json::to_string_pretty(&self);
        match json {
            Ok(json) => {
                let path = Configuration::get_path(self.shape);

                // Create the "config" folder if it does not exist
                let folder = path.parent().expect("Could not get the configuration folder");
                if !folder.exists() {
                    std::fs::create_dir(folder).expect("Could not create the configuration folder");
                }

                // Write the configuration to the file
                std::fs::write(path.as_path(), json).expect("Could not write the configuration file");
                println!("Configuration saved to {:?}", path);
            }
            Err(e) => { println!("Error: Could not serialize the configuration: {}", e); }
        }
    }

    /// Load the configuration from a JSON file inside the "config" folder (next to the executable) with the same name as the shape
    pub fn load(shape: Shape) -> Option<Configuration> {
        let path = Configuration::get_path(shape);

        // Check if the configuration file exists
        if !path.exists() {
            println!("Configuration file not found: {:?}", path);
            return None;
        }

        // Load the configuration from the file
        let json = std::fs::read_to_string(path).expect("Could not read the configuration file");
        let config: Configuration = serde_json::from_str(&json).expect("Could not parse the configuration file");
        Some(config)
    }

    /// Get the path of the configuration file for the given shape (config/{shape}.json, next to the executable)
    fn get_path(shape: Shape) -> std::path::PathBuf {
        let exe_path = std::env::current_exe().expect("Could not get the executable path");
        let mut path = exe_path.parent().expect("Could not get the executable folder").to_path_buf();
        path.push("config");
        path.push(format!("{}.json", shape.to_string()));
        path
    }
}

/// Returns the list of shapes for which exists a configuration file
pub fn shapes_with_config() -> Vec<Shape> {
    // Keep only the shapes that have a configuration file
    let shapes = vec![Shape::Circle, Shape::Square, Shape::Triangle];
    shapes.iter()
        .filter(|shape| Configuration::get_path(**shape).exists())  // Shapes with a config file
        .map(|shape| *shape)
        .collect()
}

/// Returns true if there is at least one shape configured
pub fn has_shapes_configured() -> bool { !shapes_with_config().is_empty() }

// TESTS
#[cfg(test)]
mod tests {
    use std::fs;
    use serial_test::serial;
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
    #[serial]
    fn test_configuration_save() {
        let config = Configuration::new(Shape::Circle, "source".to_string(), "destination".to_string(), Some("jpg".to_string()));
        config.save();
        let json = fs::read_to_string(Configuration::get_path(config.shape)).expect("Could not read the configuration file");
        let loaded_config: Configuration = serde_json::from_str(&json).expect("Could not parse the configuration file");
        assert_eq!(config, loaded_config);
        fs::remove_file(Configuration::get_path(config.shape)).expect("Unable to remove config file");  // Remove the test file
    }

    #[test]
    #[serial]
    fn test_configuration_load() {
        let config = Configuration::new(Shape::Circle, "source".to_string(), "destination".to_string(), Some("jpg".to_string()));
        config.save();
        let loaded_config = Configuration::load(config.shape).expect("Could not load the configuration");
        assert_eq!(config, loaded_config);
        fs::remove_file(Configuration::get_path(config.shape)).expect("Unable to remove config file");  // Remove the test file
    }
}