use std::fs;
use std::path::Path;
use crate::configuration::Configuration;

pub fn copy_files_with_extention(config: Configuration) -> Result<(), std::io::Error> {
    let src_path = Path::new(&config.source_path);
    let dest_path = Path::new(&config.destination_path);
    let ext = config.extension_filter.as_ref();

    if !src_path.exists() {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Source path does not exist"));
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
            copy_files_with_extention(new_config)?;
        } else if path.is_file() {
            if let Some(file_name) = path.file_name() {
                if let Some(file_name) = file_name.to_str() {
                    match ext {
                        Some(ext) => {
                            if file_name.ends_with(ext) {
                                let dest_file = dest_path.join(file_name);
                                fs::copy(&path, &dest_file)?;
                            }
                        },
                        None => {
                            let dest_file = dest_path.join(file_name);
                            fs::copy(&path, &dest_file)?;
                        },
                    }
                }
            }
        }
    }

    Ok(())
}
#[cfg(test)]
mod tests {
    use crate::pattern_recognition::Shape;
    use super::*;

    #[test]
    fn test_copy_files_with_extention() {
        let src = r"C:\Users\erika\Desktop\PoliTo\Programmazione di sistema\TEST COPIA SORGENTE FILE";
        let dest = r"C:\Users\erika\Desktop\PoliTo\Programmazione di sistema\TEST DESTINAZIONE FILE";
        let ext = "txt";
        let config = Configuration::new(Shape::Circle, src.to_string(), dest.to_string(), Some(ext.to_string()));
        let result = copy_files_with_extention(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_copy_every_file() {
        let src = r"C:\Users\erika\Desktop\PoliTo\Programmazione di sistema\TEST COPIA SORGENTE FILE";
        let dest = r"C:\Users\erika\Desktop\PoliTo\Programmazione di sistema\TEST DESTINAZIONE FILE NO EXTENSION";
        let config = Configuration::new(Shape::Circle, src.to_string(), dest.to_string(), None);
        let result = copy_files_with_extention(config);
        assert!(result.is_ok());
    }
}
