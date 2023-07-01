use std::fs;

/// inspects the configured packages directory, looks for packages and returns them relative to the root
/// i.e. packages/client
pub fn get_packages(root_dir_path: &String, packages_dir: &String) -> Vec<String> {
    let mut package_dirs: Vec<String> = Vec::new();
    
    let mut package_path = String::from(root_dir_path.clone());
    package_path.push_str("/");
    package_path.push_str(&packages_dir);
    
    let dirs = match fs::read_dir(&package_path) {
        Ok(dirs) => dirs,
        Err(e) => panic!("Failed to read package dir: {} error: {}", package_path, e),
    };
    
    for entry in dirs {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue
        };
        
        let path = entry.path();
        
        let metadata = match fs::metadata(&path) {
            Ok(metadata) => metadata,
            Err(e) => panic!("Failed reading metadata from file: {}", e),
        };
        
        if metadata.is_dir() {
            let filename = match path.file_name() {
                Some(filename) => filename.to_str().unwrap(),
                None => panic!("Couldn't get filename"),
            };
            
            let mut package_path = packages_dir.clone();
            package_path.push_str("/");
            package_path.push_str(filename);
            
            package_dirs.push(package_path);
        }
    }
    
    return package_dirs;
}

/// takes known packages and files, returns distinct packages that contain the changed_files
pub fn filter_files_to_packages(known_packages: Vec<String>, changed_files: Vec<String>) -> Vec<String> {
    let mut changed_packages: Vec<String> = Vec::new();

    for change in changed_files {
        for package in &known_packages {
            if change.starts_with(package) {
                if !changed_packages.contains(&package) {
                    changed_packages.push(package.clone())
                }
            }
        } 
    }

    return changed_packages;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_packages_returns_package_dirs() {
        let result = get_packages(&String::from("/tmp/shivr/.testdata"), &String::from("packages"));
        assert_eq!(result.len(), 3);

        // can't guarantee order of return
        let mut found_client = false;
        let mut found_frontend = false;
        let mut found_backend = false;
        for package in result {
            if package == "packages/client" {
                found_client = true;
            } else if package == "packages/frontend" {
                found_frontend = true;
            } else if package == "packages/backend" {
                found_backend = true;
            }
        }

        assert_eq!(found_client, true);
        assert_eq!(found_frontend, true);
        assert_eq!(found_backend, true);
    }

    #[test]
    fn filter_files_to_packages_returns_packages() {
        let known_packages = vec!("packages/client".to_string(), "packages/frontend".to_string(), "packages/backend".to_string());
        let changed_files = vec!("garbage.txt".to_string(), "packages/frontend/run.sh".to_string(), "packages/backend/run.sh".to_string());

        let result = filter_files_to_packages(known_packages, changed_files);
        assert_eq!(result[0], "packages/frontend".to_string());
        assert_eq!(result[1], "packages/backend".to_string());
    }
}