use std::{fs, io};
use std::fs::File;
use std::io::Write;
use std::path::{Path};
use crate::configuration::Configuration;
use std::time;
use crate::pattern_recognition::Shape;
use std::path::MAIN_SEPARATOR;

pub fn start_backup(config: Configuration) -> Result<(), io::Error> {
    let start = time::Instant::now();

    // Be sure that the destination path exists before creating the log file
    let log_dir = Path::new(&config.destination_path);
    fs::create_dir_all(&log_dir)?;
    let log_file_path = log_dir.join("log.txt");
    let mut log_file = File::create(&log_file_path)?;

    let total_size = copy_files_with_extension(config)?;
    let elapsed = start.elapsed();
    // Write the total size and the elapsed time in a log file in the destination path of configuration
    let log = format!("Total copied file size: {} bytes\nElapsed time: {:?}", total_size, elapsed);
    log_file.write_all(log.as_bytes())?;
    Ok(())
}

/// Copy the files from the source path to the destination path, filtering by extension if needed.
/// Return the total dimension of the copied files.
/// # Arguments
/// * `config`: configuration parameters: shape, source path, destination path, optional extension filter
/// If the extension filter is None, all files are copied.
/// returns: Result<usize, Error>
pub fn copy_files_with_extension(config: Configuration) -> Result<u64, io::Error> {
    let src_path = Path::new(&config.source_path);
    let dest_path = Path::new(&config.destination_path);
    let ext = config.extension_filter.as_ref();
    let mut total_size = 0;

    if !src_path.exists() {
        return Err(io::Error::new(std::io::ErrorKind::NotFound, "Source path does not exist"));
    }

    if !dest_path.exists() {
        fs::create_dir_all(&dest_path)?;
    }

    for entry in fs::read_dir(&src_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let new_dest_path = dest_path.join(entry.file_name());
            let new_config = Configuration::new(config.shape.clone(), path.to_str().unwrap().to_string(), new_dest_path.to_str().unwrap().to_string(), config.extension_filter.clone());
            // copy_files_with_extension(new_config)?;
            total_size += copy_files_with_extension(new_config)?;
        } else if path.is_file() {
            if let Some(file_name) = path.file_name() {
                if let Some(file_name) = file_name.to_str() {
                    match ext {
                        Some(ext) => {
                            if file_name.ends_with(ext) {
                                let dest_file = dest_path.join(file_name);
                                fs::copy(&path, &dest_file)?;
                                total_size += fs::metadata(&path)?.len();
                            }
                        }
                        None => {
                            let dest_file = dest_path.join(file_name);
                            fs::copy(&path, &dest_file)?;
                            total_size += fs::metadata(&path)?.len();
                        }
                    }
                }
            }
        }
    }

    Ok(total_size)
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use serial_test::serial;
    use crate::pattern_recognition::Shape;
    use super::*;

    /// Creates a dummy directory with files for testing purposes.
    ///
    /// This function creates a dummy directory with a set of files. It creates two text files and two PDF files in the main directory.
    /// It also creates a subdirectory with a single text file. The function then returns the paths to the source and destination directories.
    ///
    /// # Returns
    ///
    /// * `(String, String)` - A tuple containing the paths to the source and destination directories.
    fn create_dummy_directory_with_files() -> (String, String) {
        // use the main separator to create the path for every OS
        let src = format!(".{}TEST COPY SOURCE FILE", MAIN_SEPARATOR);
        let dest = format!(".{}TEST DESTINATION FILE", MAIN_SEPARATOR);

        fs::create_dir_all(src.clone()).unwrap();
        //create two dummy txt files
        let file_path = Path::new(&src).join("dummy.txt");
        let mut file = File::create(file_path).unwrap();
        file.write_all(b"Hello, world!").unwrap();
        let file_path = Path::new(&src).join("dummy2.txt");
        let mut file = File::create(file_path).unwrap();
        file.write_all(b"Hello, world!!!!").unwrap();
        //create two dummy pdf files
        let file_path = Path::new(&src).join("dummy.pdf");
        let file = File::create(file_path).unwrap();
        let file_path = Path::new(&src).join("dummy2.pdf");
        let file = File::create(file_path).unwrap();
        //create one dummy subdirectory
        let subdir = Path::new(&src).join("subdir");
        fs::create_dir_all(subdir.clone()).unwrap();
        //create one dummy txt file in the subdirectory
        let file_path = subdir.join("dummy_subdir.txt");
        let mut file = File::create(file_path).unwrap();
        file.write_all(b"Hello, sub directory!").unwrap();
        // wait for completion
        sleep(time::Duration::from_millis(100));
        (src.to_string(), dest.to_string())
    }

    fn cleanup_dummy_directory(src: &str, dest: &str) {
        fs::remove_dir_all(src).unwrap();
        fs::remove_dir_all(dest).unwrap();
        //wait for completion
        sleep(time::Duration::from_millis(100));
    }

    #[test]
    #[serial]
    fn test_copy_files_with_extension() {
        let (src, dest) = create_dummy_directory_with_files();
        let config = Configuration::new(Shape::Circle, src.to_string(), dest.to_string(), Some("txt".to_string()));
        let result = copy_files_with_extension(config);
        println!("{:?}", result);
        assert!(result.is_ok());
        cleanup_dummy_directory(&src, &dest);
    }

    #[test]
    #[serial]
    fn test_copy_every_file() {
        let (src, dest) = create_dummy_directory_with_files();
        let config = Configuration::new(Shape::Circle, src.to_string(), dest.to_string(), None);
        let result = copy_files_with_extension(config);
        println!("{:?}", result);
        assert!(result.is_ok());
        cleanup_dummy_directory(&src, &dest);
    }

    #[test]
    #[serial]
    fn test_dimension() {
        let (src, dest) = create_dummy_directory_with_files();
        let ext = "txt";
        let config = Configuration::new(Shape::Circle, src.to_string(), dest.to_string(), Some(ext.to_string()));
        let result = copy_files_with_extension(config);
        // assert equal with 37 byte
        assert_eq!(result.unwrap(), 50);
        cleanup_dummy_directory(&src, &dest);
    }

    #[test]
    #[serial]
    fn test_log_file() {
        let (src, dest) = create_dummy_directory_with_files();
        let ext = "txt";
        let config = Configuration::new(Shape::Circle, src.to_string(), dest.to_string(), Some(ext.to_string()));
        let result = start_backup(config);
        //print result
        println!("{:?}", result);
        assert!(result.is_ok());
        cleanup_dummy_directory(&src, &dest);
    }
}
